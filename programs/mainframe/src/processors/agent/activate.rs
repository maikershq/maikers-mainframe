use crate::errors::MainframeError;
use crate::events::AgentActivated;
use crate::instructions::ActivateAgent;
use crate::state::AgentStatus;
use anchor_lang::prelude::*;

pub fn activate_agent(ctx: Context<ActivateAgent>, agent_nft: Pubkey) -> Result<()> {
    let agent_account = &mut ctx.accounts.agent_account;
    let clock = Clock::get()?;

    require!(
        agent_account.status == AgentStatus::Pending,
        MainframeError::InvalidAgentStatus
    );

    agent_account.agent_nft = Some(agent_nft);
    agent_account.status = AgentStatus::Active;
    agent_account.updated_at = clock.unix_timestamp;

    emit!(AgentActivated {
        agent_account: ctx.accounts.agent_account.key(),
        nft_mint: agent_account.nft_mint,
        agent_nft,
        owner: agent_account.owner,
        timestamp: clock.unix_timestamp,
    });

    msg!("Agent activated with Agent-NFT: {}", agent_nft);
    Ok(())
}


