use anchor_lang::prelude::*;

#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "Maikers Mainframe",
    project_url: "https://mainframe.maikers.com",
    contacts: "email:security@maikers.com,discord:https://discord.gg/maikers,twitter:TheMaikers",
    policy: "https://github.com/maikershq/maikers-mainframe/blob/main/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/maikershq/maikers-mainframe",
    auditors: "To be announced",
    acknowledgements: "Thank you to our security researchers and the Solana community!"
}

pub mod constants;
pub mod errors;
pub mod events;
pub mod state;
pub mod instructions;
pub mod processor;

pub use constants::*;
pub use errors::*;
pub use events::*;
pub use state::*;
pub use instructions::*;
// Note: processor functions are only used internally by the program module

declare_id!("CtWQTpSjJGeWnsFWSr2otzNjjJZdK2UUUAaBHHCA9BWY");

#[program]
pub mod mainframe {
    use super::*;

    /// Initialize protocol configuration
    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        fees: FeeStructure,
        protocol_treasury: Pubkey,
        validator_treasury: Pubkey,
        network_treasury: Pubkey,
        protocol_treasury_bps: u16,
        validator_treasury_bps: u16,
        network_treasury_bps: u16,
        max_partner_collections: u64,
        max_affiliate_bps: u16,
    ) -> Result<()> {
        processor::initialize_config(
            ctx,
            fees,
            protocol_treasury,
            validator_treasury,
            network_treasury,
            protocol_treasury_bps,
            validator_treasury_bps,
            network_treasury_bps,
            max_partner_collections,
            max_affiliate_bps,
        )
    }

    /// Create new agent from NFT
    pub fn create_agent(
        ctx: Context<CreateAgent>,
        nft_mint: Pubkey,
        metadata_uri: String,
        seller_affiliate_bps: u16,
        collection_mint: Option<Pubkey>,
    ) -> Result<()> {
        processor::create_agent(ctx, nft_mint, metadata_uri, seller_affiliate_bps, collection_mint)
    }

    /// Update agent configuration
    pub fn update_agent_config(
        ctx: Context<UpdateAgentConfig>,
        new_metadata_uri: String,
    ) -> Result<()> {
        processor::update_agent_config(ctx, new_metadata_uri)
    }

    /// Transfer agent ownership
    pub fn transfer_agent(ctx: Context<TransferAgent>) -> Result<()> {
        processor::transfer_agent(ctx)
    }

    /// Pause or resume agent
    pub fn pause_agent(ctx: Context<PauseAgent>) -> Result<()> {
        processor::pause_agent(ctx)
    }

    /// Close agent permanently
    pub fn close_agent(ctx: Context<CloseAgent>) -> Result<()> {
        processor::close_agent(ctx)
    }

    /// Close agent account and recover rent (Protocol only)
    pub fn close_agent_account(ctx: Context<CloseAgentAccount>) -> Result<()> {
        processor::close_agent_account(ctx)
    }

    /// Pause/unpause protocol
    pub fn pause(ctx: Context<Pause>, paused: bool) -> Result<()> {
        processor::pause(ctx, paused)
    }

    /// Update protocol authority
    pub fn update_authority(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
        processor::update_authority(ctx, new_authority)
    }

    /// Update fee structure
    pub fn update_fees(ctx: Context<UpdateFees>, new_fees: FeeStructure) -> Result<()> {
        processor::update_fees(ctx, new_fees)
    }

    /// Update protocol limits
    pub fn update_protocol_limits(
        ctx: Context<UpdateProtocolLimits>, 
        max_partner_collections: u64,
        max_affiliate_bps: u16
    ) -> Result<()> {
        processor::update_protocol_limits(ctx, max_partner_collections, max_affiliate_bps)
    }

    /// Update treasury distribution
    pub fn update_treasury_distribution(
        ctx: Context<UpdateTreasuryDistribution>,
        protocol_treasury_bps: u16,
        validator_treasury_bps: u16,
        network_treasury_bps: u16,
    ) -> Result<()> {
        processor::update_treasury_distribution(
            ctx,
            protocol_treasury_bps,
            validator_treasury_bps,
            network_treasury_bps,
        )
    }

    /// Add partner collection
    pub fn add_partner_collection(
        ctx: Context<AddPartnerCollection>, 
        collection: Pubkey,
        discount_percent: u8,
        name: String
    ) -> Result<()> {
        processor::add_partner_collection(ctx, collection, discount_percent, name)
    }

    /// Remove partner collection
    pub fn remove_partner_collection(ctx: Context<RemovePartnerCollection>, collection: Pubkey) -> Result<()> {
        processor::remove_partner_collection(ctx, collection)
    }
}