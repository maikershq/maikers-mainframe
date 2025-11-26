use anchor_lang::prelude::*;

#[error_code]
pub enum MainframeError {
    #[msg("NFT not owned by the signer")]
    NFTNotOwned = 6000,

    #[msg("Agent metadata exceeds maximum size")]
    MetadataTooLarge,

    #[msg("Agent is not active")]
    AgentNotActive,

    #[msg("Insufficient fee payment")]
    InsufficientFee,

    #[msg("Protocol is paused")]
    ProtocolPaused,

    #[msg("Unauthorized operation")]
    Unauthorized,

    #[msg("Agent already exists for this NFT")]
    AgentAlreadyExists,

    #[msg("Invalid NFT metadata")]
    InvalidNFTMetadata,

    #[msg("Version counter overflow")]
    VersionOverflow,

    #[msg("Total agents counter overflow")]
    CounterOverflow,

    #[msg("Collection already exists in partner list")]
    CollectionAlreadyExists,

    #[msg("Collection not found in partner list")]
    CollectionNotFound,

    #[msg("Agent is already closed")]
    AgentAlreadyClosed,

    #[msg("Invalid or empty metadata URI")]
    InvalidMetadataUri,

    #[msg("Invalid discount percentage (must be 0-100)")]
    InvalidDiscountPercent,

    #[msg("Treasury distribution basis points must sum to 10,000 (100%)")]
    InvalidTreasuryDistribution,

    #[msg("Cross-program invocation unauthorized")]
    UnauthorizedCPI,

    #[msg("Metadata URI format invalid or exceeds length limit")]
    InvalidURIFormat,

    #[msg("Partner collection limit exceeded")]
    PartnerCollectionLimitExceeded,

    #[msg("Account relationship validation failed")]
    InvalidAccountRelationship,

    #[msg("Agent is not closed - cannot recover rent")]
    AgentNotClosed,

    #[msg("Insufficient balance for fee payment")]
    InsufficientBalance,

    #[msg("Invalid affiliate percentage (must be 0-5000 bps, max 50%)")]
    InvalidAffiliate,

    #[msg("Affiliate account not found")]
    AffiliateNotFound,

    #[msg("Unauthorized manager operation")]
    UnauthorizedManager,

    #[msg("Referral would create circular reference")]
    CircularReferral,

    #[msg("Maximum referral depth exceeded")]
    MaxReferralDepthExceeded,

    #[msg("Agent is already owned by the new owner")]
    AlreadyOwner,

    #[msg("NFT mint does not match agent account")]
    InvalidNFT,

    #[msg("Invalid treasury address - cannot be system program, protocol config, or program ID")]
    InvalidTreasuryAddress,

    #[msg("All three treasuries must be different addresses")]
    TreasuriesMustBeDifferent,

    #[msg("Treasury account mismatch - provided account doesn't match pubkey")]
    TreasuryAccountMismatch,

    #[msg("Invalid agent status for this operation")]
    InvalidAgentStatus,
}
