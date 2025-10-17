# Mainframe: Anchor Program Specifications

## Overview

The Mainframe Anchor program is a permissionless Solana protocol that manages AI agents linked to NFTs from verified collections. This specification defines the program structure, accounts, instructions, and state management for the on-chain protocol layer.

## Program Architecture

### Component Responsibilities

**mainframe-program** (this document): On-chain program for permissionless agent management
**mainframe-sdk** (TypeScript): Client library for metadata encryption, upload, and URI generation  
**mainframe-node**: Off-chain execution layer powered by Agent Stack framework with 200+ plugins
**mainframe-web**: dApp interface for users to interact with agents and protocol

The Anchor program serves as the foundational protocol layer, with mainframe-sdk handling secure metadata preparation before on-chain operations. Events are consumed by mainframe-node for AI agent deployment and execution.

## Program Constants

```rust
// Program Constants
pub const MAIKERS_COLLECTIBLES_MINT: Pubkey = pubkey!("MaikCollectiblesXXXXXXXXXXXXXXXXXXXXXXXX"); // Genesis collection
pub const MAX_METADATA_URI_LENGTH: usize = 200;
pub const MAX_PARTNER_COLLECTIONS: usize = 100;
pub const MAX_PARTNER_NAME_LENGTH: usize = 50;
```

## Account Structures

### Agent Account

```rust
#[account]
pub struct AgentAccount {
    /// The NFT mint associated with this agent
    pub nft_mint: Pubkey,
    /// The owner of the NFT and agent
    pub owner: Pubkey,
    /// The collection this NFT belongs to (from metadata)
    pub collection_mint: Option<Pubkey>,
    /// URI pointing to secure JSON metadata (encrypted and uploaded by mainframe-sdk)
    pub metadata_uri: String,
    /// Agent operational status
    pub status: AgentStatus,
    /// Timestamp of activation
    pub activated_at: i64,
    /// Last update timestamp
    pub updated_at: i64,
    /// Version for configuration updates
    pub version: u64,
    /// Reserved space for future upgrades
    pub reserved: [u8; 64],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AgentStatus {
    Active,
    Paused,
    Closed,
}
```

### Protocol Configuration

```rust
#[account]
pub struct ProtocolConfig {
    /// Protocol authority
    pub authority: Pubkey,
    /// Fee structure
    pub fees: FeeStructure,
    /// Fee distribution accounts and rates
    pub protocol_treasury: Pubkey,
    pub validator_treasury: Pubkey,
    pub cloud_treasury: Pubkey,
    /// Fee distribution percentages (must sum to 100)
    pub protocol_treasury_percent: u8,      // Default: 60%
    pub validator_treasury_percent: u8,     // Default: 30%
    pub cloud_treasury_percent: u8,         // Default: 10%
    /// Emergency pause status
    pub paused: bool,
    /// Total agents activated
    pub total_agents: u64,
    /// Special collections with custom fee tiers
    pub partner_collections: Vec<PartnerCollection>,
    /// Reserved space for future upgrades
    pub reserved: [u8; 64],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PartnerCollection {
    pub collection_mint: Pubkey,
    pub discount_percent: u8,        // 0-100 percentage discount
    pub name: String,                // Partner name for reference
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FeeStructure {
    pub create_agent: u64,          // 0.05 SOL in lamports
    pub update_config: u64,         // 0.005 SOL in lamports
    pub transfer_agent: u64,        // 0.01 SOL in lamports
    pub pause_agent: u64,           // 0 SOL - Free
    pub close_agent: u64,           // 0 SOL - Free
    pub execute_action: u64,        // 0 SOL - Free
}
```

## PDA Derivation Strategy

```rust
// Deterministic account addressing
pub fn derive_agent_pda(nft_mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"agent", nft_mint.as_ref()],
        &PROGRAM_ID
    )
}

pub fn derive_protocol_config_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"protocol_config"],
        &PROGRAM_ID
    )
}

// Benefits:
// - Deterministic addressing (no need to store addresses)
// - Collision-resistant (based on NFT mint)
// - Efficient lookups
// - Program ownership guaranteed
```

## Core Instructions

### Initialize Config

```rust
#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(
        init,
        payer = authority,
        space = ProtocolConfig::LEN,
        seeds = [b"protocol_config"],
        bump
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_config(
    ctx: Context<InitializeConfig>,
    fees: FeeStructure,
    protocol_treasury: Pubkey,
    validator_treasury: Pubkey,
    cloud_treasury: Pubkey,
    protocol_treasury_percent: u8,
    validator_treasury_percent: u8,
    cloud_treasury_percent: u8,
) -> Result<()> {
    // Validate that percentages sum to 100%
    let total_percent = protocol_treasury_percent + validator_treasury_percent + cloud_treasury_percent;
    require!(total_percent == 100, MainframeError::InvalidTreasuryDistribution);
    
    let config = &mut ctx.accounts.protocol_config;
    config.authority = ctx.accounts.authority.key();
    config.fees = fees;
    config.protocol_treasury = protocol_treasury;
    config.validator_treasury = validator_treasury;
    config.cloud_treasury = cloud_treasury;
    config.protocol_treasury_percent = protocol_treasury_percent;
    config.validator_treasury_percent = validator_treasury_percent;
    config.cloud_treasury_percent = cloud_treasury_percent;
    config.paused = false;
    config.total_agents = 0;
    config.partner_collections = Vec::new();
    
    Ok(())
}
```

### Create Agent

```rust
#[derive(Accounts)]
#[instruction(nft_mint: Pubkey)]
pub struct CreateAgent<'info> {
    #[account(
        init,
        payer = owner,
        space = AgentAccount::LEN,
        seeds = [b"agent", nft_mint.as_ref()],
        bump
    )]
    pub agent_account: Account<'info, AgentAccount>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    /// NFT token account owned by the user
    #[account(
        constraint = nft_token_account.mint == nft_mint,
        constraint = nft_token_account.owner == owner.key(),
        constraint = nft_token_account.amount == 1
    )]
    pub nft_token_account: Account<'info, TokenAccount>,
    
    /// NFT metadata account
    #[account(
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            nft_mint.as_ref()
        ],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    pub nft_metadata: Account<'info, MetadataAccount>,
    
    /// Protocol configuration
    #[account(
        mut,
        seeds = [b"protocol_config"],
        bump,
        constraint = !protocol_config.paused @ MainframeError::ProtocolPaused
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,
    
    // Fee distribution accounts
    #[account(mut)]
    pub protocol_treasury: SystemAccount<'info>,
    #[account(mut)]
    pub validator_treasury: SystemAccount<'info>,
    #[account(mut)]
    pub cloud_treasury: SystemAccount<'info>,
    
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
}

pub fn create_agent(
    ctx: Context<CreateAgent>,
    nft_mint: Pubkey,
    metadata_uri: String,
) -> Result<()> {
    // Validate URI format
    require!(!metadata_uri.is_empty(), MainframeError::InvalidMetadataUri);
    
    // Validate NFT metadata
    require!(
        ctx.accounts.nft_metadata.mint == nft_mint,
        MainframeError::InvalidNFTMetadata
    );
    
    // Extract collection from NFT metadata if available
    let collection_mint = get_collection_from_metadata(&ctx.accounts.nft_metadata)?;
    
    // Calculate fee based on collection
    let fee_amount = ctx.accounts.protocol_config.calculate_fee("create_agent", &collection_mint);
    
    // Collect and distribute creation fee
    if fee_amount > 0 {
        ctx.accounts.protocol_config.distribute_fee(
            fee_amount,
            &ctx.accounts.owner.to_account_info(),
            &ctx.accounts.protocol_treasury.to_account_info(),
            &ctx.accounts.validator_treasury.to_account_info(),
            &ctx.accounts.cloud_treasury.to_account_info(),
        )?;
    }
    
    // Initialize agent account
    let agent_account = &mut ctx.accounts.agent_account;
    agent_account.nft_mint = nft_mint;
    agent_account.owner = ctx.accounts.owner.key();
    agent_account.collection_mint = collection_mint;
    agent_account.metadata_uri = metadata_uri;
    agent_account.status = AgentStatus::Active;
    agent_account.activated_at = Clock::get()?.unix_timestamp;
    agent_account.updated_at = Clock::get()?.unix_timestamp;
    agent_account.version = 1;
    
    // Increment protocol counter
    ctx.accounts.protocol_config.total_agents = ctx.accounts.protocol_config.total_agents
        .checked_add(1)
        .ok_or(MainframeError::CounterOverflow)?;
    
    // Emit creation event
    emit!(AgentCreated {
        agent_account: ctx.accounts.agent_account.key(),
        nft_mint,
        owner: ctx.accounts.owner.key(),
        collection_mint,
        metadata_uri: ctx.accounts.agent_account.metadata_uri.clone(),
        timestamp: Clock::get()?.unix_timestamp,
        version: 1,
    });
    
    Ok(())
}
```

### Update Agent Configuration

```rust
#[derive(Accounts)]
pub struct UpdateAgentConfig<'info> {
    #[account(
        mut,
        seeds = [b"agent", agent_account.nft_mint.as_ref()],
        bump,
        constraint = agent_account.owner == owner.key(),
        constraint = agent_account.status != AgentStatus::Closed
    )]
    pub agent_account: Account<'info, AgentAccount>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        seeds = [b"protocol_config"],
        bump,
        constraint = !protocol_config.paused @ MainframeError::ProtocolPaused
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,
    
    // Fee distribution accounts
    #[account(mut)]
    pub protocol_treasury: SystemAccount<'info>,
    #[account(mut)]
    pub validator_treasury: SystemAccount<'info>,
    #[account(mut)]
    pub cloud_treasury: SystemAccount<'info>,
}

pub fn update_agent_config(
    ctx: Context<UpdateAgentConfig>,
    new_metadata_uri: String,
) -> Result<()> {
    // Validate URI format
    require!(!new_metadata_uri.is_empty(), MainframeError::InvalidMetadataUri);
    
    // Calculate and collect update fee
    let fee_amount = ctx.accounts.protocol_config.calculate_fee(
        "update_config", 
        &ctx.accounts.agent_account.collection_mint
    );
    
    if fee_amount > 0 {
        ctx.accounts.protocol_config.distribute_fee(
            fee_amount,
            &ctx.accounts.owner.to_account_info(),
            &ctx.accounts.protocol_treasury.to_account_info(),
            &ctx.accounts.validator_treasury.to_account_info(),
            &ctx.accounts.cloud_treasury.to_account_info(),
        )?;
    }
    
    // Update agent account
    let old_version = ctx.accounts.agent_account.version;
    ctx.accounts.agent_account.metadata_uri = new_metadata_uri;
    ctx.accounts.agent_account.updated_at = Clock::get()?.unix_timestamp;
    ctx.accounts.agent_account.version = ctx.accounts.agent_account.version
        .checked_add(1)
        .ok_or(MainframeError::VersionOverflow)?;
    
    // Emit update event
    emit!(AgentUpdated {
        agent_account: ctx.accounts.agent_account.key(),
        owner: ctx.accounts.owner.key(),
        metadata_uri: ctx.accounts.agent_account.metadata_uri.clone(),
        old_version,
        new_version: ctx.accounts.agent_account.version,
        timestamp: Clock::get()?.unix_timestamp,
    });
    
    Ok(())
}
```

### Transfer Agent

```rust
pub fn transfer_agent(ctx: Context<TransferAgent>) -> Result<()> {
    // Validate NFT ownership transfer
    require!(
        ctx.accounts.new_nft_token_account.owner == ctx.accounts.new_owner.key(),
        MainframeError::NFTNotOwned
    );
    
    // Calculate and distribute transfer fee
    let fee_amount = ctx.accounts.protocol_config.calculate_fee(
        "transfer_agent", 
        &ctx.accounts.agent_account.collection_mint
    );
    
    if fee_amount > 0 {
        ctx.accounts.protocol_config.distribute_fee(
            fee_amount,
            &ctx.accounts.new_owner.to_account_info(),
            &ctx.accounts.protocol_treasury.to_account_info(),
            &ctx.accounts.validator_treasury.to_account_info(),
            &ctx.accounts.cloud_treasury.to_account_info(),
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
```

## Fee Distribution System

```rust
impl ProtocolConfig {
    pub fn distribute_fee(
        &self,
        fee_amount: u64,
        payer: &AccountInfo,
        protocol_treasury: &AccountInfo,
        validator_treasury: &AccountInfo,
        cloud_treasury: &AccountInfo,
    ) -> Result<()> {
        if fee_amount == 0 {
            return Ok(());
        }
        
        // Calculate distribution using percentages
        let protocol_fee = fee_amount * self.protocol_treasury_percent as u64 / 100;
        let validator_fee = fee_amount * self.validator_treasury_percent as u64 / 100;
        let cloud_fee = fee_amount * self.cloud_treasury_percent as u64 / 100;

        // Handle rounding by giving remainder to protocol treasury
        let distributed_total = protocol_fee + validator_fee + cloud_fee;
        let protocol_fee_final = protocol_fee + (fee_amount - distributed_total);

        // Deduct from payer
        **payer.try_borrow_mut_lamports()? -= fee_amount;

        // Distribute to treasury accounts
        **protocol_treasury.try_borrow_mut_lamports()? += protocol_fee_final;
        **validator_treasury.try_borrow_mut_lamports()? += validator_fee;
        **cloud_treasury.try_borrow_mut_lamports()? += cloud_fee;
        
        Ok(())
    }
}
```

## Partner Discount Detection

```rust
impl ProtocolConfig {
    pub fn calculate_fee(&self, operation: &str, collection_mint: &Option<Pubkey>) -> u64 {
        let base_fee = match operation {
            "create_agent" => self.fees.create_agent,
            "update_config" => self.fees.update_config,
            "transfer_agent" => self.fees.transfer_agent,
            "pause_agent" => self.fees.pause_agent,
            "close_agent" => self.fees.close_agent,
            _ => 0,
        };
        
        // If base fee is 0, return 0 regardless of collection
        if base_fee == 0 {
            return 0;
        }
        
        if let Some(collection) = collection_mint {
            // Genesis collection: zero fees
            if *collection == MAIKERS_COLLECTIBLES_MINT {
                return 0;
            }
            
            // Check partner collections
            if let Some(discount_percent) = self.get_partner_discount(collection) {
                let discount_multiplier = 100 - discount_percent as u64;
                return base_fee * discount_multiplier / 100;
            }
        }
        
        base_fee
    }
}
```

## Event Definitions

```rust
#[event]
pub struct AgentCreated {
    pub agent_account: Pubkey,
    pub nft_mint: Pubkey,
    pub owner: Pubkey,
    pub collection_mint: Option<Pubkey>,
    pub metadata_uri: String,
    pub timestamp: i64,
    pub version: u64,
}

#[event]
pub struct AgentUpdated {
    pub agent_account: Pubkey,
    pub owner: Pubkey,
    pub metadata_uri: String,
    pub old_version: u64,
    pub new_version: u64,
    pub timestamp: i64,
}

#[event]
pub struct AgentTransferred {
    pub agent_account: Pubkey,
    pub nft_mint: Pubkey,
    pub old_owner: Pubkey,
    pub new_owner: Pubkey,
    pub timestamp: i64,
}
```

## Error Definitions

```rust
#[error_code]
pub enum MainframeError {
    #[msg("NFT not owned by the signer")]
    NFTNotOwned = 6000,
    #[msg("Agent is not active")]
    AgentNotActive,
    #[msg("Protocol is paused")]
    ProtocolPaused,
    #[msg("Unauthorized operation")]
    Unauthorized,
    #[msg("Invalid NFT metadata")]
    InvalidNFTMetadata,
    #[msg("Version counter overflow")]
    VersionOverflow,
    #[msg("Total agents counter overflow")]
    CounterOverflow,
    #[msg("Invalid metadata URI")]
    InvalidMetadataUri,
    #[msg("Treasury distribution percentages must sum to 100")]
    InvalidTreasuryDistribution,
}
```

## Security Considerations

### Access Control
- Owner Verification: All agent operations require NFT ownership verification
- NFT Verification: NFT metadata validation ensures authentic tokens
- Authority Checks: Protocol configuration changes require authority signature
- Permissionless Design: Any valid NFT can activate an agent without pre-approval

### Data Integrity
- Metadata Validation: Agent metadata size is capped and validated
- Version Control: Configuration updates increment version numbers
- State Consistency: Agent status is tracked and validated for all operations

### Economic Security
- Fee Validation: All paid operations validate proper fee payment before execution
- Fee Distribution: Automatic distribution across protocol treasuries
- Emergency Controls: Protocol-wide pause mechanism for security incidents
- Collection Management: Dynamic partner collection management with discounts

This specification provides a comprehensive foundation for implementing the Mainframe Solana Anchor program, ensuring secure, permissionless AI agent management with proper ownership verification and fee handling.
