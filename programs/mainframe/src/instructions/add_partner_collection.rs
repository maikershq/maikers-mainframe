use anchor_lang::prelude::*;
use crate::errors::MainframeError;
use crate::state::{ProtocolConfig, PartnerCollectionAccount};

#[derive(Accounts)]
#[instruction(collection_mint: Pubkey)]
pub struct AddPartnerCollection<'info> {
    #[account(
        init,
        payer = authority,
        space = PartnerCollectionAccount::LEN,
        seeds = [b"partner", collection_mint.as_ref()],
        bump
    )]
    pub partner_account: Account<'info, PartnerCollectionAccount>,
    
    #[account(
        mut,
        seeds = [b"protocol_config"],
        bump,
        constraint = protocol_config.authority == authority.key() @ MainframeError::Unauthorized
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}
