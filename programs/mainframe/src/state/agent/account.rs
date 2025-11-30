use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
pub struct AgentAccount {
    /// The NFT mint associated with this agent (source NFT)
    pub nft_mint: Pubkey,
    /// The owner of the NFT and agent
    pub owner: Pubkey,
    /// The collection this NFT belongs to
    pub collection_mint: Option<Pubkey>,
    /// URI pointing to secure JSON metadata
    pub metadata_uri: String,
    /// Minted Agent-NFT representing this agent (None = Pending)
    pub agent_nft: Option<Pubkey>,
    /// Agent operational status
    pub status: AgentStatus,
    /// Timestamp of activation
    pub activated_at: i64,
    /// Last update timestamp
    pub updated_at: i64,
    /// Version for configuration updates
    pub version: u64,
    /// Reserved space for future upgrades
    pub reserved: [u8; 32],
}

impl AgentAccount {
    pub const LEN: usize = 8 + // discriminator
        32 + // nft_mint
        32 + // owner
        1 + 32 + // collection_mint (Option<Pubkey>)
        4 + MAX_METADATA_URI_LENGTH + // metadata_uri (String)
        1 + 32 + // agent_nft (Option<Pubkey>) - NEW
        1 + // status (enum)
        8 + // activated_at
        8 + // updated_at
        8 + // version
        32; // reserved
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum AgentStatus {
    Active,  // 0 - Agent-NFT minted and running (legacy agents)
    Paused,  // 1 - Temporarily stopped
    Closed,  // 2 - Permanently closed
    Pending, // 3 - Waiting for Agent-NFT to be minted
    Error,   // 4 - Validation or minting failed
}
