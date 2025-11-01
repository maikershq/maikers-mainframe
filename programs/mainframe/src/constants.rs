/// Maximum length for agent metadata URI
pub const MAX_METADATA_URI_LENGTH: usize = 200;

/// Maximum number of partner collections
pub const MAX_PARTNER_COLLECTIONS: usize = 100;

/// Maximum length for partner collection name
pub const MAX_PARTNER_NAME_LENGTH: usize = 50;

/// Seed for protocol authority PDA
pub const PROTOCOL_AUTHORITY_SEED: &[u8] = b"protocol_authority";

// ============================================================================
// Affiliate Program Constants
// ============================================================================

/// Affiliate tier thresholds and commission rates
pub const TIER_BRONZE_THRESHOLD: u64 = 0;
pub const TIER_BRONZE_BPS: u16 = 1500; // 15%

pub const TIER_SILVER_THRESHOLD: u64 = 100;
pub const TIER_SILVER_BPS: u16 = 2000; // 20%

pub const TIER_GOLD_THRESHOLD: u64 = 500;
pub const TIER_GOLD_BPS: u16 = 3000; // 30%

pub const TIER_PLATINUM_THRESHOLD: u64 = 2000;
pub const TIER_PLATINUM_BPS: u16 = 4000; // 40%

pub const TIER_DIAMOND_THRESHOLD: u64 = 10000;
pub const TIER_DIAMOND_BPS: u16 = 5000; // 50%

/// Referrer commission (percentage of affiliate's commission)
pub const REFERRER_BPS: u16 = 500; // 5% of affiliate commission
pub const MAX_REFERRAL_DEPTH: u8 = 1; // Single level only
