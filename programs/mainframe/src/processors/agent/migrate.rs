use crate::errors::MainframeError;
use crate::instructions::MigrateAgent;
use crate::state::AgentStatus;
use anchor_lang::prelude::*;

/// Migrate existing agent to new status system
/// Sets status = Pending for agents without Agent-NFT
pub fn migrate_agent(ctx: Context<MigrateAgent>) -> Result<()> {
    let agent = &mut ctx.accounts.agent_account;

    // Verify signer is authority or owner
    let is_authority = ctx.accounts.signer.key() == ctx.accounts.protocol_config.authority;
    let is_owner = ctx.accounts.signer.key() == agent.owner;

    require!(is_authority || is_owner, MainframeError::Unauthorized);

    msg!("Migrating agent...");
    msg!("  Agent: {}", agent.key());
    msg!("  Owner: {}", agent.owner);
    msg!("  Current status: {:?}", agent.status);
    msg!("  Agent-NFT: {:?}", agent.agent_nft);

    // Set status to Pending if no Agent-NFT
    if agent.agent_nft.is_none() {
        agent.status = AgentStatus::Pending;
        agent.updated_at = Clock::get()?.unix_timestamp;
        msg!("✓ Status updated: → Pending");
    } else {
        msg!("⏭️  Agent already has Agent-NFT, no migration needed");
    }

    Ok(())
}
