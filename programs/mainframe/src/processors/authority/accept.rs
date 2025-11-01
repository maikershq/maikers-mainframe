use anchor_lang::prelude::*;
use crate::errors::MainframeError;
use crate::instructions::AcceptAuthority;

/// Accept authority transfer (step 2 of 2)
/// New authority must explicitly accept to prevent typos
pub fn accept_authority_transfer(ctx: Context<AcceptAuthority>) -> Result<()> {
    let config = &mut ctx.accounts.protocol_config;
    
    // Additional validation for safety (redundant with constraints but safer)
    require!(
        config.pending_authority.is_some(),
        MainframeError::Unauthorized
    );
    require!(
        config.pending_authority.unwrap() == ctx.accounts.new_authority.key(),
        MainframeError::Unauthorized
    );
    
    let old_authority = config.authority;
    config.authority = ctx.accounts.new_authority.key();
    config.pending_authority = None;
    
    msg!("Authority transferred from {} to {}", old_authority, config.authority);
    
    Ok(())
}

