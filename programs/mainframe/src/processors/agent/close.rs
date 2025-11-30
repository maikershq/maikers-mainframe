use crate::errors::MainframeError;
use crate::events::*;
use crate::instructions::CloseAgent;
use crate::state::AgentStatus;
use anchor_lang::prelude::*;

/// Close agent permanently
/// Can be called by agent owner OR protocol authority
pub fn close_agent(ctx: Context<CloseAgent>) -> Result<()> {
    let agent = &mut ctx.accounts.agent_account;

    // Verify signer is owner OR protocol authority (dual control)
    let is_owner = agent.owner == ctx.accounts.owner.key();
    let is_authority = ctx.accounts.protocol_config.authority == ctx.accounts.owner.key();

    require!(is_owner || is_authority, MainframeError::Unauthorized);

    msg!("Closing agent...");
    msg!(
        "  Signer: {}",
        if is_owner {
            "Owner"
        } else {
            "Protocol Authority"
        }
    );

    // Update agent status to permanently closed
    agent.status = AgentStatus::Closed;
    agent.updated_at = Clock::get()?.unix_timestamp;

    // Emit closure event
    emit!(AgentClosed {
        agent_account: agent.key(),
        owner: ctx.accounts.owner.key(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
