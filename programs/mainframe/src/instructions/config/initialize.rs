use anchor_lang::prelude::*;
use crate::state::ProtocolConfig;

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(
        init,
        payer = authority,
        space = ProtocolConfig::LEN,
        seeds = [b"protocol_config"],
        bump
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}
