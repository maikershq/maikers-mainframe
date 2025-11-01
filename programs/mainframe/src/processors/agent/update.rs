use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::MainframeError;
use crate::events::*;
use crate::instructions::UpdateAgentConfig;
use crate::state::ProtocolConfig;

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

