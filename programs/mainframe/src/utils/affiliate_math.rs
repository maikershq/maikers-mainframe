use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::AffiliateTier;

/// Calculate affiliate tier based on total sales
pub fn calculate_tier(total_sales: u64) -> AffiliateTier {
    if total_sales >= TIER_DIAMOND_THRESHOLD {
        AffiliateTier::Diamond
    } else if total_sales >= TIER_PLATINUM_THRESHOLD {
        AffiliateTier::Platinum
    } else if total_sales >= TIER_GOLD_THRESHOLD {
        AffiliateTier::Gold
    } else if total_sales >= TIER_SILVER_THRESHOLD {
        AffiliateTier::Silver
    } else {
        AffiliateTier::Bronze
    }
}

/// Get commission rate (bps) for a given tier
pub fn get_tier_commission_bps(tier: AffiliateTier) -> u16 {
    match tier {
        AffiliateTier::Bronze => TIER_BRONZE_BPS,
        AffiliateTier::Silver => TIER_SILVER_BPS,
        AffiliateTier::Gold => TIER_GOLD_BPS,
        AffiliateTier::Platinum => TIER_PLATINUM_BPS,
        AffiliateTier::Diamond => TIER_DIAMOND_BPS,
    }
}

/// Calculate total commission rate (tier only, no streak bonuses)
pub fn calculate_total_commission_bps(tier: AffiliateTier) -> u16 {
    get_tier_commission_bps(tier)
}

/// Calculate affiliate commission from total fee
pub fn calculate_affiliate_commission(
    total_fee: u64,
    commission_bps: u16,
) -> Result<u64> {
    let commission = total_fee
        .checked_mul(commission_bps as u64)
        .ok_or(ProgramError::ArithmeticOverflow)?
        .checked_div(10_000)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    Ok(commission)
}

/// Calculate referrer commission from affiliate's commission (5% of affiliate share)
pub fn calculate_referrer_commission(affiliate_commission: u64) -> Result<u64> {
    let commission = affiliate_commission
        .checked_mul(REFERRER_BPS as u64)
        .ok_or(ProgramError::ArithmeticOverflow)?
        .checked_div(10_000)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    Ok(commission)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_tier() {
        assert_eq!(calculate_tier(0), AffiliateTier::Bronze);
        assert_eq!(calculate_tier(99), AffiliateTier::Bronze);
        assert_eq!(calculate_tier(100), AffiliateTier::Silver);
        assert_eq!(calculate_tier(499), AffiliateTier::Silver);
        assert_eq!(calculate_tier(500), AffiliateTier::Gold);
        assert_eq!(calculate_tier(1999), AffiliateTier::Gold);
        assert_eq!(calculate_tier(2000), AffiliateTier::Platinum);
        assert_eq!(calculate_tier(9999), AffiliateTier::Platinum);
        assert_eq!(calculate_tier(10000), AffiliateTier::Diamond);
        assert_eq!(calculate_tier(100000), AffiliateTier::Diamond);
    }

    #[test]
    fn test_get_tier_commission_bps() {
        assert_eq!(get_tier_commission_bps(AffiliateTier::Bronze), 1500);
        assert_eq!(get_tier_commission_bps(AffiliateTier::Silver), 2000);
        assert_eq!(get_tier_commission_bps(AffiliateTier::Gold), 3000);
        assert_eq!(get_tier_commission_bps(AffiliateTier::Platinum), 4000);
        assert_eq!(get_tier_commission_bps(AffiliateTier::Diamond), 5000);
    }

    #[test]
    fn test_calculate_referrer_commission() {
        let affiliate_commission = 200_000; // 0.0002 SOL (20% of 0.001 SOL)
        let commission = calculate_referrer_commission(affiliate_commission).unwrap();
        assert_eq!(commission, 10_000); // 5% of affiliate = 0.00001 SOL
    }

    #[test]
    fn test_single_level_only() {
        assert_eq!(MAX_REFERRAL_DEPTH, 1);
        assert_eq!(REFERRER_BPS, 500); // 5%
    }
}

