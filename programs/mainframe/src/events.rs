use anchor_lang::prelude::*;

#[event]
pub struct AgentCreated {
    pub agent_account: Pubkey,
    pub nft_mint: Pubkey,
    pub owner: Pubkey,
    pub collection_mint: Option<Pubkey>,
    pub metadata_uri: String,
    pub seller: Option<Pubkey>,
    pub timestamp: i64,
    pub version: u64,
}

#[event]
pub struct AgentUpdated {
    pub agent_account: Pubkey,
    pub owner: Pubkey,
    pub metadata_uri: String,
    pub old_version: u64,
    pub new_version: u64,
    pub timestamp: i64,
}

#[event]
pub struct AgentTransferred {
    pub agent_account: Pubkey,
    pub nft_mint: Pubkey,
    pub old_owner: Pubkey,
    pub new_owner: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct AgentPaused {
    pub agent_account: Pubkey,
    pub owner: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct AgentResumed {
    pub agent_account: Pubkey,
    pub owner: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct AgentClosed {
    pub agent_account: Pubkey,
    pub owner: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct AgentAccountClosed {
    pub agent_account: Pubkey,
    pub rent_recovered: u64,
    pub timestamp: i64,
}

#[event]
pub struct AffiliatePaid {
    pub agent_account: Pubkey,
    pub seller: Pubkey,
    pub affiliate_amount: u64,
    pub affiliate_bps: u16,
    pub timestamp: i64,
}

// ============================================================================
// Affiliate Program Events
// ============================================================================

#[event]
pub struct AffiliateRegistered {
    pub affiliate: Pubkey,
    pub referrer: Option<Pubkey>,
    pub timestamp: i64,
}

#[event]
pub struct TierUpgraded {
    pub affiliate: Pubkey,
    pub old_tier: u8,
    pub new_tier: u8,
    pub total_sales: u64,
    pub timestamp: i64,
}

#[event]
pub struct AffiliateBonusSet {
    pub affiliate: Pubkey,
    pub bonus_bps: u16,
    pub set_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct TreasuryAddressesUpdated {
    pub authority: Pubkey,
    pub old_protocol_treasury: Pubkey,
    pub old_validator_treasury: Pubkey,
    pub old_network_treasury: Pubkey,
    pub new_protocol_treasury: Pubkey,
    pub new_validator_treasury: Pubkey,
    pub new_network_treasury: Pubkey,
    pub timestamp: i64,
}
