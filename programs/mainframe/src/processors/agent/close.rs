use crate::errors::MainframeError;
use crate::events::*;
use crate::instructions::{CloseAgent, CloseAgentAccount};
use crate::state::AgentStatus;
use anchor_lang::prelude::*;

/// Close agent permanently
pub fn close_agent(ctx: Context<CloseAgent>) -> Result<()> {
    // Update agent status to permanently closed
    ctx.accounts.agent_account.status = AgentStatus::Closed;
    ctx.accounts.agent_account.updated_at = Clock::get()?.unix_timestamp;

    // Emit closure event
    emit!(AgentClosed {
        agent_account: ctx.accounts.agent_account.key(),
        owner: ctx.accounts.owner.key(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

/// Close agent account and recover rent (Protocol only)
pub fn close_agent_account(ctx: Context<CloseAgentAccount>) -> Result<()> {
    // Validate agent is closed
    require!(
        ctx.accounts.agent_account.status == AgentStatus::Closed,
        MainframeError::AgentNotClosed
    );

    // Emit account closure event
    emit!(AgentAccountClosed {
        agent_account: ctx.accounts.agent_account.key(),
        rent_recovered: ctx.accounts.agent_account.to_account_info().lamports(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
