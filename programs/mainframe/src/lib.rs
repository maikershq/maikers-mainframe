#![allow(ambiguous_glob_reexports)]
#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "Mainframe",
    project_url: "https://mainframe.maikers.com",
    contacts: "email:security@maikers.com,discord:https://discord.gg/maikers,twitter:TheMaikers",
    policy: "https://github.com/maikershq/maikers-mainframe/blob/main/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/maikershq/maikers-mainframe",
    source_revision: "4fbeee47e3d548c59ba7f3d4abf6e90ccf8414e8",
    auditors: "To be announced",
    acknowledgements: "Thank you to our security researchers and the Solana community!"
}

pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod processors;
pub mod state;
pub mod utils;

pub use constants::*;
pub use errors::*;
pub use events::*;
pub use instructions::*;
pub use state::*;
pub use utils::*;

declare_id!("mnfm211AwTDA8fGvPezYs3jjxAXgoucHGuTMUbjFssE");

#[program]
pub mod mainframe {
    use super::*;

    /// Initialize protocol configuration
    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        fees: FeeStructure,
        genesis_collection_mint: Pubkey,
        protocol_treasury: Pubkey,
        validator_treasury: Pubkey,
        network_treasury: Pubkey,
        protocol_treasury_bps: u16,
        validator_treasury_bps: u16,
        network_treasury_bps: u16,
        max_partner_collections: u64,
        max_affiliate_bps: u16,
        manager: Pubkey,
    ) -> Result<()> {
        processors::config::initialize_config(
            ctx,
            fees,
            genesis_collection_mint,
            protocol_treasury,
            validator_treasury,
            network_treasury,
            protocol_treasury_bps,
            validator_treasury_bps,
            network_treasury_bps,
            max_partner_collections,
            max_affiliate_bps,
            manager,
        )
    }

    /// Create new agent from NFT
    pub fn create_agent(
        ctx: Context<CreateAgent>,
        nft_mint: Pubkey,
        metadata_uri: String,
        collection_mint: Option<Pubkey>,
    ) -> Result<()> {
        processors::agent::create_agent(ctx, nft_mint, metadata_uri, collection_mint)
    }

    /// Update agent configuration
    pub fn update_agent_config(
        ctx: Context<UpdateAgentConfig>,
        new_metadata_uri: String,
    ) -> Result<()> {
        processors::agent::update_agent_config(ctx, new_metadata_uri)
    }

    /// Transfer agent ownership
    pub fn transfer_agent(ctx: Context<TransferAgent>) -> Result<()> {
        processors::agent::transfer_agent(ctx)
    }

    /// Pause or resume agent
    pub fn pause_agent(ctx: Context<PauseAgent>) -> Result<()> {
        processors::agent::pause_agent(ctx)
    }

    /// Close agent permanently
    pub fn close_agent(ctx: Context<CloseAgent>) -> Result<()> {
        processors::agent::close_agent(ctx)
    }

    /// Close agent account and recover rent (Protocol only)
    pub fn close_agent_account(ctx: Context<CloseAgentAccount>) -> Result<()> {
        processors::agent::close_agent_account(ctx)
    }

    /// Pause/unpause protocol
    pub fn pause(ctx: Context<Pause>, paused: bool) -> Result<()> {
        processors::config::pause(ctx, paused)
    }

    /// Propose new protocol authority (step 1 of 2-step transfer)
    /// Two-step authority transfer prevents accidental lockout
    pub fn propose_authority_transfer(
        ctx: Context<ProposeAuthority>,
        new_authority: Pubkey,
    ) -> Result<()> {
        processors::authority::propose_authority_transfer(ctx, new_authority)
    }

    /// Accept authority transfer (step 2 of 2-step transfer)
    /// New authority must explicitly accept
    pub fn accept_authority_transfer(ctx: Context<AcceptAuthority>) -> Result<()> {
        processors::authority::accept_authority_transfer(ctx)
    }

    /// Cancel pending authority transfer
    /// Current authority can cancel if needed
    pub fn cancel_authority_transfer(ctx: Context<CancelAuthority>) -> Result<()> {
        processors::authority::cancel_authority_transfer(ctx)
    }

    /// Update fee structure
    pub fn update_fees(ctx: Context<UpdateFees>, new_fees: FeeStructure) -> Result<()> {
        processors::treasury::update_fees(ctx, new_fees)
    }

    /// Update protocol limits
    pub fn update_protocol_limits(
        ctx: Context<UpdateProtocolLimits>,
        max_partner_collections: u64,
        max_affiliate_bps: u16,
    ) -> Result<()> {
        processors::treasury::update_protocol_limits(
            ctx,
            max_partner_collections,
            max_affiliate_bps,
        )
    }

    /// Update treasury distribution
    pub fn update_treasury_distribution(
        ctx: Context<UpdateTreasuryDistribution>,
        protocol_treasury_bps: u16,
        validator_treasury_bps: u16,
        network_treasury_bps: u16,
    ) -> Result<()> {
        processors::treasury::update_treasury_distribution(
            ctx,
            protocol_treasury_bps,
            validator_treasury_bps,
            network_treasury_bps,
        )
    }

    /// Update treasury addresses
    pub fn update_treasury_addresses(
        ctx: Context<UpdateTreasuryAddresses>,
        new_protocol_treasury: Pubkey,
        new_validator_treasury: Pubkey,
        new_network_treasury: Pubkey,
    ) -> Result<()> {
        processors::treasury::update_treasury_addresses(
            ctx,
            new_protocol_treasury,
            new_validator_treasury,
            new_network_treasury,
        )
    }

    /// Add partner collection
    pub fn add_partner_collection(
        ctx: Context<AddPartnerCollection>,
        collection: Pubkey,
        discount_percent: u8,
        name: String,
    ) -> Result<()> {
        processors::partner::add_partner_collection(ctx, collection, discount_percent, name)
    }

    /// Remove partner collection
    pub fn remove_partner_collection(
        ctx: Context<RemovePartnerCollection>,
        collection: Pubkey,
    ) -> Result<()> {
        processors::partner::remove_partner_collection(ctx, collection)
    }

    // ========================================================================
    // Affiliate Program Instructions
    // ========================================================================

    /// Register as an affiliate
    pub fn register_affiliate(
        ctx: Context<RegisterAffiliate>,
        referrer: Option<Pubkey>,
    ) -> Result<()> {
        processors::affiliate::register_affiliate(ctx, referrer)
    }

    /// Set custom affiliate bonus (Authority or Manager)
    pub fn set_affiliate_bonus(ctx: Context<SetAffiliateBonus>, bonus_bps: u16) -> Result<()> {
        processors::affiliate::set_affiliate_bonus(ctx, bonus_bps)
    }
}
