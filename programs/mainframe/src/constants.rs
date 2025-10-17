use anchor_lang::prelude::*;

/// Genesis collection with zero fees (maikers'collectibles)
pub const MAIKERS_COLLECTIBLES_MINT: Pubkey = pubkey!("mA1K3VFobNqs8xw16CCyU5S1mqEfDdJByjMLvczxVch");

/// Maximum length for agent metadata URI
pub const MAX_METADATA_URI_LENGTH: usize = 200;

/// Maximum number of partner collections
pub const MAX_PARTNER_COLLECTIONS: usize = 100;

/// Maximum length for partner collection name
pub const MAX_PARTNER_NAME_LENGTH: usize = 50;

/// Seed for protocol authority PDA
pub const PROTOCOL_AUTHORITY_SEED: &[u8] = b"protocol_authority";
