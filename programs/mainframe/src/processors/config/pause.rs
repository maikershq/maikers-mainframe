use anchor_lang::prelude::*;
use crate::instructions::Pause;

/// Pause/unpause protocol
pub fn pause(ctx: Context<Pause>, paused: bool) -> Result<()> {
    ctx.accounts.protocol_config.paused = paused;
    Ok(())
}

