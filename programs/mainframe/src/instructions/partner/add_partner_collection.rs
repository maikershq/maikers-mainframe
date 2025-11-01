use crate::errors::MainframeError;
use crate::state::{PartnerCollectionAccount, ProtocolConfig};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(collection_mint: Pubkey)]
pub struct AddPartnerCollection<'info> {
    #[account(
        init,
        payer = signer,
        space = PartnerCollectionAccount::LEN,
        seeds = [b"partner", collection_mint.as_ref()],
        bump
    )]
    pub partner_account: Account<'info, PartnerCollectionAccount>,

    #[account(
        mut,
        seeds = [b"protocol_config"],
        bump,
        constraint = protocol_config.authority == signer.key() || protocol_config.manager == signer.key() @ MainframeError::Unauthorized
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
