use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
pub struct PartnerCollectionAccount {
    pub collection_mint: Pubkey,
    pub discount_percent: u8,
    pub name: String,
    pub active: bool,
    pub added_at: i64,
    pub bump: u8,
}

impl PartnerCollectionAccount {
    pub const LEN: usize = 8 + // discriminator
        32 + // collection_mint
        1 + // discount_percent
        4 + MAX_PARTNER_NAME_LENGTH + // name
        1 + // active
        8 + // added_at
        1; // bump
}
