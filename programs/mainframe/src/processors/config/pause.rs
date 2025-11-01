use crate::instructions::Pause;
use anchor_lang::prelude::*;

/// Pause/unpause protocol
pub fn pause(ctx: Context<Pause>, paused: bool) -> Result<()> {
    ctx.accounts.protocol_config.paused = paused;
    Ok(())
}
