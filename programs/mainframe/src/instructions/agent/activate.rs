use crate::errors::MainframeError;
use crate::state::{AgentAccount, AgentStatus, ProtocolConfig};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ActivateAgent<'info> {
    #[account(
        mut,
        seeds = [b"agent", agent_account.nft_mint.as_ref()],
        bump,
        constraint = agent_account.status == AgentStatus::Pending @ MainframeError::InvalidAgentStatus
    )]
    pub agent_account: Account<'info, AgentAccount>,

    #[account(
        constraint = authority.key() == protocol_config.authority ||
                     authority.key() == protocol_config.manager @ MainframeError::Unauthorized
    )]
    pub authority: Signer<'info>,

    #[account(
        seeds = [b"protocol_config"],
        bump
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,
}
