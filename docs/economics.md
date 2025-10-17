# Mainframe: Program Economics & Fee Structure

## Overview

The Mainframe program implements a transparent, on-chain fee structure that supports protocol development, network security, and ecosystem growth. All fees are calculated and distributed automatically within program instructions.

## Program Fee Structure

### Base Operation Fees

| Operation | Base Fee (Lamports) | SOL Equivalent | When Charged |
|-----------|-------------------|----------------|--------------|
| **create_agent** | 50,000,000 | 0.05 SOL | Agent creation |
| **update_config** | 5,000,000 | 0.005 SOL | Configuration updates |
| **transfer_agent** | 10,000,000 | 0.01 SOL | Ownership transfers |
| **pause_agent** | 0 | 0 SOL | Always free |
| **close_agent** | 0 | 0 SOL | Always free |

### Fee Calculation Logic

```rust
impl ProtocolConfig {
    pub fn calculate_fee(&self, operation: &str, collection_mint: &Option<Pubkey>) -> u64 {
        let base_fee = match operation {
            "create_agent" => self.fees.create_agent,     // 50,000,000 lamports
            "update_config" => self.fees.update_config,   // 5,000,000 lamports  
            "transfer_agent" => self.fees.transfer_agent, // 10,000,000 lamports
            "pause_agent" => 0,                          // Always free
            "close_agent" => 0,                          // Always free
            _ => 0,
        };
        
        // Apply collection-based discounts
        if let Some(collection) = collection_mint {
            // Genesis collection: zero fees
            if *collection == MAIKERS_COLLECTIBLES_MINT {
                return 0;
            }
            
            // Partner collection discounts
            for partner in &self.partner_collections {
                if partner.collection_mint == *collection {
                    let discount_multiplier = 100 - partner.discount_percent as u64;
                    return base_fee * discount_multiplier / 100;
                }
            }
        }
        
        base_fee
    }
}
```

## Collection-Based Fee Tiers

### Genesis Collection (maikers'collectibles)
- **Fee Discount**: 100% (All operations free)
- **Collection Mint**: `MAIKERS_COLLECTIBLES_MINT` constant in program
- **Validation**: Hardcoded in program for zero fees

### Partner Collections
- **Fee Discount**: Variable (25-75% off base fees)
- **Storage**: `partner_collections` Vec in ProtocolConfig account
- **Management**: Protocol authority can add/remove partners

```rust
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PartnerCollection {
    pub collection_mint: Pubkey,      // Collection identifier
    pub discount_percent: u8,         // 0-100 percentage discount
    pub name: String,                 // Reference name (max 50 chars)
}
```

### Standard Collections
- **Fee Discount**: None (full base fees apply)
- **Applies to**: All collections not in genesis or partner lists

## Automatic Fee Distribution

### Distribution Percentages

The program automatically distributes collected fees across three treasury accounts:

```rust
pub struct ProtocolConfig {
    pub protocol_treasury_percent: u8,    // Default: 60%
    pub validator_treasury_percent: u8,   // Default: 30%  
    pub cloud_treasury_percent: u8,       // Default: 10%
    // Percentages must sum to 100%
}
```

### Distribution Implementation

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
        
        // Calculate distribution amounts
        let protocol_fee = fee_amount * self.protocol_treasury_percent as u64 / 100;
        let validator_fee = fee_amount * self.validator_treasury_percent as u64 / 100;
        let cloud_fee = fee_amount * self.cloud_treasury_percent as u64 / 100;
        
        // Handle rounding (remainder goes to protocol treasury)
        let remainder = fee_amount - (protocol_fee + validator_fee + cloud_fee);
        let protocol_fee_final = protocol_fee + remainder;
        
        // Atomic distribution
        **payer.try_borrow_mut_lamports()? -= fee_amount;
        **protocol_treasury.try_borrow_mut_lamports()? += protocol_fee_final;
        **validator_treasury.try_borrow_mut_lamports()? += validator_fee;
        **cloud_treasury.try_borrow_mut_lamports()? += cloud_fee;
        
        Ok(())
    }
}
```

## Fee Validation & Security

### Pre-flight Validation

```rust
pub fn validate_fee_payment(
    payer_balance: u64,
    required_fee: u64
) -> Result<()> {
    require!(
        payer_balance >= required_fee,
        MainframeError::InsufficientBalance
    );
    Ok(())
}
```

### Treasury Validation

```rust
pub fn validate_treasury_distribution(
    protocol_percent: u8,
    validator_percent: u8,
    cloud_percent: u8
) -> Result<()> {
    let total = protocol_percent + validator_percent + cloud_percent;
    require!(
        total == 100,
        MainframeError::InvalidTreasuryDistribution
    );
    Ok(())
}
```

## Partner Management

### Adding Partner Collections

```rust
pub fn add_partner_collection(
    ctx: Context<AddPartnerCollection>,
    collection_mint: Pubkey,
    discount_percent: u8,
    name: String
) -> Result<()> {
    // Validate discount percentage
    require!(
        discount_percent <= 100,
        MainframeError::InvalidDiscountPercent
    );
    
    // Check collection doesn't already exist
    let exists = ctx.accounts.protocol_config.partner_collections
        .iter()
        .any(|partner| partner.collection_mint == collection_mint);
    
    require!(!exists, MainframeError::CollectionAlreadyExists);
    
    // Add new partner collection
    ctx.accounts.protocol_config.partner_collections.push(PartnerCollection {
        collection_mint,
        discount_percent,
        name,
    });
    
    Ok(())
}
```

### Removing Partner Collections

```rust
pub fn remove_partner_collection(
    ctx: Context<RemovePartnerCollection>,
    collection_mint: Pubkey
) -> Result<()> {
    let collections = &mut ctx.accounts.protocol_config.partner_collections;
    
    if let Some(index) = collections.iter().position(|p| p.collection_mint == collection_mint) {
        collections.remove(index);
        Ok(())
    } else {
        Err(MainframeError::CollectionNotFound.into())
    }
}
```

## Fee Structure Updates

### Protocol Authority Control

```rust
pub fn update_fees(
    ctx: Context<UpdateFees>,
    new_fees: FeeStructure
) -> Result<()> {
    // Only protocol authority can update fees
    require!(
        ctx.accounts.protocol_config.authority == ctx.accounts.authority.key(),
        MainframeError::Unauthorized
    );
    
    ctx.accounts.protocol_config.fees = new_fees;
    Ok(())
}

pub fn update_treasury_distribution(
    ctx: Context<UpdateTreasuryDistribution>,
    protocol_percent: u8,
    validator_percent: u8,
    cloud_percent: u8,
) -> Result<()> {
    // Validate percentages sum to 100
    validate_treasury_distribution(protocol_percent, validator_percent, cloud_percent)?;
    
    let config = &mut ctx.accounts.protocol_config;
    config.protocol_treasury_percent = protocol_percent;
    config.validator_treasury_percent = validator_percent;
    config.cloud_treasury_percent = cloud_percent;
    
    Ok(())
}
```

## Economic Incentives

### Fee Tier Comparison

For an agent creation (0.01 SOL base fee):

| Collection Type | Fee Amount | Discount | Annual Cost (5 agents) |
|----------------|------------|----------|------------------------|
| **Genesis** | 0 SOL | 100% | 0 SOL |
| **Strategic Partner** | 0.0025 SOL | 75% | 0.0125 SOL |
| **Verified Partner** | 0.005 SOL | 50% | 0.025 SOL |
| **Standard** | 0.01 SOL | 0% | 0.05 SOL |

### Revenue Transparency

Query current protocol economics:

```rust
pub fn get_protocol_stats(ctx: Context<GetProtocolStats>) -> Result<ProtocolStats> {
    let config = &ctx.accounts.protocol_config;
    
    Ok(ProtocolStats {
        total_agents: config.total_agents,
        fee_structure: config.fees.clone(),
        treasury_distribution: TreasuryDistribution {
            protocol_percent: config.protocol_treasury_percent,
            validator_percent: config.validator_treasury_percent,
            cloud_percent: config.cloud_treasury_percent,
        },
        partner_collections_count: config.partner_collections.len(),
        protocol_paused: config.paused,
    })
}
```

## Error Codes

```rust
#[error_code]
pub enum MainframeError {
    #[msg("Insufficient balance for fee payment")]
    InsufficientBalance = 6008,
    #[msg("Invalid discount percentage (must be 0-100)")]
    InvalidDiscountPercent = 6009,
    #[msg("Treasury distribution percentages must sum to 100")]
    InvalidTreasuryDistribution = 6010,
    #[msg("Collection already exists in partner list")]
    CollectionAlreadyExists = 6011,
    #[msg("Collection not found in partner list")]
    CollectionNotFound = 6012,
}
```

This economic model ensures the Mainframe program operates with transparent, fair fee calculation and automatic distribution while providing clear incentives for ecosystem participation through collection-based benefits.