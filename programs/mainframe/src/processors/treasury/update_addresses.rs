use anchor_lang::prelude::*;
use crate::instructions::UpdateTreasuryAddresses;
use crate::errors::MainframeError;
use crate::events::TreasuryAddressesUpdated;

pub fn update_treasury_addresses(
    ctx: Context<UpdateTreasuryAddresses>,
    new_protocol_treasury: Pubkey,
    new_validator_treasury: Pubkey,
    new_network_treasury: Pubkey,
) -> Result<()> {
    // Get keys before mutable borrow to avoid borrow checker issues
    let protocol_config_key = ctx.accounts.protocol_config.key();
    let program_id = *ctx.program_id;

    // ========================================================================
    // SECURITY VALIDATIONS
    // ========================================================================
    
    // 1. Validate none of the new treasuries are the System Program
    require!(
        new_protocol_treasury != anchor_lang::system_program::ID,
        MainframeError::InvalidTreasuryAddress
    );
    require!(
        new_validator_treasury != anchor_lang::system_program::ID,
        MainframeError::InvalidTreasuryAddress
    );
    require!(
        new_network_treasury != anchor_lang::system_program::ID,
        MainframeError::InvalidTreasuryAddress
    );

    // 2. Validate none of the treasuries are the protocol config itself
    require!(
        new_protocol_treasury != protocol_config_key,
        MainframeError::InvalidTreasuryAddress
    );
    require!(
        new_validator_treasury != protocol_config_key,
        MainframeError::InvalidTreasuryAddress
    );
    require!(
        new_network_treasury != protocol_config_key,
        MainframeError::InvalidTreasuryAddress
    );

    // 3. Validate none of the treasuries are the program itself
    require!(
        new_protocol_treasury != program_id,
        MainframeError::InvalidTreasuryAddress
    );
    require!(
        new_validator_treasury != program_id,
        MainframeError::InvalidTreasuryAddress
    );
    require!(
        new_network_treasury != program_id,
        MainframeError::InvalidTreasuryAddress
    );

    // 4. Validate all three addresses are different (prevent consolidation exploit)
    require!(
        new_protocol_treasury != new_validator_treasury,
        MainframeError::TreasuriesMustBeDifferent
    );
    require!(
        new_protocol_treasury != new_network_treasury,
        MainframeError::TreasuriesMustBeDifferent
    );
    require!(
        new_validator_treasury != new_network_treasury,
        MainframeError::TreasuriesMustBeDifferent
    );

    // 5. Validate the accounts passed match the pubkeys provided (prevent mismatch exploit)
    require!(
        ctx.accounts.new_protocol_treasury.key() == new_protocol_treasury,
        MainframeError::TreasuryAccountMismatch
    );
    require!(
        ctx.accounts.new_validator_treasury.key() == new_validator_treasury,
        MainframeError::TreasuryAccountMismatch
    );
    require!(
        ctx.accounts.new_network_treasury.key() == new_network_treasury,
        MainframeError::TreasuryAccountMismatch
    );

    // ========================================================================
    // UPDATE TREASURY ADDRESSES
    // ========================================================================
    
    let protocol_config = &mut ctx.accounts.protocol_config;
    
    let old_protocol_treasury = protocol_config.protocol_treasury;
    let old_validator_treasury = protocol_config.validator_treasury;
    let old_network_treasury = protocol_config.network_treasury;

    protocol_config.protocol_treasury = new_protocol_treasury;
    protocol_config.validator_treasury = new_validator_treasury;
    protocol_config.network_treasury = new_network_treasury;

    // ========================================================================
    // EMIT EVENT FOR TRANSPARENCY AND INDEXING
    // ========================================================================
    
    emit!(TreasuryAddressesUpdated {
        authority: ctx.accounts.authority.key(),
        old_protocol_treasury,
        old_validator_treasury,
        old_network_treasury,
        new_protocol_treasury,
        new_validator_treasury,
        new_network_treasury,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!("Treasury addresses updated successfully");
    msg!("  Protocol: {} -> {}", old_protocol_treasury, new_protocol_treasury);
    msg!("  Validator: {} -> {}", old_validator_treasury, new_validator_treasury);
    msg!("  Network: {} -> {}", old_network_treasury, new_network_treasury);

    Ok(())
}

