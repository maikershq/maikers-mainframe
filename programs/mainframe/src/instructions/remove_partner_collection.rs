use anchor_lang::prelude::*;
use crate::errors::MainframeError;
use crate::state::{ProtocolConfig, PartnerCollectionAccount};

#[derive(Accounts)]
#[instruction(collection_mint: Pubkey)]
pub struct RemovePartnerCollection<'info> {
    #[account(
        mut,
        close = authority,
        seeds = [b"partner", collection_mint.as_ref()],
        bump,
        constraint = partner_account.active @ MainframeError::CollectionNotFound
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
}
