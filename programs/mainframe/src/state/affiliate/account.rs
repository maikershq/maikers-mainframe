use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum AffiliateTier {
    Bronze,    // 0-99 sales (15%)
    Silver,    // 100-499 sales (20%)
    Gold,      // 500-1,999 sales (30%)
    Platinum,  // 2,000-9,999 sales (40%)
    Diamond,   // 10,000+ sales (50%)
}

#[account]
pub struct AffiliateAccount {
    /// The affiliate's wallet address
    pub affiliate: Pubkey,
    /// Total number of agent sales
    pub total_sales: u64,
    /// Total revenue earned (lamports)
    pub total_revenue: u64,
    /// Number of direct referrals (Level 1)
    pub referral_count: u64,
    /// Total sales made by referrals
    pub referree_sales: u64,
    /// Total revenue earned from referral commissions
    pub referree_revenue: u64,
    /// Who referred this affiliate (single-level referrals only)
    pub referrer: Option<Pubkey>,
    /// Account creation timestamp
    pub created_at: i64,
    /// Custom bonus rate in basis points (set by authority or manager for special deals)
    pub bonus_bps: u16,
    /// Bump seed for PDA
    pub bump: u8,
}

impl AffiliateAccount {
    pub const LEN: usize = 8 + // discriminator
        32 + // affiliate
        8 + // total_sales
        8 + // total_revenue
        8 + // referral_count
        8 + // referree_sales
        8 + // referree_revenue
        1 + 32 + // referrer (Option<Pubkey>)
        8 + // created_at
        2 + // bonus_bps
        1; // bump
    
    /// Get current tier based on total sales
    pub fn get_tier(&self) -> AffiliateTier {
        crate::utils::calculate_tier(self.total_sales)
    }
    
    /// Get total commission rate (tier + bonus, capped at protocol max)
    pub fn get_commission_bps(&self, max_affiliate_bps: u16) -> u16 {
        let tier_bps = crate::utils::get_tier_commission_bps(self.get_tier());
        let total_bps = tier_bps.saturating_add(self.bonus_bps);
        total_bps.min(max_affiliate_bps)
    }
}
