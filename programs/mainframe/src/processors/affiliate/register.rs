use crate::errors::MainframeError;
use crate::events::*;
use crate::instructions::RegisterAffiliate;
use anchor_lang::prelude::*;

/// Register new affiliate
pub fn register_affiliate(
    ctx: Context<RegisterAffiliate>,
    referrer_key: Option<Pubkey>,
) -> Result<()> {
    let affiliate_account = &mut ctx.accounts.affiliate_account;
    let clock = Clock::get()?;

    // Validate referrer if provided
    if let Some(ref_key) = referrer_key {
        // Check that referrer account exists
        require!(
            ctx.accounts.referrer.is_some(),
            MainframeError::AffiliateNotFound
        );
        let referrer_acc = ctx.accounts.referrer.as_ref().unwrap();

        // Prevent self-referral
        require!(
            ref_key != ctx.accounts.affiliate.key(),
            MainframeError::CircularReferral
        );

        // Single-level referral validation
        // Only direct referrals allowed (MAX_REFERRAL_DEPTH = 1)
        // Prevents referral chain saturation attacks with Sybil accounts

        // Only need to check for immediate circular reference
        // (A refers B, B tries to refer A - blocked)
        if let Some(ref_of_ref) = referrer_acc.referrer {
            if ref_of_ref == ctx.accounts.affiliate.key() {
                return Err(MainframeError::CircularReferral.into());
            }
        }

        msg!("Referral validated: direct referral only (1-level)");
    }

    // Initialize affiliate account
    affiliate_account.affiliate = ctx.accounts.affiliate.key();
    affiliate_account.total_sales = 0;
    affiliate_account.total_revenue = 0;
    affiliate_account.referral_count = 0;
    affiliate_account.referree_sales = 0;
    affiliate_account.referree_revenue = 0;
    affiliate_account.referrer = referrer_key;
    affiliate_account.created_at = clock.unix_timestamp;
    affiliate_account.bump = ctx.bumps.affiliate_account;
    affiliate_account.bonus_bps = 0;

    // Increment referral count for referrer
    if ctx.accounts.referrer.is_some() {
        let referrer = &mut ctx.accounts.referrer.as_mut().unwrap();
        referrer.referral_count = referrer
            .referral_count
            .checked_add(1)
            .ok_or(MainframeError::CounterOverflow)?;
    }

    emit!(AffiliateRegistered {
        affiliate: ctx.accounts.affiliate.key(),
        referrer: referrer_key,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
