use crate::errors::MainframeError;
use crate::events::*;
use crate::instructions::agent::CloseAgentAccount;
use crate::state::AgentStatus;
use anchor_lang::prelude::*;

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
