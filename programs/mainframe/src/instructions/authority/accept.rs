use anchor_lang::prelude::*;
use crate::errors::MainframeError;
use crate::state::ProtocolConfig;

/// Accept authority transfer (step 2 of 2-step transfer)
/// New authority must explicitly accept to prevent loss of control
#[derive(Accounts)]
pub struct AcceptAuthority<'info> {
    #[account(
        mut,
        seeds = [b"protocol_config"],
        bump,
        constraint = protocol_config.pending_authority.is_some() @ MainframeError::Unauthorized,
        constraint = protocol_config.pending_authority.as_ref().unwrap() == &new_authority.key() @ MainframeError::Unauthorized
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,
    
    #[account(mut)]
    pub new_authority: Signer<'info>,
}

