use crate::errors::MainframeError;
use crate::state::ProtocolConfig;
use anchor_lang::prelude::*;

/// Propose new protocol authority (step 1 of 2-step transfer)
/// Two-step authority transfer prevents accidental lockout
#[derive(Accounts)]
pub struct ProposeAuthority<'info> {
    #[account(
        mut,
        seeds = [b"protocol_config"],
        bump,
        constraint = protocol_config.authority == current_authority.key() @ MainframeError::Unauthorized
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,

    #[account(mut)]
    pub current_authority: Signer<'info>,
}
