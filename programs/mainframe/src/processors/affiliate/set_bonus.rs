use crate::errors::MainframeError;
use crate::events::*;
use crate::instructions::SetAffiliateBonus;
use anchor_lang::prelude::*;

/// Set custom affiliate bonus (Authority only)
/// Used for special deals with influencers and strategic partners
pub fn set_affiliate_bonus(ctx: Context<SetAffiliateBonus>, bonus_bps: u16) -> Result<()> {
    // Validate bonus doesn't exceed protocol maximum
    require!(
        bonus_bps <= ctx.accounts.protocol_config.max_affiliate_bps,
        MainframeError::InvalidAffiliate
    );

    let affiliate = &mut ctx.accounts.affiliate_account;
    let clock = Clock::get()?;

    // Update bonus (setting to 0 removes bonus)
    affiliate.bonus_bps = bonus_bps;

    emit!(AffiliateBonusSet {
        affiliate: affiliate.affiliate,
        bonus_bps,
        set_by: ctx.accounts.signer.key(),
        timestamp: clock.unix_timestamp,
    });

    if bonus_bps == 0 {
        msg!("Bonus removed for affiliate: {}", affiliate.affiliate);
    } else {
        msg!(
            "Bonus set to {}bps for affiliate: {}",
            bonus_bps,
            affiliate.affiliate
        );
    }

    Ok(())
}
