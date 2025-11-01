use anchor_lang::prelude::*;
use crate::errors::MainframeError;
use crate::events::*;
use crate::instructions::PauseAgent;
use crate::state::AgentStatus;

/// Pause or resume agent
pub fn pause_agent(ctx: Context<PauseAgent>) -> Result<()> {
    // Toggle agent pause status
    let is_paused = match ctx.accounts.agent_account.status {
        AgentStatus::Active => {
            ctx.accounts.agent_account.status = AgentStatus::Paused;
            true
        },
        AgentStatus::Paused => {
            ctx.accounts.agent_account.status = AgentStatus::Active;
            false
        },
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

