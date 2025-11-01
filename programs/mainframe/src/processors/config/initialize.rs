use crate::errors::MainframeError;
use crate::instructions::InitializeConfig;
use crate::state::FeeStructure;
use anchor_lang::prelude::*;

/// Initialize protocol configuration
pub fn initialize_config(
    ctx: Context<InitializeConfig>,
    fees: FeeStructure,
    genesis_collection_mint: Pubkey,
    protocol_treasury: Pubkey,
    validator_treasury: Pubkey,
    network_treasury: Pubkey,
    protocol_treasury_bps: u16,
    validator_treasury_bps: u16,
    network_treasury_bps: u16,
    max_partner_collections: u64,
    max_affiliate_bps: u16,
    manager: Pubkey,
) -> Result<()> {
    // Validate that basis points sum to 100% (10,000 bps) with checked arithmetic
    let total_bps = protocol_treasury_bps
        .checked_add(validator_treasury_bps)
        .and_then(|x| x.checked_add(network_treasury_bps))
        .ok_or(MainframeError::InvalidTreasuryDistribution)?;
    require!(
        total_bps == 10_000,
        MainframeError::InvalidTreasuryDistribution
    );

    // Validate affiliate bps doesn't exceed 100%
    require!(
        max_affiliate_bps <= 10_000,
        MainframeError::InvalidAffiliate
    );

    let config = &mut ctx.accounts.protocol_config;
    config.authority = ctx.accounts.authority.key();
    config.manager = manager;
    config.genesis_collection_mint = genesis_collection_mint;
    config.fees = fees;
    config.protocol_treasury = protocol_treasury;
    config.validator_treasury = validator_treasury;
    config.network_treasury = network_treasury;
    config.protocol_treasury_bps = protocol_treasury_bps;
    config.validator_treasury_bps = validator_treasury_bps;
    config.network_treasury_bps = network_treasury_bps;
    config.paused = false;
    config.total_agents = 0;
    config.total_partners = 0;
    config.max_partner_collections = max_partner_collections;
    config.max_affiliate_bps = max_affiliate_bps;
    config.pending_authority = None;

    msg!(
        "Protocol initialized with genesis collection: {}",
        genesis_collection_mint
    );

    Ok(())
}
