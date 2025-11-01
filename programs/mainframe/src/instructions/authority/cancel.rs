use anchor_lang::prelude::*;
use crate::errors::MainframeError;
use crate::state::ProtocolConfig;

/// Cancel pending authority transfer
/// Current authority can cancel if they change their mind
#[derive(Accounts)]
pub struct CancelAuthority<'info> {
    #[account(
        mut,
        seeds = [b"protocol_config"],
        bump,
        constraint = protocol_config.authority == authority.key() @ MainframeError::Unauthorized
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}

