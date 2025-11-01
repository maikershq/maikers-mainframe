use crate::errors::MainframeError;
use crate::instructions::UpdateTreasuryDistribution;
use anchor_lang::prelude::*;

/// Update treasury distribution
pub fn update_treasury_distribution(
    ctx: Context<UpdateTreasuryDistribution>,
    protocol_treasury_bps: u16,
    validator_treasury_bps: u16,
    network_treasury_bps: u16,
) -> Result<()> {
    // Validate that basis points sum to 100% with checked arithmetic
    let total_bps = protocol_treasury_bps
        .checked_add(validator_treasury_bps)
        .and_then(|x| x.checked_add(network_treasury_bps))
        .ok_or(MainframeError::InvalidTreasuryDistribution)?;
    require!(
        total_bps == 10_000,
        MainframeError::InvalidTreasuryDistribution
    );

    let config = &mut ctx.accounts.protocol_config;
    config.protocol_treasury_bps = protocol_treasury_bps;
    config.validator_treasury_bps = validator_treasury_bps;
    config.network_treasury_bps = network_treasury_bps;

    Ok(())
}
