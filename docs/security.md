# Mainframe: Program Security Model

> **⚠️ Security Audit**: See [SECURITY_AUDIT_2025.md](SECURITY_AUDIT_2025.md) for details on vulnerabilities identified and fixed in November 2025.

## Security Overview

The Mainframe Anchor program implements comprehensive security controls to ensure safe agent management, secure fee handling, and proper access controls. The program follows Solana best practices and includes multiple layers of validation to protect against common attack vectors.

### Recent Security Improvements (November 2025)

All critical and high-severity vulnerabilities have been identified and fixed:
- ✅ Real-time NFT ownership verification (prevents stale ownership attacks)
- ✅ Comprehensive PDA validation (prevents fake account exploits)  
- ✅ Metaplex collection verification (validates genuine collection membership)
- ✅ Reentrancy protection via CEI pattern (prevents state inconsistency)
- ✅ Safe arithmetic everywhere (prevents overflow-based DOS)

See [SECURITY_AUDIT_2025.md](SECURITY_AUDIT_2025.md) for complete details.

## Core Security Principles

### Access Control Enforcement
All program operations validate proper ownership and authorization before execution, ensuring only authorized parties can manage agents and protocol settings.

### Economic Security
Fee calculation and distribution is handled atomically within transactions to prevent manipulation or partial execution states.

### Input Validation
All user inputs are validated for format, size, and safety constraints before processing to prevent injection attacks or data corruption.

### State Consistency
Program maintains consistent state across all operations with proper error handling and rollback mechanisms.

## Program Security Architecture

### Account Security Model

```rust
// Agent account security validation
#[derive(Accounts)]
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
    
    // NFT ownership validation
    #[account(
        constraint = nft_token_account.mint == nft_mint,
        constraint = nft_token_account.owner == owner.key(),
        constraint = nft_token_account.amount == 1
    )]
    pub nft_token_account: Account<'info, TokenAccount>,
    
    // Additional security constraints...
}
```

### Access Control Implementation

```rust
pub fn validate_agent_ownership(
    agent_account: &AgentAccount,
    signer: &Pubkey
) -> Result<()> {
    require!(
        agent_account.owner == *signer,
        MainframeError::Unauthorized
    );
    
    require!(
        agent_account.status != AgentStatus::Closed,
        MainframeError::AgentNotActive
    );
    
    Ok(())
}

pub fn validate_nft_ownership(
    nft_token_account: &TokenAccount,
    nft_mint: &Pubkey,
    owner: &Pubkey
) -> Result<()> {
    require!(
        nft_token_account.mint == *nft_mint,
        MainframeError::InvalidNFTMetadata
    );
    
    require!(
        nft_token_account.owner == *owner,
        MainframeError::NFTNotOwned
    );
    
    require!(
        nft_token_account.amount == 1,
        MainframeError::InvalidNFTBalance
    );
    
    Ok(())
}
```

## Fee Security System

### Secure Fee Distribution

```rust
impl ProtocolConfig {
    pub fn distribute_fee<'info>(
        &self,
        fee_amount: u64,
        payer: &AccountInfo<'info>,
        protocol_treasury: &AccountInfo<'info>,
        validator_treasury: &AccountInfo<'info>,
        network_treasury: &AccountInfo<'info>,
        system_program: &AccountInfo<'info>,
    ) -> Result<()> {
        // Validate sufficient balance before any transfers
        require!(
            payer.lamports() >= fee_amount,
            MainframeError::InsufficientBalance
        );
        
        // Validate treasury distribution basis points
        let total_bps = self.protocol_treasury_bps
            .checked_add(self.validator_treasury_bps)
            .and_then(|x| x.checked_add(self.network_treasury_bps))
            .ok_or(MainframeError::InvalidTreasuryDistribution)?;
        require!(
            total_bps == 10_000,
            MainframeError::InvalidTreasuryDistribution
        );
        
        // Calculate distribution using basis points (1 bps = 0.01%)
        let protocol_fee = fee_amount * self.protocol_treasury_bps as u64 / 10_000;
        let validator_fee = fee_amount * self.validator_treasury_bps as u64 / 10_000;
        let network_fee = fee_amount * self.network_treasury_bps as u64 / 10_000;
        
        // Handle rounding by giving remainder to protocol
        let remainder = fee_amount - (protocol_fee + validator_fee + network_fee);
        let protocol_fee_final = protocol_fee + remainder;
        
        // Transfer to respective treasury accounts using CPI
        transfer_lamports(payer, protocol_treasury, protocol_fee_final, system_program)?;
        transfer_lamports(payer, validator_treasury, validator_fee, system_program)?;
        transfer_lamports(payer, network_treasury, network_fee, system_program)?;
        
        Ok(())
    }
}
```

### Fee Manipulation Prevention

```rust
pub fn calculate_fee_with_validation(
    config: &ProtocolConfig,
    operation: &str,
    collection_mint: &Option<Pubkey>
) -> Result<u64> {
    // Get base fee for operation
    let base_fee = match operation {
        "create_agent" => config.fees.create_agent,
        "update_agent_config" => config.fees.update_agent_config,
        "transfer_agent" => config.fees.transfer_agent,
        _ => return Err(MainframeError::InvalidOperation.into()),
    };
    
    // Apply collection-based discounts securely
    if let Some(collection) = collection_mint {
        // Genesis collection verification
        if *collection == MAIKERS_COLLECTIBLES_MINT {
            return Ok(0);
        }
        
        // Partner collection discount validation
        for partner in &config.partner_collections {
            if partner.collection_mint == *collection {
                // Validate discount percentage is within bounds
                require!(
                    partner.discount_percent <= 100,
                    MainframeError::InvalidDiscountPercent
                );
                
                let discount_multiplier = 100 - partner.discount_percent as u64;
                return Ok(base_fee * discount_multiplier / 100);
            }
        }
    }
    
    Ok(base_fee)
}
```

## Input Validation & Sanitization

### Metadata URI Validation

```rust
pub fn validate_metadata_uri(uri: &str) -> Result<()> {
    // Check URI is not empty
    require!(!uri.is_empty(), MainframeError::InvalidMetadataUri);
    
    // Check URI length constraints
    require!(
        uri.len() <= MAX_METADATA_URI_LENGTH,
        MainframeError::MetadataUriTooLong
    );
    
    // Validate URI format (basic checks)
    require!(
        uri.starts_with("https://") || 
        uri.starts_with("ipfs://") || 
        uri.starts_with("ar://"),
        MainframeError::InvalidUriScheme
    );
    
    // Additional validation for malicious content
    require!(
        !uri.contains("javascript:") && 
        !uri.contains("data:") && 
        !uri.contains("<script"),
        MainframeError::MaliciousUriContent
    );
    
    Ok(())
}
```

### Numerical Overflow Protection

```rust
pub fn safe_increment_counter(current: u64) -> Result<u64> {
    current
        .checked_add(1)
        .ok_or(MainframeError::CounterOverflow.into())
}

pub fn safe_version_increment(current: u64) -> Result<u64> {
    current
        .checked_add(1)
        .ok_or(MainframeError::VersionOverflow.into())
}
```

## Access Control Matrix

### Operation Permissions

| Operation | NFT Owner Required | Agent Owner Required | Protocol Authority Required | Additional Checks |
|-----------|-------------------|---------------------|----------------------------|------------------|
| **create_agent** | ✅ | ❌ | ❌ | Valid NFT metadata |
| **update_agent_config** | ❌ | ✅ | ❌ | Agent not closed |
| **transfer_agent** | ✅  | ❌ | ❌ | Valid ownership transfer |
| **pause_agent** | ❌ | ✅ | ❌ | Agent currently active |
| **resume_agent** | ❌ | ✅ | ❌ | Agent currently paused |
| **close_agent** | ❌ | ✅ | ❌ | Irreversible operation |
| **update_fees** | ❌ | ❌ | ✅ | Valid fee structure |
| **pause_protocol** | ❌ | ❌ | ✅ | Emergency only |
| **add_partner** | ❌ | ❌ | ✅ | Valid collection mint |

### Authority Validation

```rust
pub fn validate_protocol_authority(
    config: &ProtocolConfig,
    signer: &Pubkey
) -> Result<()> {
    require!(
        config.authority == *signer,
        MainframeError::Unauthorized
    );
    Ok(())
}

pub fn validate_protocol_not_paused(config: &ProtocolConfig) -> Result<()> {
    require!(
        !config.paused,
        MainframeError::ProtocolPaused
    );
    Ok(())
}
```

## Attack Vector Mitigation

### Common Attack Patterns

| Attack Vector | Risk Level | Mitigation | Implementation |
|---------------|------------|------------|----------------|
| **Unauthorized Access** | High | NFT ownership verification | Token account validation |
| **Fee Manipulation** | High | Atomic fee distribution | Pre-flight balance checks |
| **Reentrancy** | Medium | State updates before external calls | Anchor framework protection |
| **Integer Overflow** | Medium | Safe arithmetic operations | Checked math operations |
| **Metadata Injection** | Medium | URI validation and sanitization | Input format validation |
| **Rent Exemption Bypass** | Low | Proper account sizing | Fixed account sizes |

### Reentrancy Protection

```rust
pub fn create_agent_with_protection(
    ctx: Context<CreateAgent>,
    nft_mint: Pubkey,
    metadata_uri: String,
) -> Result<()> {
    // 1. All validations first (no state changes)
    validate_nft_ownership(&ctx.accounts.nft_token_account, &nft_mint, &ctx.accounts.owner.key())?;
    validate_metadata_uri(&metadata_uri)?;
    
    // 2. Calculate fees (no state changes)
    let fee_amount = calculate_fee_with_validation(
        &ctx.accounts.protocol_config,
        "create_agent",
        &collection_mint
    )?;
    
    // 3. State updates (before any external operations)
    initialize_agent_account(&mut ctx.accounts.agent_account, nft_mint, metadata_uri)?;
    increment_total_agents(&mut ctx.accounts.protocol_config)?;
    
    // 4. Fee transfer (external operation last)
    if fee_amount > 0 {
        distribute_fee_securely(fee_amount, &ctx.accounts)?;
    }
    
    // 5. Event emission (informational only)
    emit_agent_created_event(&ctx.accounts.agent_account)?;
    
    Ok(())
}
```

### State Validation

```rust
pub fn validate_agent_state_transition(
    current_status: AgentStatus,
    new_status: AgentStatus
) -> Result<()> {
    match (current_status, new_status) {
        // Valid transitions
        (AgentStatus::Active, AgentStatus::Paused) => Ok(()),
        (AgentStatus::Paused, AgentStatus::Active) => Ok(()),
        (AgentStatus::Active, AgentStatus::Closed) => Ok(()),
        (AgentStatus::Paused, AgentStatus::Closed) => Ok(()),
        
        // Invalid transitions
        (AgentStatus::Closed, _) => Err(MainframeError::InvalidStateTransition.into()),
        (status, new_status) if status == new_status => Err(MainframeError::NoStateChange.into()),
        
        _ => Err(MainframeError::InvalidStateTransition.into())
    }
}
```

## Emergency Controls

### Protocol Pause Mechanism

```rust
pub fn emergency_pause(ctx: Context<EmergencyPause>, paused: bool) -> Result<()> {
    // Only protocol authority can pause
    validate_protocol_authority(&ctx.accounts.protocol_config, &ctx.accounts.authority.key())?;
    
    // Update pause state
    ctx.accounts.protocol_config.paused = paused;
    
    // Emit pause event
    emit!(ProtocolPauseChanged {
        authority: ctx.accounts.authority.key(),
        paused,
        timestamp: Clock::get()?.unix_timestamp,
    });
    
    Ok(())
}
```

### Authority Transfer Security

```rust
pub fn transfer_authority(
    ctx: Context<TransferAuthority>,
    new_authority: Pubkey
) -> Result<()> {
    // Current authority must sign
    validate_protocol_authority(&ctx.accounts.protocol_config, &ctx.accounts.current_authority.key())?;
    
    // Validate new authority is not zero address
    require!(
        new_authority != Pubkey::default(),
        MainframeError::InvalidAuthority
    );
    
    // Update authority
    let old_authority = ctx.accounts.protocol_config.authority;
    ctx.accounts.protocol_config.authority = new_authority;
    
    // Emit authority change event
    emit!(AuthorityTransferred {
        old_authority,
        new_authority,
        timestamp: Clock::get()?.unix_timestamp,
    });
    
    Ok(())
}
```

## Error Handling & Recovery

### Comprehensive Error Codes

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
    #[msg("Invalid metadata URI")]
    InvalidMetadataUri,
    #[msg("Metadata URI too long")]
    MetadataUriTooLong,
    #[msg("Invalid URI scheme")]
    InvalidUriScheme,
    #[msg("Malicious URI content detected")]
    MaliciousUriContent,
    #[msg("Insufficient balance for fee payment")]
    InsufficientBalance,
    #[msg("Treasury distribution percentages must sum to 100")]
    InvalidTreasuryDistribution,
    #[msg("Invalid discount percentage (must be 0-100)")]
    InvalidDiscountPercent,
    #[msg("Counter overflow detected")]
    CounterOverflow,
    #[msg("Version counter overflow")]
    VersionOverflow,
    #[msg("Invalid state transition")]
    InvalidStateTransition,
    #[msg("Collection already exists in partner list")]
    CollectionAlreadyExists,
    #[msg("Collection not found in partner list")]
    CollectionNotFound,
}
```

### Transaction Failure Handling

```rust
pub fn handle_transaction_failure(error: &ProgramError) -> Result<()> {
    match error {
        // Retryable errors
        ProgramError::Custom(6000) => {
            // NFT ownership issues - user should check wallet
            msg!("NFT ownership validation failed - check wallet connection");
        },
        ProgramError::Custom(6008) => {
            // Insufficient balance - user needs more SOL
            msg!("Insufficient balance for transaction fees");
        },
        
        // Non-retryable errors  
        ProgramError::Custom(6002) => {
            // Protocol paused - wait for resolution
            msg!("Protocol is paused - try again later");
        },
        ProgramError::Custom(6013) => {
            // Invalid state transition - agent may be closed
            msg!("Invalid operation - check agent status");
        },
        
        _ => {
            msg!("Unexpected error occurred - contact support");
        }
    }
    
    Err(error.clone())
}
```

## Security Best Practices

### For Program Interactions
- Always validate account ownership before state changes
- Use safe arithmetic operations to prevent overflows
- Validate all user inputs for format and safety
- Implement proper error handling with informative messages
- Test edge cases and error conditions thoroughly

### For Protocol Administration
- Use multi-signature wallets for protocol authority
- Test all configuration changes on devnet first
- Implement gradual rollout for protocol updates
- Monitor protocol health and usage metrics
- Maintain emergency response procedures

### For Integration Security
- Validate transaction parameters client-side before submission
- Handle transaction failures gracefully with retry logic
- Monitor account changes for unauthorized modifications
- Implement proper key management for automated systems
- Use connection pooling and rate limiting for RPC calls

This security model ensures the Mainframe program maintains the highest standards of safety and reliability while providing clear error handling and recovery mechanisms for all participants.