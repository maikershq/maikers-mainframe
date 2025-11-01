use anchor_lang::prelude::*;
use crate::errors::MainframeError;
use crate::state::ProtocolConfig;

#[derive(Accounts)]
pub struct UpdateTreasuryDistribution<'info> {
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
