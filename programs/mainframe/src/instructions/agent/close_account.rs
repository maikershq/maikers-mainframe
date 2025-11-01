use crate::errors::MainframeError;
use crate::state::{AgentAccount, AgentStatus, ProtocolConfig};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseAgentAccount<'info> {
    #[account(
        mut,
        close = rent_receiver,
        seeds = [b"agent", agent_account.nft_mint.as_ref()],
        bump,
        constraint = agent_account.status == AgentStatus::Closed @ MainframeError::AgentNotClosed
    )]
    pub agent_account: Account<'info, AgentAccount>,

    /// Protocol authority that can close accounts for rent recovery
    #[account(
        mut,
        constraint = authority.key() == protocol_config.authority @ MainframeError::Unauthorized
    )]
    pub authority: Signer<'info>,

    /// Protocol configuration to validate authority
    #[account(
        seeds = [b"protocol_config"],
        bump
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,

    /// Account that receives the recovered rent (protocol treasury)
    #[account(
        mut,
        constraint = rent_receiver.key() == protocol_config.protocol_treasury
    )]
    /// CHECK: Validated by protocol_config constraint
    pub rent_receiver: AccountInfo<'info>,
}
