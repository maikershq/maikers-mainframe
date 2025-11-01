use crate::state::AffiliateAccount;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RegisterAffiliate<'info> {
    #[account(
        init,
        payer = affiliate,
        space = AffiliateAccount::LEN,
        seeds = [b"affiliate", affiliate.key().as_ref()],
        bump
    )]
    pub affiliate_account: Account<'info, AffiliateAccount>,

    /// The affiliate registering
    #[account(mut)]
    pub affiliate: Signer<'info>,

    /// Optional referrer (who referred this affiliate)
    /// CHECK: Must be a valid affiliate account if provided
    pub referrer: Option<Account<'info, AffiliateAccount>>,

    pub system_program: Program<'info, System>,
}
