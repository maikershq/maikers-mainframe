use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer, create_account, CreateAccount};
use crate::constants::MAX_METADATA_URI_LENGTH;
use crate::errors::MainframeError;
use crate::events::*;
use crate::instructions::CreateAgent;
use crate::state::{AgentStatus, ProtocolConfig, AffiliateAccount, AffiliateTier};
use crate::utils::{get_tier_commission_bps, calculate_affiliate_commission, calculate_referrer_commission, calculate_tier};
use mpl_token_metadata::accounts::Metadata;

/// Create new agent from NFT with permissionless operation
/// Validates NFT ownership via token account, accepts optional collection mint
/// Enforces protocol fees with genesis/partner discounts and affiliate commissions
pub fn create_agent(
    ctx: Context<CreateAgent>,
    nft_mint: Pubkey,
    metadata_uri: String,
    collection_mint: Option<Pubkey>,
) -> Result<()> {
    // Validate URI format
    require!(!metadata_uri.is_empty(), MainframeError::InvalidMetadataUri);
    require!(metadata_uri.len() <= MAX_METADATA_URI_LENGTH, MainframeError::InvalidURIFormat);

    // NFT ownership is validated by the token account constraints in the instruction
    
    // Verify collection if provided
    if let Some(collection) = &collection_mint {
        require!(
            ctx.accounts.nft_metadata.is_some(),
            MainframeError::InvalidNFTMetadata
        );
        
        let metadata_account = ctx.accounts.nft_metadata.as_ref().unwrap();
        
        // Derive expected metadata PDA
        let (expected_metadata_pda, _) = Pubkey::find_program_address(
            &[
                b"metadata",
                mpl_token_metadata::ID.as_ref(),
                nft_mint.as_ref(),
            ],
            &mpl_token_metadata::ID,
        );
        
        require!(
            metadata_account.key() == expected_metadata_pda,
            MainframeError::InvalidNFTMetadata
        );
        
        // Deserialize and check collection membership
        let metadata = Metadata::try_from(metadata_account)
            .map_err(|_| MainframeError::InvalidNFTMetadata)?;
        
        let nft_collection = metadata.collection
            .ok_or(MainframeError::InvalidNFTMetadata)?;
        
        // Only check that NFT belongs to the specified collection
        // No need to verify collection.verified = true
        require!(
            nft_collection.key == *collection,
            MainframeError::InvalidAccountRelationship
        );
    }

    let clock = Clock::get()?;
    
    // Calculate base fee with partner discounts
    let mut fee_amount = ctx.accounts.protocol_config.calculate_base_fee("create_agent");
    
    if let Some(collection) = &collection_mint {
        // Check if this is the genesis collection (zero fees)
        if *collection == ctx.accounts.protocol_config.genesis_collection_mint {
            fee_amount = 0;
        } else if let Some(partner) = &ctx.accounts.partner_account {
            // Validate partner account PDA to prevent fake discounts
            let (expected_partner_pda, _) = Pubkey::find_program_address(
                &[b"partner", collection.as_ref()],
                ctx.program_id,
            );
            
            require!(
                partner.key() == expected_partner_pda,
                MainframeError::InvalidAccountRelationship
            );
            
            require!(
                partner.collection_mint == *collection && partner.active,
                MainframeError::InvalidAccountRelationship
            );
            
            fee_amount = ProtocolConfig::apply_discount(fee_amount, partner.discount_percent);
        }
    }
    
    let affiliate_pubkey = ctx.accounts.affiliate.as_ref().map(|a| a.key());
    let mut total_affiliate_commission = 0u64;
    let mut referrer_commission = 0u64;

    // Process affiliate commission if affiliate provided
    if fee_amount > 0 && affiliate_pubkey.is_some() {
        let affiliate_key = affiliate_pubkey.unwrap();
        
        // Safe account deserialization with PDA validation
        let (tier, referrer_pubkey, needs_init) = if let Some(aff_acc_info) = &ctx.accounts.affiliate_account {
            // Validate PDA seeds to prevent fake affiliate accounts
            let (expected_affiliate_pda, _) = Pubkey::find_program_address(
                &[b"affiliate", affiliate_key.as_ref()],
                ctx.program_id,
            );
            
            require!(
                aff_acc_info.key() == expected_affiliate_pda,
                MainframeError::InvalidAccountRelationship
            );
            
            if aff_acc_info.data_is_empty() || aff_acc_info.lamports() == 0 {
                // Account doesn't exist - will initialize with first commission
                (AffiliateTier::Bronze, None, true)
            } else {
                // Validate account is owned by this program
                require!(
                    aff_acc_info.owner == ctx.program_id,
                    MainframeError::UnauthorizedCPI
                );
                
                // Safely deserialize with proper validation
                let data = aff_acc_info.try_borrow_data()?;
                if data.len() < AffiliateAccount::LEN {
                    (AffiliateTier::Bronze, None, true)
                } else {
                    // Deserialize safely using Anchor's deserialize
                    match AffiliateAccount::try_deserialize(&mut &data[..]) {
                        Ok(aff_acc) => {
                            // Validate account belongs to the affiliate
                            require!(
                                aff_acc.affiliate == affiliate_key,
                                MainframeError::InvalidAccountRelationship
                            );
                            
                            let tier = aff_acc.get_tier();
                            (tier, aff_acc.referrer, false)
                        },
                        Err(_) => {
                            // Invalid account format - treat as needs initialization
                            (AffiliateTier::Bronze, None, true)
                        }
                    }
                }
            }
        } else {
            // No affiliate account provided - use base tier
            (AffiliateTier::Bronze, None, false)
        };
        
        // Calculate commission (tier + bonus, capped at protocol maximum)
        let commission_bps = if !needs_init && ctx.accounts.affiliate_account.is_some() {
            // Load existing account to get bonus
            let aff_acc_info = ctx.accounts.affiliate_account.as_ref().unwrap();
            let data = aff_acc_info.try_borrow_data()?;
            match AffiliateAccount::try_deserialize(&mut &data[..]) {
                Ok(aff_acc) => aff_acc.get_commission_bps(ctx.accounts.protocol_config.max_affiliate_bps),
                Err(_) => {
                    // Fallback to tier-only for invalid accounts
                    let tier_bps = get_tier_commission_bps(tier);
                    tier_bps.min(ctx.accounts.protocol_config.max_affiliate_bps)
                }
            }
        } else {
            // New account or no account - use tier only
            let tier_bps = get_tier_commission_bps(tier);
            tier_bps.min(ctx.accounts.protocol_config.max_affiliate_bps)
        };
        
        total_affiliate_commission = calculate_affiliate_commission(fee_amount, commission_bps)?;
        
        // Calculate referrer commission if affiliate has referrer (5% of affiliate's commission)
        if referrer_pubkey.is_some() && ctx.accounts.referrer.is_some() {
            referrer_commission = calculate_referrer_commission(total_affiliate_commission)?;
        }
        
        // Initialize affiliate account if needed (using part of commission for rent)
        if needs_init && ctx.accounts.affiliate_account.is_some() {
            let aff_acc_info = ctx.accounts.affiliate_account.as_ref().unwrap();
            let rent = Rent::get()?;
            let rent_lamports = rent.minimum_balance(AffiliateAccount::LEN);
            
            // Ensure we have enough commission to cover rent
            if total_affiliate_commission >= rent_lamports {
                // Create account
                let (expected_pda, bump) = Pubkey::find_program_address(
                    &[b"affiliate", affiliate_key.as_ref()],
                    ctx.program_id
                );
                
                require!(
                    aff_acc_info.key() == expected_pda,
                    MainframeError::InvalidAccountRelationship
                );
                
                // Initialize account
                create_account(
                    CpiContext::new(
                        ctx.accounts.system_program.to_account_info(),
                        CreateAccount {
                            from: ctx.accounts.owner.to_account_info(),
                            to: aff_acc_info.to_account_info(),
                        }
                    ),
                    rent_lamports,
                    AffiliateAccount::LEN as u64,
                    ctx.program_id
                )?;
                
                // Initialize affiliate data
                let mut aff_data = aff_acc_info.try_borrow_mut_data()?;
                let affiliate_account = AffiliateAccount {
                    affiliate: affiliate_key,
                    total_sales: 1,
                    total_revenue: total_affiliate_commission - rent_lamports,
                    referral_count: 0,
                    referree_sales: 0,
                    referree_revenue: 0,
                    referrer: referrer_pubkey,
                    created_at: clock.unix_timestamp,
                    bump,
                    bonus_bps: 0,
                };
                
                // Serialize
                let mut data: &mut [u8] = &mut aff_data;
                affiliate_account.try_serialize(&mut data)?;
                
                // Reduce commission by rent amount
                total_affiliate_commission -= rent_lamports;
            }
        } else if !needs_init && ctx.accounts.affiliate_account.is_some() {
            // Safe update using proper deserialization
            let aff_acc_info = ctx.accounts.affiliate_account.as_ref().unwrap();
            
            // Validate account is owned by this program
            require!(
                aff_acc_info.owner == ctx.program_id,
                MainframeError::UnauthorizedCPI
            );
            
            // Load and validate account safely
            let mut data = aff_acc_info.try_borrow_mut_data()?;
            let mut aff_acc = AffiliateAccount::try_deserialize(&mut &data[..])
                .map_err(|_| MainframeError::InvalidAccountRelationship)?;
            
            require!(
                aff_acc.affiliate == affiliate_key,
                MainframeError::InvalidAccountRelationship
            );
            
            // Update stats safely with overflow checks
            aff_acc.total_sales = aff_acc.total_sales
                .checked_add(1)
                .ok_or(MainframeError::CounterOverflow)?;
            
            aff_acc.total_revenue = aff_acc.total_revenue
                .checked_add(total_affiliate_commission)
                .ok_or(ProgramError::ArithmeticOverflow)?;
            
            // Serialize and write back
            let mut writer: &mut [u8] = &mut data;
            aff_acc.try_serialize(&mut writer)?;
            
            // Check for tier upgrade and emit event if changed
            let old_tier = calculate_tier(aff_acc.total_sales - 1); // Previous sales count
            let new_tier = aff_acc.get_tier();
            
            if new_tier != old_tier {
                emit!(TierUpgraded {
                    affiliate: affiliate_key,
                    old_tier: old_tier as u8,
                    new_tier: new_tier as u8,
                    total_sales: aff_acc.total_sales,
                    timestamp: clock.unix_timestamp,
                });
            }
        }
        
        // Pay affiliate commission
        if total_affiliate_commission > 0 {
            transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.owner.to_account_info(),
                        to: ctx.accounts.affiliate.as_ref().unwrap().to_account_info(),
                    }
                ),
                total_affiliate_commission
            )?;
            
            emit!(AffiliatePaid {
                agent_account: ctx.accounts.agent_account.key(),
                seller: affiliate_key,
                affiliate_amount: total_affiliate_commission,
                affiliate_bps: commission_bps,
                timestamp: clock.unix_timestamp,
            });
        }
        
        // Pay referrer commission (5% of affiliate's commission)
        if referrer_commission > 0 && ctx.accounts.referrer.is_some() {
            transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.owner.to_account_info(),
                        to: ctx.accounts.referrer.as_ref().unwrap().to_account_info(),
                    }
                ),
                referrer_commission
            )?;
            
            // Update referrer's AffiliateAccount stats
            if let Some(ref_acc_info) = &ctx.accounts.referrer_account {
                if !ref_acc_info.data_is_empty() && ref_acc_info.lamports() > 0 {
                    let referrer_key = referrer_pubkey.unwrap();
                    
                    // Validate PDA seeds for referrer account
                    let (expected_referrer_pda, _) = Pubkey::find_program_address(
                        &[b"affiliate", referrer_key.as_ref()],
                        ctx.program_id,
                    );
                    
                    require!(
                        ref_acc_info.key() == expected_referrer_pda,
                        MainframeError::InvalidAccountRelationship
                    );
                    
                    require!(
                        ref_acc_info.owner == ctx.program_id,
                        MainframeError::UnauthorizedCPI
                    );
                    
                    let mut data = ref_acc_info.try_borrow_mut_data()?;
                    let mut ref_acc = AffiliateAccount::try_deserialize(&mut &data[..])
                        .map_err(|_| MainframeError::InvalidAccountRelationship)?;
                    
                    require!(
                        ref_acc.affiliate == referrer_key,
                        MainframeError::InvalidAccountRelationship
                    );
                    
                    // Update referree stats
                    ref_acc.referree_sales = ref_acc.referree_sales
                        .checked_add(1)
                        .ok_or(MainframeError::CounterOverflow)?;
                    
                    ref_acc.referree_revenue = ref_acc.referree_revenue
                        .checked_add(referrer_commission)
                        .ok_or(ProgramError::ArithmeticOverflow)?;
                    
                    // Serialize and write back
                    let mut writer: &mut [u8] = &mut data;
                    ref_acc.try_serialize(&mut writer)?;
                }
            }
        }
    }
    
    // Initialize agent account BEFORE transfers (prevents reentrancy)
    let agent_account = &mut ctx.accounts.agent_account;
    agent_account.nft_mint = nft_mint;
    agent_account.owner = ctx.accounts.owner.key();
    agent_account.collection_mint = collection_mint;
    agent_account.metadata_uri = metadata_uri.clone();
    agent_account.status = AgentStatus::Active;
    agent_account.activated_at = clock.unix_timestamp;
    agent_account.updated_at = clock.unix_timestamp;
    agent_account.version = 1;
    
    // Increment protocol counter BEFORE transfers
    ctx.accounts.protocol_config.total_agents = ctx.accounts.protocol_config.total_agents
        .checked_add(1)
        .ok_or(MainframeError::CounterOverflow)?;
    
    // Distribute remaining fee to treasuries (after affiliate and referrer)
    let remaining_fee = fee_amount
        .saturating_sub(total_affiliate_commission)
        .saturating_sub(referrer_commission);
    
    if remaining_fee > 0 {
        let protocol_config = &ctx.accounts.protocol_config;
        
        let protocol_fee = remaining_fee
            .checked_mul(protocol_config.protocol_treasury_bps as u64)
            .ok_or(ProgramError::ArithmeticOverflow)?
            .checked_div(10_000)
            .ok_or(ProgramError::ArithmeticOverflow)?;
            
        let validator_fee = remaining_fee
            .checked_mul(protocol_config.validator_treasury_bps as u64)
            .ok_or(ProgramError::ArithmeticOverflow)?
            .checked_div(10_000)
            .ok_or(ProgramError::ArithmeticOverflow)?;
            
        let network_fee = remaining_fee
            .checked_mul(protocol_config.network_treasury_bps as u64)
            .ok_or(ProgramError::ArithmeticOverflow)?
            .checked_div(10_000)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        
        if protocol_fee > 0 {
            transfer(CpiContext::new(ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.owner.to_account_info(),
                    to: ctx.accounts.protocol_treasury.to_account_info(),
                }), protocol_fee)?;
        }
        
        if validator_fee > 0 {
            transfer(CpiContext::new(ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.owner.to_account_info(),
                    to: ctx.accounts.validator_treasury.to_account_info(),
                }), validator_fee)?;
        }

        if network_fee > 0 {
            transfer(CpiContext::new(ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.owner.to_account_info(),
                    to: ctx.accounts.network_treasury.to_account_info(),
                }), network_fee)?;
        }
    }
    
    // Emit creation event
    emit!(AgentCreated {
        agent_account: ctx.accounts.agent_account.key(),
        nft_mint,
        owner: ctx.accounts.owner.key(),
        collection_mint,
        metadata_uri,
        seller: affiliate_pubkey,
        timestamp: clock.unix_timestamp,
        version: 1,
    });
    
    Ok(())
}

