# Mainframe Code References

Technical references for Mainframe Anchor program - Rust and TypeScript code examples, account structures, and CLI commands.

## Table of Contents

- [Rust Program Examples](#rust-program-examples)
- [Account Structures](#account-structures)
- [Events](#events)
- [CLI Commands](#cli-commands)
- [Testing](#testing)
- [TypeScript Integration Examples](#typescript-integration-examples)

**For SDK Integration**: See [@maikers/mainframe-sdk](https://github.com/maikershq/maikers-mainframe-sdk)

---

## Rust Program Examples

### Create Agent Instruction

```rust
pub fn create_agent(
    ctx: Context<CreateAgent>,
    nft_mint: Pubkey,
    metadata_uri: String,
    collection_mint: Option<Pubkey>,
) -> Result<()>
```

**Affiliate System**: Commission is calculated automatically from the affiliate's tier (based on total_sales) + bonus_bps. Affiliate accounts are passed as optional accounts in the context, not as instruction parameters.

### Register Affiliate Instruction

```rust
pub fn register_affiliate(
    ctx: Context<RegisterAffiliate>,
    referrer: Option<Pubkey>,
) -> Result<()>
```

Explicitly registers an affiliate account with optional referrer. Accounts auto-initialize on first commission if not registered, so this is optional.

### Set Affiliate Bonus Instruction

```rust
pub fn set_affiliate_bonus(
    ctx: Context<SetAffiliateBonus>,
    bonus_bps: u16,
) -> Result<()>
```

Authority or manager can set custom bonus rate for specific affiliates (special deals, promotions).

### Fee Distribution Logic

```rust
// Calculate affiliate commission from tier + bonus (auto-calculated)
let commission_bps = affiliate_account.get_commission_bps(protocol_config.max_affiliate_bps);
let affiliate_fee = total_fee * commission_bps / 10_000;

// Calculate referrer commission (5% of affiliate commission)
let referrer_fee = affiliate_fee * 500 / 10_000;  // 5%

// Remaining fee after affiliate and referrer
let remaining_fee = total_fee - affiliate_fee - referrer_fee;

// Distribute remaining to treasuries
protocol_fee = remaining_fee * protocol_bps / 10_000;
validator_fee = remaining_fee * validator_bps / 10_000;
network_fee = remaining_fee * network_bps / 10_000;
```

### Fee Calculation

```rust
impl ProtocolConfig {
    pub fn calculate_fee(&self, operation: &str, collection_mint: &Option<Pubkey>) -> u64 {
        let base_fee = match operation {
            "create_agent" => self.fees.create_agent,     // 50,000,000 lamports
            "update_agent_config" => self.fees.update_agent_config,   // 5,000,000 lamports  
            "transfer_agent" => self.fees.transfer_agent, // 10,000,000 lamports
            "pause_agent" => 0,
            "close_agent" => 0,
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

### Fee Distribution Implementation

```rust
impl ProtocolConfig {
    pub fn distribute_fee(
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

### PDA Derivation

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
```

---

## Account Structures

### Agent Account

```rust
#[account]
pub struct AgentAccount {
    pub nft_mint: Pubkey,
    pub owner: Pubkey,
    pub collection_mint: Option<Pubkey>,
    pub metadata_uri: String,
    pub status: AgentStatus,
    pub activated_at: i64,
    pub updated_at: i64,
    pub version: u64,
    pub reserved: [u8; 32],
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
    pub authority: Pubkey,
    pub manager: Pubkey,
    pub genesis_collection_mint: Pubkey,
    pub fees: FeeStructure,
    pub protocol_treasury: Pubkey,
    pub validator_treasury: Pubkey,
    pub network_treasury: Pubkey,
    pub protocol_treasury_bps: u16,
    pub validator_treasury_bps: u16,
    pub network_treasury_bps: u16,
    pub paused: bool,
    pub total_agents: u64,
    pub total_partners: u64,
    pub max_partner_collections: u64,
    pub max_affiliate_bps: u16,
    pub pending_authority: Option<Pubkey>,
    pub reserved: [u8; 20],
}

// Partner collections stored in separate PDAs (PartnerCollectionAccount)
```

### Fee Structure

```rust
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FeeStructure {
    pub create_agent: u64,          // 0.05 SOL in lamports
    pub update_agent_config: u64,   // 0.005 SOL in lamports
    pub transfer_agent: u64,        // 0.01 SOL in lamports
    pub pause_agent: u64,           // 0 SOL - Free
    pub close_agent: u64,           // 0 SOL - Free
    pub execute_action: u64,        // 0 SOL - Free
}
```

### Partner Collection

```rust
#[account]
pub struct PartnerCollectionAccount {
    pub collection_mint: Pubkey,
    pub discount_percent: u8,        // 0-100 percentage discount
    pub name: String,                // Partner name (max 50 chars)
    pub active: bool,
    pub added_at: i64,
    pub bump: u8,
}

// PDA derivation: seeds = [b"partner", collection_mint.as_ref()]
```

### Affiliate Account

```rust
#[account]
pub struct AffiliateAccount {
    pub affiliate: Pubkey,           // Affiliate wallet address
    pub total_sales: u64,            // Total agent sales (determines tier)
    pub total_revenue: u64,          // Total commission earned (lamports)
    pub referral_count: u64,         // Number of direct referrals
    pub referree_sales: u64,         // Sales made by referrals
    pub referree_revenue: u64,       // Revenue from referral commissions
    pub referrer: Option<Pubkey>,    // Who referred this affiliate
    pub created_at: i64,             // Registration timestamp
    pub bonus_bps: u16,              // Custom bonus rate (authority-set)
    pub bump: u8,                    // PDA bump seed
}

// PDA derivation: seeds = [b"affiliate", affiliate.as_ref()]

// Tier calculation based on total_sales:
// Bronze: 0-99, Silver: 100-499, Gold: 500-1,999, Platinum: 2,000-9,999, Diamond: 10,000+
```

### Create Agent Context

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
    
    /// NFT token account - validates ownership
    #[account(
        constraint = nft_token_account.mint == nft_mint,
        constraint = nft_token_account.owner == owner.key(),
        constraint = nft_token_account.amount == 1
    )]
    pub nft_token_account: InterfaceAccount<'info, TokenAccount>,
    
    /// Protocol configuration
    #[account(
        mut,
        seeds = [b"protocol_config"],
        bump,
        constraint = !protocol_config.paused @ MainframeError::ProtocolPaused
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,
    
    /// Fee distribution accounts
    #[account(mut, constraint = protocol_treasury.key() == protocol_config.protocol_treasury)]
    /// CHECK: Validated by protocol_config constraint
    pub protocol_treasury: AccountInfo<'info>,
    
    #[account(mut, constraint = validator_treasury.key() == protocol_config.validator_treasury)]
    /// CHECK: Validated by protocol_config constraint
    pub validator_treasury: AccountInfo<'info>,
    
    #[account(mut, constraint = network_treasury.key() == protocol_config.network_treasury)]
    /// CHECK: Validated by protocol_config constraint
    pub network_treasury: AccountInfo<'info>,
    
    /// Optional affiliate wallet (receives commission)
    #[account(mut)]
    /// CHECK: Any valid wallet can be an affiliate
    pub affiliate: Option<AccountInfo<'info>>,
    
    /// Optional affiliate account PDA (auto-initialized if needed)
    #[account(mut)]
    /// CHECK: PDA validated in processor [b"affiliate", affiliate]
    pub affiliate_account: Option<AccountInfo<'info>>,
    
    /// Optional referrer wallet (receives 5% if affiliate has referrer)
    #[account(mut)]
    /// CHECK: Any valid wallet can receive referrer commission
    pub referrer: Option<AccountInfo<'info>>,
    
    /// Optional referrer's affiliate account PDA
    #[account(mut)]
    /// CHECK: PDA validated in processor [b"affiliate", referrer]
    pub referrer_account: Option<AccountInfo<'info>>,
    
    /// Optional partner collection account for discount validation
    /// CHECK: PDA validated in processor [b"partner", collection_mint]
    pub partner_account: Option<Account<'info, PartnerCollectionAccount>>,
    
    /// Optional Metaplex metadata account for collection verification
    /// CHECK: Validated in processor when collection_mint provided
    pub nft_metadata: Option<AccountInfo<'info>>,
    
    pub system_program: Program<'info, System>,
}
```

---

## Events

### AgentCreated Event

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
```

### AffiliatePaid Event

```rust
#[event]
pub struct AffiliatePaid {
    pub agent_account: Pubkey,
    pub seller: Pubkey,
    pub affiliate_amount: u64,
    pub affiliate_bps: u16,
    pub timestamp: i64,
}
```

### AgentUpdated Event

```rust
#[event]
pub struct AgentUpdated {
    pub agent_account: Pubkey,
    pub owner: Pubkey,
    pub metadata_uri: String,
    pub old_version: u64,
    pub new_version: u64,
    pub timestamp: i64,
}
```

### AgentTransferred Event

```rust
#[event]
pub struct AgentTransferred {
    pub agent_account: Pubkey,
    pub nft_mint: Pubkey,
    pub old_owner: Pubkey,
    pub new_owner: Pubkey,
    pub timestamp: i64,
}
```

### AffiliateRegistered Event

```rust
#[event]
pub struct AffiliateRegistered {
    pub affiliate: Pubkey,
    pub referrer: Option<Pubkey>,
    pub timestamp: i64,
}
```

### TierUpgraded Event

```rust
#[event]
pub struct TierUpgraded {
    pub affiliate: Pubkey,
    pub old_tier: u8,              // 0=Bronze, 1=Silver, 2=Gold, 3=Platinum, 4=Diamond
    pub new_tier: u8,
    pub total_sales: u64,
    pub timestamp: i64,
}
```

### AffiliateBonusSet Event

```rust
#[event]
pub struct AffiliateBonusSet {
    pub affiliate: Pubkey,
    pub bonus_bps: u16,
    pub set_by: Pubkey,
    pub timestamp: i64,
}
```

---

## CLI Commands

### Development Setup

```bash
# Clone the repository
git clone https://github.com/maikershq/maikers-mainframe
cd maikers-mainframe

# Install dependencies  
yarn install

# Build the program
anchor build

# Start local validator (optional)
solana-test-validator

# Run tests
anchor test
```

### Security Verification

```bash
# Check security info in binary
yarn check:security

# Check deployed program
yarn check:security <PROGRAM_ID>
```

### Verified Builds

```bash
# Build with verification
yarn build:verified

# Full verification pipeline
yarn verify
```

---

## Testing

### Running Tests

```bash
# Run all tests
anchor test

# Run with local validator
anchor test --skip-local-validator
```

### Test Structure

Tests automatically clone required programs (Metaplex Token Metadata) from mainnet for local testing. Configuration is in `Anchor.toml` under the `[[test.validator.clone]]` section.

---

## SDK Integration

For TypeScript/JavaScript integration, use the official Mainframe SDK:

**[@maikers/mainframe-sdk](https://github.com/maikershq/maikers-mainframe-sdk)** - Complete TypeScript SDK with:
- Transaction building and signing
- PDA derivation and account fetching
- Event monitoring
- Fee calculation
- Error handling
- Encryption utilities
- Storage integration

See the SDK repository for complete integration examples and documentation.

---

## Additional Resources

- **[Mainframe SDK](https://github.com/maikershq/maikers-mainframe-sdk)** - TypeScript SDK
- **[Program Source](https://github.com/maikershq/maikers-mainframe)** - Anchor program code
- **[Architecture Guide](architecture.md)** - System design
- **[Economics & Fees](economics.md)** - Fee structure
- **[Security Model](security.md)** - Security practices

