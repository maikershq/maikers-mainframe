use anchor_lang::prelude::*;
use crate::errors::MainframeError;
use crate::instructions::RemovePartnerCollection;

/// Remove partner collection
pub fn remove_partner_collection(ctx: Context<RemovePartnerCollection>, _collection_mint: Pubkey) -> Result<()> {
    // Mark as inactive (account will be closed automatically)
    ctx.accounts.partner_account.active = false;
    
    // Decrement partner counter
    ctx.accounts.protocol_config.total_partners = ctx.accounts.protocol_config.total_partners
        .checked_sub(1)
        .ok_or(MainframeError::CounterOverflow)?;

    Ok(())
}

