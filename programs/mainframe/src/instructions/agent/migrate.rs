use crate::errors::MainframeError;
use crate::state::{AgentAccount, AgentStatus, ProtocolConfig};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct MigrateAgent<'info> {
    /// Agent account (must be able to deserialize - so already in new format)
    #[account(mut)]
    pub agent_account: Account<'info, AgentAccount>,

    #[account(
        seeds = [b"protocol_config"],
        bump,
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,

    /// Authority can migrate any agent, owner can migrate their own
    pub signer: Signer<'info>,
}
