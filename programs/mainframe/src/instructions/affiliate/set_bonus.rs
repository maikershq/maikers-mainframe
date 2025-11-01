use anchor_lang::prelude::*;
use crate::errors::MainframeError;
use crate::state::{AffiliateAccount, ProtocolConfig};

#[derive(Accounts)]
pub struct SetAffiliateBonus<'info> {
    #[account(
        mut,
        seeds = [b"affiliate", affiliate_account.affiliate.as_ref()],
        bump = affiliate_account.bump
    )]
    pub affiliate_account: Account<'info, AffiliateAccount>,
    
    #[account(
        seeds = [b"protocol_config"],
        bump,
        constraint = protocol_config.authority == signer.key() || protocol_config.manager == signer.key() @ MainframeError::Unauthorized
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,
    
    #[account(mut)]
    pub signer: Signer<'info>,
}

