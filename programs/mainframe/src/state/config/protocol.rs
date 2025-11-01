use super::fees::FeeStructure;
use crate::errors::MainframeError;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[account]
pub struct ProtocolConfig {
    /// Protocol authority
    pub authority: Pubkey,
    /// Manager authority (can manage flash bonuses and operational tasks)
    pub manager: Pubkey,
    /// Genesis collection mint (zero fees for this collection)
    pub genesis_collection_mint: Pubkey,
    /// Fee structure
    pub fees: FeeStructure,
    /// Fee distribution accounts
    pub protocol_treasury: Pubkey,
    pub validator_treasury: Pubkey,
    pub network_treasury: Pubkey,
    /// Fee distribution in basis points (must sum to 10000)
    pub protocol_treasury_bps: u16,
    pub validator_treasury_bps: u16,
    pub network_treasury_bps: u16,
    /// Emergency pause status
    pub paused: bool,
    /// Total agents activated
    pub total_agents: u64,
    /// Total partner collections (stored in separate PDAs)
    pub total_partners: u64,
    /// Maximum number of partner collections allowed
    pub max_partner_collections: u64,
    /// Maximum affiliate commission in basis points (default: 5000 = 50%)
    pub max_affiliate_bps: u16,
    /// Pending authority for two-step transfer
    pub pending_authority: Option<Pubkey>,
    /// Reserved space for future upgrades
    pub reserved: [u8; 20],
}

impl ProtocolConfig {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        32 + // manager
        32 + // genesis_collection_mint
        8 * 6 + // fees (FeeStructure)
        32 * 3 + // treasury addresses
        2 * 3 + // treasury bps
        1 + // paused
        8 + // total_agents
        8 + // total_partners (counter only, partners stored in separate PDAs)
        8 + // max_partner_collections
        2 + // max_affiliate_bps
        1 + 32 + // pending_authority (Option<Pubkey>)
        20; // reserved

    /// Calculate fee based on operation and collection
    /// Partner discount is checked via PDA lookup in processor
    pub fn calculate_base_fee(&self, operation: &str) -> u64 {
        match operation {
            "create_agent" => self.fees.create_agent,
            "update_agent_config" => self.fees.update_agent_config,
            "transfer_agent" => self.fees.transfer_agent,
            "pause_agent" => self.fees.pause_agent,
            "close_agent" => self.fees.close_agent,
            "execute_action" => self.fees.execute_action,
            _ => 0,
        }
    }

    /// Apply discount to base fee
    pub fn apply_discount(base_fee: u64, discount_percent: u8) -> u64 {
        if discount_percent >= 100 {
            return 0;
        }
        let discount_multiplier = 100 - discount_percent as u64;
        base_fee * discount_multiplier / 100
    }

    /// Distribute fees according to basis points with security checks
    pub fn distribute_fee<'info>(
        &self,
        fee_amount: u64,
        payer: &AccountInfo<'info>,
        protocol_treasury: &AccountInfo<'info>,
        validator_treasury: &AccountInfo<'info>,
        network_treasury: &AccountInfo<'info>,
        system_program: &AccountInfo<'info>,
    ) -> Result<()> {
        if fee_amount == 0 {
            return Ok(());
        }

        // Security: Validate payer has sufficient balance
        let payer_balance = payer.lamports();
        require!(
            payer_balance >= fee_amount,
            MainframeError::InsufficientBalance
        );

        // Validate basis points are correctly configured with checked arithmetic
        let total_bps = self
            .protocol_treasury_bps
            .checked_add(self.validator_treasury_bps)
            .and_then(|x| x.checked_add(self.network_treasury_bps))
            .ok_or(MainframeError::InvalidTreasuryDistribution)?;
        require!(
            total_bps == 10_000,
            MainframeError::InvalidTreasuryDistribution
        );

        // Calculate distribution using basis points
        let protocol_fee = fee_amount * self.protocol_treasury_bps as u64 / 10_000;
        let validator_fee = fee_amount * self.validator_treasury_bps as u64 / 10_000;
        let network_fee = fee_amount * self.network_treasury_bps as u64 / 10_000;

        // Handle rounding by giving remainder to protocol treasury
        let distributed_total = protocol_fee + validator_fee + network_fee;
        let protocol_fee_final = protocol_fee + (fee_amount - distributed_total);

        // Transfer fees to treasuries using system program
        if protocol_fee_final > 0 {
            transfer(
                CpiContext::new(
                    system_program.to_account_info(),
                    Transfer {
                        from: payer.to_account_info(),
                        to: protocol_treasury.to_account_info(),
                    },
                ),
                protocol_fee_final,
            )?;
        }

        if validator_fee > 0 {
            transfer(
                CpiContext::new(
                    system_program.to_account_info(),
                    Transfer {
                        from: payer.to_account_info(),
                        to: validator_treasury.to_account_info(),
                    },
                ),
                validator_fee,
            )?;
        }

        if network_fee > 0 {
            transfer(
                CpiContext::new(
                    system_program.to_account_info(),
                    Transfer {
                        from: payer.to_account_info(),
                        to: network_treasury.to_account_info(),
                    },
                ),
                network_fee,
            )?;
        }

        msg!(
            "Fee distributed: {} lamports (protocol={}, validator={}, network={})",
            fee_amount,
            protocol_fee_final,
            validator_fee,
            network_fee
        );

        Ok(())
    }

    /// Distribute fees with seller affiliate
    pub fn distribute_fee_with_affiliate<'info>(
        &self,
        total_fee: u64,
        seller_affiliate_bps: u16,
        payer: &AccountInfo<'info>,
        seller: Option<&AccountInfo<'info>>,
        protocol_treasury: &AccountInfo<'info>,
        validator_treasury: &AccountInfo<'info>,
        network_treasury: &AccountInfo<'info>,
        system_program: &AccountInfo<'info>,
    ) -> Result<u64> {
        if total_fee == 0 {
            return Ok(0);
        }

        // Validate affiliate doesn't exceed configured maximum
        require!(
            seller_affiliate_bps <= self.max_affiliate_bps,
            MainframeError::InvalidAffiliate
        );

        // Security: Validate payer has sufficient balance
        let payer_balance = payer.lamports();
        require!(
            payer_balance >= total_fee,
            MainframeError::InsufficientBalance
        );

        let (seller_fee, remaining_fee) = if seller_affiliate_bps > 0 && seller.is_some() {
            let affiliate = total_fee * seller_affiliate_bps as u64 / 10_000;
            let remaining = total_fee - affiliate;
            (affiliate, remaining)
        } else {
            (0, total_fee)
        };

        // Transfer affiliate fee to seller using system program
        // NOTE: Supports zero-balance accounts - seller account is validated by being passed in transaction
        if seller_fee > 0 && seller.is_some() {
            transfer(
                CpiContext::new(
                    system_program.to_account_info(),
                    Transfer {
                        from: payer.to_account_info(),
                        to: seller.unwrap().to_account_info(),
                    },
                ),
                seller_fee,
            )?;
        }

        // Validate treasury distribution with checked arithmetic
        let total_bps = self
            .protocol_treasury_bps
            .checked_add(self.validator_treasury_bps)
            .and_then(|x| x.checked_add(self.network_treasury_bps))
            .ok_or(MainframeError::InvalidTreasuryDistribution)?;
        require!(
            total_bps == 10_000,
            MainframeError::InvalidTreasuryDistribution
        );

        // Calculate treasury distributions
        let protocol_fee = remaining_fee * self.protocol_treasury_bps as u64 / 10_000;
        let validator_fee = remaining_fee * self.validator_treasury_bps as u64 / 10_000;
        let network_fee = remaining_fee * self.network_treasury_bps as u64 / 10_000;
        let distributed_total = protocol_fee + validator_fee + network_fee;
        let protocol_fee_final = protocol_fee + (remaining_fee - distributed_total);

        // Transfer fees to treasuries using system program
        if protocol_fee_final > 0 {
            transfer(
                CpiContext::new(
                    system_program.to_account_info(),
                    Transfer {
                        from: payer.to_account_info(),
                        to: protocol_treasury.to_account_info(),
                    },
                ),
                protocol_fee_final,
            )?;
        }

        if validator_fee > 0 {
            transfer(
                CpiContext::new(
                    system_program.to_account_info(),
                    Transfer {
                        from: payer.to_account_info(),
                        to: validator_treasury.to_account_info(),
                    },
                ),
                validator_fee,
            )?;
        }

        if network_fee > 0 {
            transfer(
                CpiContext::new(
                    system_program.to_account_info(),
                    Transfer {
                        from: payer.to_account_info(),
                        to: network_treasury.to_account_info(),
                    },
                ),
                network_fee,
            )?;
        }

        msg!(
            "Fee distributed: total={}, seller={}, protocol={}, validator={}, network={}",
            total_fee,
            seller_fee,
            protocol_fee_final,
            validator_fee,
            network_fee
        );

        Ok(seller_fee)
    }
}
