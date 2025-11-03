# Mainframe: Program Economics & Fee Structure

## Overview

The Mainframe program implements a transparent, on-chain fee structure that supports protocol development, network security, and ecosystem growth. All fees are calculated and distributed automatically within program instructions.

## Program Fee Structure

### Base Operation Fees

| Operation | Base Fee (SOL) | When Charged |
|-----------|----------------|--------------|
| **create_agent** | 0.05 SOL | Agent creation |
| **update_agent_config** | 0.005 SOL | Configuration updates |
| **transfer_agent** | 0.01 SOL | Ownership transfers |
| **pause_agent** | 0 SOL | Always free |
| **close_agent** | 0 SOL | Always free |

### Fee Calculation Logic

```rust
impl ProtocolConfig {
    pub fn calculate_base_fee(&self, operation: &str) -> u64 {
        match operation {
            "create_agent" => self.fees.create_agent,           // 50,000,000 lamports
            "update_agent_config" => self.fees.update_agent_config, // 5,000,000 lamports  
            "transfer_agent" => self.fees.transfer_agent,       // 10,000,000 lamports
            "pause_agent" => self.fees.pause_agent,             // 0 - Always free
            "close_agent" => self.fees.close_agent,             // 0 - Always free
            "execute_action" => self.fees.execute_action,       // 0 - Always free
            _ => 0,
        }
    }
    
    pub fn apply_discount(base_fee: u64, discount_percent: u8) -> u64 {
        if discount_percent >= 100 {
            return 0;
        }
        let discount_multiplier = 100 - discount_percent as u64;
        base_fee * discount_multiplier / 100
    }
}

// Fee calculation with collection discounts is done in processors:
// 1. Get base fee from calculate_base_fee
// 2. Check if collection matches genesis_collection_mint (100% discount)
// 3. Look up PartnerCollectionAccount PDA by collection_mint
// 4. Apply discount_percent from partner account
```

## Collection-Based Fee Tiers

### Genesis Collection (maikers'collectibles)
- **Fee Discount**: 100% (All operations free)
- **Storage**: `genesis_collection_mint` field in ProtocolConfig
- **Validation**: Checked in create_agent processor for zero fees

### Partner Collections
- **Fee Discount**: Variable (25-75% off base fees)
- **Storage**: Separate PDA accounts (PartnerCollectionAccount)
- **Management**: Protocol authority can add/remove partners via instructions

```rust
#[account]
pub struct PartnerCollectionAccount {
    pub collection_mint: Pubkey,      // Collection identifier
    pub discount_percent: u8,         // 0-100 percentage discount
    pub name: String,                 // Partner name (max 50 chars)
    pub active: bool,                 // Active status flag
    pub added_at: i64,                // Timestamp when added
    pub bump: u8,                     // PDA bump seed
}

// PDA derivation:
// seeds = [b"partner", collection_mint.as_ref()]
```

### Standard Collections
- **Fee Discount**: None (full base fees apply)
- **Applies to**: All collections not in genesis or partner lists

## Automatic Fee Distribution

### Distribution Basis Points

The program automatically distributes collected fees across three treasury accounts using basis points (1 bps = 0.01%):

```rust
pub struct ProtocolConfig {
    pub protocol_treasury_bps: u16,         // Default: 5000 (50%)
    pub validator_treasury_bps: u16,        // Default: 3000 (30%)  
    pub network_treasury_bps: u16,          // Default: 2000 (20%)
    // Basis points must sum to 10,000 (100%)
}
```

**Note**: Affiliate commissions are paid separately before treasury distribution.

### Distribution Breakdown

**Standard Treasury Distribution:**
```
Agent Activation Fee: 0.05 SOL
├─ Affiliate Commission: Variable (0-50% based on tier, if applicable)
└─ Remaining Fee Distribution:
   ├─ Protocol Treasury: 50% (5000 bps)
   ├─ Validator Treasury: 30% (3000 bps)
   └─ Network Treasury: 20% (2000 bps)
```

**Example without Affiliate:**
```
Total Fee: 0.05 SOL (50,000,000 lamports)
├─ Protocol: 0.025 SOL (50%)
├─ Validators: 0.015 SOL (30%)
└─ Network: 0.010 SOL (20%)
```

**Example with Bronze Affiliate (15%), no referrer:**
```
Total Fee: 0.05 SOL (50,000,000 lamports)
├─ Bronze Affiliate: 0.0075 SOL (15%)
├─ Referrer: 0 SOL (no referrer)
└─ Remaining: 0.0425 SOL
   ├─ Protocol: 0.02125 SOL (50%)
   ├─ Validators: 0.01275 SOL (30%)
   └─ Network: 0.0085 SOL (20%)
```

**Example with Diamond Affiliate (50%), with referrer:**
```
Total Fee: 0.05 SOL (50,000,000 lamports)
├─ Diamond Affiliate: 0.025 SOL (50%)
├─ Referrer: 0.00125 SOL (5% of affiliate commission)
└─ Remaining: 0.02375 SOL
   ├─ Protocol: 0.011875 SOL (50%)
   ├─ Validators: 0.007125 SOL (30%)
   └─ Network: 0.004750 SOL (20%)
```

### Distribution Implementation

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
        if fee_amount == 0 {
            return Ok(());
        }
        
        // Validate basis points sum to 10,000 (100%)
        let total_bps = self.protocol_treasury_bps
            .checked_add(self.validator_treasury_bps)
            .and_then(|x| x.checked_add(self.network_treasury_bps))
            .ok_or(MainframeError::InvalidTreasuryDistribution)?;
        require!(total_bps == 10_000, MainframeError::InvalidTreasuryDistribution);
        
        // Calculate distribution using basis points (1 bps = 0.01%)
        let protocol_fee = fee_amount * self.protocol_treasury_bps as u64 / 10_000;
        let validator_fee = fee_amount * self.validator_treasury_bps as u64 / 10_000;
        let network_fee = fee_amount * self.network_treasury_bps as u64 / 10_000;
        
        // Handle rounding (remainder goes to protocol treasury)
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
    protocol_bps: u16,
    validator_bps: u16,
    network_bps: u16
) -> Result<()> {
    let total = protocol_bps
        .checked_add(validator_bps)
        .and_then(|x| x.checked_add(network_bps))
        .ok_or(MainframeError::InvalidTreasuryDistribution)?;
    require!(
        total == 10_000,
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
    protocol_bps: u16,
    validator_bps: u16,
    network_bps: u16,
) -> Result<()> {
    // Validate basis points sum to 10,000
    validate_treasury_distribution(protocol_bps, validator_bps, network_bps)?;
    
    let config = &mut ctx.accounts.protocol_config;
    config.protocol_treasury_bps = protocol_bps;
    config.validator_treasury_bps = validator_bps;
    config.network_treasury_bps = network_bps;
    
    Ok(())
}
```

## Affiliate Bonus Pool Economics

### Bonus Pool Funding

The affiliate bonus pool is continuously funded from protocol fees:

**Accumulation Rate:**
```
10% of all remaining fees (after affiliate commission) → Bonus Pool

Monthly Example (1,000 activations):
├─ Total fees: 50 SOL
├─ Average affiliate commission: 15 SOL (varies by tier)
├─ Remaining: 35 SOL
└─ Bonus pool: 3.5 SOL (10% of remaining)
```

### Affiliate Program Economics

The affiliate program operates separately from the treasury distribution:

**Commission Structure:**
- Affiliates earn 15-50% commission based on their tier
- Commission is paid directly from the activation fee
- Remaining fee (after affiliate commission) goes to treasury distribution

**Affiliate Tiers:**
- Bronze (0-99 sales): 15% commission (1500 bps)
- Silver (100-499 sales): 20% commission (2000 bps)
- Gold (500-1,999 sales): 30% commission (3000 bps)
- Platinum (2,000-9,999 sales): 40% commission (4000 bps)
- Diamond (10,000+ sales): 50% commission (5000 bps)

### Economic Sustainability

**Pool Balance Management:**
```
Monthly Inflow: 3.5 SOL (from fees)
Monthly Outflow:
├─ Milestones claimed: ~1.3 SOL
├─ Referral earnings: ~0.9 SOL
├─ Season prizes (quarterly): ~0.65 SOL/month
└─ Flash bonuses: ~0.45 SOL

Net: +0.2 SOL growth per month
Annual surplus: ~2.4 SOL
```

The 10% allocation ensures sustainable long-term funding for all affiliate incentives.

## Economic Incentives

### Fee Tier Comparison

For an agent creation (0.05 SOL base fee):

| Collection Type | Fee Amount | Discount | Annual Cost (5 agents) |
|----------------|------------|----------|------------------------|
| **Genesis** | 0 SOL | 100% | 0 SOL |
| **Strategic Partner** | 0.0125 SOL | 75% | 0.0625 SOL |
| **Verified Partner** | 0.025 SOL | 50% | 0.125 SOL |
| **Standard** | 0.05 SOL | 0% | 0.25 SOL |

### Affiliate Tier Impact

**For Affiliates (Standard Collection, 0.05 SOL fee):**

| Tier | Per Sale | 100 Sales | 1,000 Sales | 10,000 Sales |
|------|----------|-----------|-------------|--------------|
| Bronze (15%) | 0.0075 SOL | 0.75 SOL | 7.5 SOL | 75 SOL |
| Silver (20%) | 0.010 SOL | 1 SOL | 10 SOL | 100 SOL |
| Gold (30%) | 0.015 SOL | 1.5 SOL | 15 SOL | 150 SOL |
| Platinum (40%) | 0.020 SOL | 2 SOL | 20 SOL | 200 SOL |
| Diamond (50%) | 0.025 SOL | 2.5 SOL | 25 SOL | 250 SOL |

### Revenue Transparency

Query current protocol economics:

```rust
pub fn get_protocol_stats(ctx: Context<GetProtocolStats>) -> Result<ProtocolStats> {
    let config = &ctx.accounts.protocol_config;
    
    Ok(ProtocolStats {
        total_agents: config.total_agents,
        fee_structure: config.fees.clone(),
        treasury_distribution: TreasuryDistribution {
            protocol_bps: config.protocol_treasury_bps,
            validator_bps: config.validator_treasury_bps,
            network_bps: config.network_treasury_bps,
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