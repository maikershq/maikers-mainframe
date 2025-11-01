use crate::errors::MainframeError;
use crate::instructions::InitializeConfig;
use crate::state::FeeStructure;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct TreasuryParams {
    pub protocol_treasury: Pubkey,
    pub validator_treasury: Pubkey,
    pub network_treasury: Pubkey,
    pub protocol_treasury_bps: u16,
    pub validator_treasury_bps: u16,
    pub network_treasury_bps: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ConfigParams {
    pub genesis_collection_mint: Pubkey,
    pub max_partner_collections: u64,
    pub max_affiliate_bps: u16,
    pub manager: Pubkey,
}

/// Initialize protocol configuration
pub fn initialize_config(
    ctx: Context<InitializeConfig>,
    fees: FeeStructure,
    treasury_params: TreasuryParams,
    config_params: ConfigParams,
) -> Result<()> {
    // Validate that basis points sum to 100% (10,000 bps) with checked arithmetic
    let total_bps = treasury_params.protocol_treasury_bps
        .checked_add(treasury_params.validator_treasury_bps)
        .and_then(|x| x.checked_add(treasury_params.network_treasury_bps))
        .ok_or(MainframeError::InvalidTreasuryDistribution)?;
    require!(
        total_bps == 10_000,
        MainframeError::InvalidTreasuryDistribution
    );

    // Validate affiliate bps doesn't exceed 100%
    require!(
        config_params.max_affiliate_bps <= 10_000,
        MainframeError::InvalidAffiliate
    );

    let config = &mut ctx.accounts.protocol_config;
    config.authority = ctx.accounts.authority.key();
    config.manager = config_params.manager;
    config.genesis_collection_mint = config_params.genesis_collection_mint;
    config.fees = fees;
    config.protocol_treasury = treasury_params.protocol_treasury;
    config.validator_treasury = treasury_params.validator_treasury;
    config.network_treasury = treasury_params.network_treasury;
    config.protocol_treasury_bps = treasury_params.protocol_treasury_bps;
    config.validator_treasury_bps = treasury_params.validator_treasury_bps;
    config.network_treasury_bps = treasury_params.network_treasury_bps;
    config.paused = false;
    config.total_agents = 0;
    config.total_partners = 0;
    config.max_partner_collections = config_params.max_partner_collections;
    config.max_affiliate_bps = config_params.max_affiliate_bps;
    config.pending_authority = None;

    msg!(
        "Protocol initialized with genesis collection: {}",
        config_params.genesis_collection_mint
    );

    Ok(())
}
