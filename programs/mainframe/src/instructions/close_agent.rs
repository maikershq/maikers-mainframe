use anchor_lang::prelude::*;
use crate::errors::MainframeError;
use crate::state::{AgentAccount, AgentStatus, ProtocolConfig};

#[derive(Accounts)]
pub struct CloseAgent<'info> {
    #[account(
        mut,
        seeds = [b"agent", agent_account.nft_mint.as_ref()],
        bump,
        constraint = agent_account.owner == owner.key() @ MainframeError::Unauthorized,
        constraint = agent_account.status != AgentStatus::Closed @ MainframeError::AgentAlreadyClosed
    )]
    pub agent_account: Account<'info, AgentAccount>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        seeds = [b"protocol_config"],
        bump,
        constraint = !protocol_config.paused @ MainframeError::ProtocolPaused
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,
}
