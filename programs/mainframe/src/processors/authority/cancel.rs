use crate::instructions::CancelAuthority;
use anchor_lang::prelude::*;

/// Cancel pending authority transfer
/// Current authority can cancel if they made a mistake
pub fn cancel_authority_transfer(ctx: Context<CancelAuthority>) -> Result<()> {
    ctx.accounts.protocol_config.pending_authority = None;

    msg!("Authority transfer cancelled");

    Ok(())
}
