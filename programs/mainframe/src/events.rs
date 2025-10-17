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
