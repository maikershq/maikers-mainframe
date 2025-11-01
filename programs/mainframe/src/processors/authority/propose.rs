use anchor_lang::prelude::*;
use crate::errors::MainframeError;
use crate::instructions::ProposeAuthority;

/// Propose new protocol authority (step 1 of 2)
/// Two-step authority transfer prevents accidental lockout
pub fn propose_authority_transfer(
    ctx: Context<ProposeAuthority>,
    new_authority: Pubkey,
) -> Result<()> {
    // Validate new authority is not zero address
    require!(
        new_authority != Pubkey::default(),
        MainframeError::Unauthorized
    );
    
    // Validate not transferring to self
    require!(
        new_authority != ctx.accounts.protocol_config.authority,
        MainframeError::Unauthorized
    );
    
    ctx.accounts.protocol_config.pending_authority = Some(new_authority);
    
    msg!("Authority transfer proposed to: {}", new_authority);
    msg!("New authority must call accept_authority_transfer to complete");
    
    Ok(())
}

