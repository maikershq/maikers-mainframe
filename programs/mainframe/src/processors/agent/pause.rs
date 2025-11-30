use crate::errors::MainframeError;
use crate::events::*;
use crate::instructions::PauseAgent;
use crate::state::AgentStatus;
use anchor_lang::prelude::*;

/// Pause or resume agent
/// Can be called by agent owner OR protocol authority
pub fn pause_agent(ctx: Context<PauseAgent>) -> Result<()> {
    let agent = &mut ctx.accounts.agent_account;

    // Verify signer is owner OR protocol authority (dual control)
    let is_owner = agent.owner == ctx.accounts.owner.key();
    let is_authority = ctx.accounts.protocol_config.authority == ctx.accounts.owner.key();

    require!(is_owner || is_authority, MainframeError::Unauthorized);

    msg!("Pause/Resume agent...");
    msg!(
        "  Signer: {}",
        if is_owner {
            "Owner"
        } else {
            "Protocol Authority"
        }
    );
    // Toggle agent pause status
    let is_paused = match ctx.accounts.agent_account.status {
        AgentStatus::Active => {
            ctx.accounts.agent_account.status = AgentStatus::Paused;
            true
        }
        AgentStatus::Paused => {
            ctx.accounts.agent_account.status = AgentStatus::Active;
            false
        }
        _ => return Err(MainframeError::AgentNotActive.into()),
    };

    ctx.accounts.agent_account.updated_at = Clock::get()?.unix_timestamp;

    // Emit appropriate event
    if is_paused {
        emit!(AgentPaused {
            agent_account: ctx.accounts.agent_account.key(),
            owner: ctx.accounts.owner.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
    } else {
        emit!(AgentResumed {
            agent_account: ctx.accounts.agent_account.key(),
            owner: ctx.accounts.owner.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
    }

    Ok(())
}
