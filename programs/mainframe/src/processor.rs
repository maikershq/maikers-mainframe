use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::constants::*;
use crate::errors::MainframeError;
use crate::events::*;
use crate::instructions::*;
use crate::state::{AgentStatus, FeeStructure, ProtocolConfig, AffiliateTier};
use crate::utils::*;

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
    affiliate_bonus_pool_bps: u16,
    max_partner_collections: u64,
    max_affiliate_bps: u16,
    manager: Pubkey,
) -> Result<()> {
    // Validate that basis points sum to 100% (10,000 bps)
    let total_bps = protocol_treasury_bps + validator_treasury_bps + network_treasury_bps + affiliate_bonus_pool_bps;
    require!(total_bps == 10_000, MainframeError::InvalidTreasuryDistribution);
    
    // Validate affiliate bps doesn't exceed 100%
    require!(max_affiliate_bps <= 10_000, MainframeError::InvalidAffiliate);
    
    let config = &mut ctx.accounts.protocol_config;
    config.authority = ctx.accounts.authority.key();
    config.manager = manager;
    config.fees = fees;
    config.protocol_treasury = protocol_treasury;
    config.validator_treasury = validator_treasury;
    config.network_treasury = network_treasury;
    config.protocol_treasury_bps = protocol_treasury_bps;
    config.validator_treasury_bps = validator_treasury_bps;
    config.network_treasury_bps = network_treasury_bps;
    config.affiliate_bonus_pool_bps = affiliate_bonus_pool_bps;
    config.paused = false;
    config.total_agents = 0;
    config.total_partners = 0;
    config.max_partner_collections = max_partner_collections;
    config.max_affiliate_bps = max_affiliate_bps;
    
    Ok(())
}

// create_agent processor implementation in separate file for modularity

/// Update agent configuration
pub fn update_agent_config(
    ctx: Context<UpdateAgentConfig>,
    new_metadata_uri: String,
) -> Result<()> {
    // Validate URI format
    require!(!new_metadata_uri.is_empty(), MainframeError::InvalidMetadataUri);
    require!(new_metadata_uri.len() <= MAX_METADATA_URI_LENGTH, MainframeError::InvalidURIFormat);
    
    // Calculate update fee (no partner discount for updates)
    let fee_amount = ctx.accounts.protocol_config.calculate_base_fee("update_agent_config");
    
    if fee_amount > 0 {
        ProtocolConfig::distribute_fee(&ctx.accounts.protocol_config,
            fee_amount,
            &ctx.accounts.owner.to_account_info(),
            &ctx.accounts.protocol_treasury.to_account_info(),
            &ctx.accounts.validator_treasury.to_account_info(),
            &ctx.accounts.network_treasury.to_account_info(),
            &ctx.accounts.system_program.to_account_info(),
        )?;
    }
    
    // Update agent account
    let old_version = ctx.accounts.agent_account.version;
    ctx.accounts.agent_account.metadata_uri = new_metadata_uri.clone();
    ctx.accounts.agent_account.updated_at = Clock::get()?.unix_timestamp;
    ctx.accounts.agent_account.version = ctx.accounts.agent_account.version
        .checked_add(1)
        .ok_or(MainframeError::VersionOverflow)?;
    
    // Emit update event
    emit!(AgentUpdated {
        agent_account: ctx.accounts.agent_account.key(),
        owner: ctx.accounts.owner.key(),
        metadata_uri: new_metadata_uri,
        old_version,
        new_version: ctx.accounts.agent_account.version,
        timestamp: Clock::get()?.unix_timestamp,
    });
    
    Ok(())
}

/// Transfer agent ownership
pub fn transfer_agent(ctx: Context<TransferAgent>) -> Result<()> {
    // Validate NFT ownership transfer
    require!(
        ctx.accounts.new_nft_token_account.owner == ctx.accounts.new_owner.key(),
        MainframeError::NFTNotOwned
    );
    
    // Calculate transfer fee (no partner discount for transfers)
    let fee_amount = ctx.accounts.protocol_config.calculate_base_fee("transfer_agent");
    
    if fee_amount > 0 {
        ProtocolConfig::distribute_fee(&ctx.accounts.protocol_config,
            fee_amount,
            &ctx.accounts.new_owner.to_account_info(),
            &ctx.accounts.protocol_treasury.to_account_info(),
            &ctx.accounts.validator_treasury.to_account_info(),
            &ctx.accounts.network_treasury.to_account_info(),
            &ctx.accounts.system_program.to_account_info(),
        )?;
    }
    
    // Update agent account ownership
    let old_owner = ctx.accounts.agent_account.owner;
    ctx.accounts.agent_account.owner = ctx.accounts.new_owner.key();
    ctx.accounts.agent_account.updated_at = Clock::get()?.unix_timestamp;
    
    // Emit transfer event
    emit!(AgentTransferred {
        agent_account: ctx.accounts.agent_account.key(),
        nft_mint: ctx.accounts.agent_account.nft_mint,
        old_owner,
        new_owner: ctx.accounts.new_owner.key(),
        timestamp: Clock::get()?.unix_timestamp,
    });
    
    Ok(())
}

/// Pause or resume agent
pub fn pause_agent(ctx: Context<PauseAgent>) -> Result<()> {
    // Toggle agent pause status
    let is_paused = match ctx.accounts.agent_account.status {
        AgentStatus::Active => {
            ctx.accounts.agent_account.status = AgentStatus::Paused;
            true
        },
        AgentStatus::Paused => {
            ctx.accounts.agent_account.status = AgentStatus::Active;
            false
        },
        _ => return Err(MainframeError::AgentNotActive.into()),
    };
    
    ctx.accounts.agent_account.updated_at = Clock::get()?.unix_timestamp;
    
    // Emit appropriate event
    if is_paused {
        emit!(AgentPaused {
            agent_account: ctx.accounts.agent_account.key(),
            owner: ctx.accounts.owner.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
    } else {
        emit!(AgentResumed {
            agent_account: ctx.accounts.agent_account.key(),
            owner: ctx.accounts.owner.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
    }
    
    Ok(())
}

/// Close agent permanently
pub fn close_agent(ctx: Context<CloseAgent>) -> Result<()> {
    // Update agent status to permanently closed
    ctx.accounts.agent_account.status = AgentStatus::Closed;
    ctx.accounts.agent_account.updated_at = Clock::get()?.unix_timestamp;
    
    // Emit closure event
    emit!(AgentClosed {
        agent_account: ctx.accounts.agent_account.key(),
        owner: ctx.accounts.owner.key(),
        timestamp: Clock::get()?.unix_timestamp,
    });
    
    Ok(())
}

/// Close agent account and recover rent (Protocol only)
pub fn close_agent_account(ctx: Context<CloseAgentAccount>) -> Result<()> {
    // Validate agent is closed
    require!(
        ctx.accounts.agent_account.status == AgentStatus::Closed,
        MainframeError::AgentNotClosed
    );

    // Emit account closure event
    emit!(AgentAccountClosed {
        agent_account: ctx.accounts.agent_account.key(),
        rent_recovered: ctx.accounts.agent_account.to_account_info().lamports(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

/// Pause/unpause protocol
pub fn pause(ctx: Context<Pause>, paused: bool) -> Result<()> {
    ctx.accounts.protocol_config.paused = paused;
    Ok(())
}

/// Update protocol authority
pub fn update_authority(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
    ctx.accounts.protocol_config.authority = new_authority;
    Ok(())
}

/// Update fee structure
pub fn update_fees(ctx: Context<UpdateFees>, new_fees: FeeStructure) -> Result<()> {
    ctx.accounts.protocol_config.fees = new_fees;
    Ok(())
}

/// Update protocol limits (Authority only)
pub fn update_protocol_limits(
    ctx: Context<UpdateProtocolLimits>, 
    max_partner_collections: u64,
    max_affiliate_bps: u16
) -> Result<()> {
    // Validate affiliate bps doesn't exceed 100%
    require!(max_affiliate_bps <= 10_000, MainframeError::InvalidAffiliate);
    
    let config = &mut ctx.accounts.protocol_config;
    config.max_partner_collections = max_partner_collections;
    config.max_affiliate_bps = max_affiliate_bps;
    
    Ok(())
}

/// Update treasury distribution
pub fn update_treasury_distribution(
    ctx: Context<UpdateTreasuryDistribution>,
    protocol_treasury_bps: u16,
    validator_treasury_bps: u16,
    network_treasury_bps: u16,
) -> Result<()> {
    // Validate that basis points sum to 100%
    let total_bps = protocol_treasury_bps + validator_treasury_bps + network_treasury_bps;
    require!(total_bps == 10_000, MainframeError::InvalidTreasuryDistribution);
    
    let config = &mut ctx.accounts.protocol_config;
    config.protocol_treasury_bps = protocol_treasury_bps;
    config.validator_treasury_bps = validator_treasury_bps;
    config.network_treasury_bps = network_treasury_bps;
    
    Ok(())
}

/// Add partner collection
pub fn add_partner_collection(
    ctx: Context<AddPartnerCollection>, 
    collection_mint: Pubkey,
    discount_percent: u8,
    name: String
) -> Result<()> {
    // Validate inputs
    require!(discount_percent <= 100, MainframeError::InvalidDiscountPercent);
    require!(name.len() <= MAX_PARTNER_NAME_LENGTH, MainframeError::InvalidURIFormat);
    require!(
        ctx.accounts.protocol_config.total_partners < ctx.accounts.protocol_config.max_partner_collections,
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
    ctx.accounts.protocol_config.total_partners = ctx.accounts.protocol_config.total_partners
        .checked_add(1)
        .ok_or(MainframeError::CounterOverflow)?;

    Ok(())
}

/// Remove partner collection
pub fn remove_partner_collection(ctx: Context<RemovePartnerCollection>, _collection_mint: Pubkey) -> Result<()> {
    // Mark as inactive (account will be closed automatically)
    ctx.accounts.partner_account.active = false;
    
    // Decrement partner counter
    ctx.accounts.protocol_config.total_partners = ctx.accounts.protocol_config.total_partners
        .checked_sub(1)
        .ok_or(MainframeError::CounterOverflow)?;

    Ok(())
}

