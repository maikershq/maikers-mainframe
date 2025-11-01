use crate::constants::*;
use crate::errors::MainframeError;
use crate::instructions::AddPartnerCollection;
use anchor_lang::prelude::*;

/// Add partner collection
pub fn add_partner_collection(
    ctx: Context<AddPartnerCollection>,
    collection_mint: Pubkey,
    discount_percent: u8,
    name: String,
) -> Result<()> {
    // Validate inputs
    require!(
        discount_percent <= 100,
        MainframeError::InvalidDiscountPercent
    );
    require!(
        name.len() <= MAX_PARTNER_NAME_LENGTH,
        MainframeError::InvalidURIFormat
    );
    require!(
        ctx.accounts.protocol_config.total_partners
            < ctx.accounts.protocol_config.max_partner_collections,
        MainframeError::PartnerCollectionLimitExceeded
    );

    // Initialize partner account
    let partner = &mut ctx.accounts.partner_account;
    partner.collection_mint = collection_mint;
    partner.discount_percent = discount_percent;
    partner.name = name;
    partner.active = true;
    partner.added_at = Clock::get()?.unix_timestamp;
    partner.bump = ctx.bumps.partner_account;

    // Increment partner counter
    ctx.accounts.protocol_config.total_partners = ctx
        .accounts
        .protocol_config
        .total_partners
        .checked_add(1)
        .ok_or(MainframeError::CounterOverflow)?;

    Ok(())
}
