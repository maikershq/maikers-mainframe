use anchor_lang::prelude::*;
use crate::errors::MainframeError;
use crate::instructions::UpdateProtocolLimits;

/// Update protocol limits (Authority only)
pub fn update_protocol_limits(
    ctx: Context<UpdateProtocolLimits>, 
    max_partner_collections: u64,
    max_affiliate_bps: u16
) -> Result<()> {
    // Validate affiliate bps doesn't exceed 100%
    require!(max_affiliate_bps <= 10_000, MainframeError::InvalidAffiliate);
    
    let config = &mut ctx.accounts.protocol_config;
    config.max_partner_collections = max_partner_collections;
    config.max_affiliate_bps = max_affiliate_bps;
    
    Ok(())
}

