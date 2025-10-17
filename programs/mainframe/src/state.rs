use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::constants::*;
use crate::errors::MainframeError;

#[account]
pub struct AgentAccount {
    /// The NFT mint associated with this agent
    pub nft_mint: Pubkey,
    /// The owner of the NFT and agent
    pub owner: Pubkey,
    /// The collection this NFT belongs to
    pub collection_mint: Option<Pubkey>,
    /// URI pointing to secure JSON metadata
    pub metadata_uri: String,
    /// Agent operational status
    pub status: AgentStatus,
    /// Timestamp of activation
    pub activated_at: i64,
    /// Last update timestamp
    pub updated_at: i64,
    /// Version for configuration updates
    pub version: u64,
    /// Seller who receives commission (if any)
    pub seller: Option<Pubkey>,
    /// Reserved space for future upgrades
    pub reserved: [u8; 32],
}

impl AgentAccount {
    pub const LEN: usize = 8 + // discriminator
        32 + // nft_mint
        32 + // owner
        1 + 32 + // collection_mint (Option<Pubkey>)
        4 + MAX_METADATA_URI_LENGTH + // metadata_uri (String)
        1 + // status (enum)
        8 + // activated_at
        8 + // updated_at
        8 + // version
        1 + 32 + // seller (Option<Pubkey>)
        32; // reserved
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AgentStatus {
    Active,
    Paused,
    Closed,
}

#[account]
pub struct ProtocolConfig {
    /// Protocol authority
    pub authority: Pubkey,
    /// Fee structure
    pub fees: FeeStructure,
    /// Fee distribution accounts
    pub protocol_treasury: Pubkey,
    pub validator_treasury: Pubkey,
    pub network_treasury: Pubkey,
    /// Fee distribution in basis points
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
    /// Reserved space for future upgrades
    pub reserved: [u8; 54],
}

impl ProtocolConfig {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        8 * 6 + // fees (FeeStructure)
        32 * 3 + // treasury addresses
        2 * 3 + // treasury bps
        1 + // paused
        8 + // total_agents
        8 + // total_partners (counter only, partners stored in separate PDAs)
        8 + // max_partner_collections
        2 + // max_affiliate_bps
        54; // reserved

    /// Calculate fee based on operation and collection
    /// Partner discount is checked via PDA lookup in processor
    pub fn calculate_base_fee(&self, operation: &str) -> u64 {
        match operation {
            "create_agent" => self.fees.create_agent,
            "update_config" => self.fees.update_config,
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

        // Validate basis points are correctly configured
        let total_bps = self.protocol_treasury_bps + self.validator_treasury_bps + self.network_treasury_bps;
        require!(total_bps == 10_000, MainframeError::InvalidTreasuryDistribution);
        
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
                    }
                ),
                protocol_fee_final
            )?;
        }

        if validator_fee > 0 {
            transfer(
                CpiContext::new(
                    system_program.to_account_info(),
                    Transfer {
                        from: payer.to_account_info(),
                        to: validator_treasury.to_account_info(),
                    }
                ),
                validator_fee
            )?;
        }

        if network_fee > 0 {
            transfer(
                CpiContext::new(
                    system_program.to_account_info(),
                    Transfer {
                        from: payer.to_account_info(),
                        to: network_treasury.to_account_info(),
                    }
                ),
                network_fee
            )?;
        }

        msg!("Fee distributed: {} lamports (protocol={}, validator={}, network={})", 
             fee_amount, protocol_fee_final, validator_fee, network_fee);
        
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
        require!(seller_affiliate_bps <= self.max_affiliate_bps, MainframeError::InvalidAffiliate);

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
                    }
                ),
                seller_fee
            )?;
        }

        // Validate treasury distribution
        let total_bps = self.protocol_treasury_bps + self.validator_treasury_bps + self.network_treasury_bps;
        require!(total_bps == 10_000, MainframeError::InvalidTreasuryDistribution);
        
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
                    }
                ),
                protocol_fee_final
            )?;
        }

        if validator_fee > 0 {
            transfer(
                CpiContext::new(
                    system_program.to_account_info(),
                    Transfer {
                        from: payer.to_account_info(),
                        to: validator_treasury.to_account_info(),
                    }
                ),
                validator_fee
            )?;
        }

        if network_fee > 0 {
            transfer(
                CpiContext::new(
                    system_program.to_account_info(),
                    Transfer {
                        from: payer.to_account_info(),
                        to: network_treasury.to_account_info(),
                    }
                ),
                network_fee
            )?;
        }

        msg!("Fee distributed: total={}, seller={}, protocol={}, validator={}, network={}", 
             total_fee, seller_fee, protocol_fee_final, validator_fee, network_fee);
        
        Ok(seller_fee)
    }
}

#[account]
pub struct PartnerCollectionAccount {
    pub collection_mint: Pubkey,
    pub discount_percent: u8,
    pub name: String,
    pub active: bool,
    pub added_at: i64,
    pub bump: u8,
}

impl PartnerCollectionAccount {
    pub const LEN: usize = 8 + // discriminator
        32 + // collection_mint
        1 + // discount_percent
        4 + MAX_PARTNER_NAME_LENGTH + // name
        1 + // active
        8 + // added_at
        1; // bump
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FeeStructure {
    pub create_agent: u64,
    pub update_config: u64,
    pub transfer_agent: u64,
    pub pause_agent: u64,
    pub close_agent: u64,
    pub execute_action: u64,
}
