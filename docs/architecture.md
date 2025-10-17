# Mainframe: Program Architecture

## Overview

Mainframe is a permissionless Solana Anchor program that manages AI agents linked to NFTs from verified collections. The program handles agent lifecycle management, fee distribution, and access controls while emitting events that external systems can consume for off-chain agent deployment.

## Program Structure

### Core Components

```mermaid
graph TB
    subgraph "Mainframe Program"
        Instructions[Program Instructions]
        Accounts[Program Accounts]
        Events[Program Events]
        Security[Access Controls]
    end
    
    subgraph "External Interfaces"
        Clients[Client Applications]
        Storage[Metadata Storage]
        Consumers[Event Consumers]
    end
    
    Clients --> Instructions
    Instructions --> Accounts
    Instructions --> Events
    Instructions --> Security
    Events --> Consumers
    Accounts --> Storage
```

### Account Architecture

#### Agent Account Structure

```rust
#[account]
pub struct AgentAccount {
    pub nft_mint: Pubkey,              // Associated NFT (32 bytes)
    pub owner: Pubkey,                 // Current owner (32 bytes)
    pub collection_mint: Option<Pubkey>, // Collection for fee calculation (33 bytes)
    pub metadata_uri: String,          // Off-chain metadata URI (4 + 200 bytes)
    pub status: AgentStatus,           // Active/Paused/Closed (1 byte)
    pub activated_at: i64,             // Creation timestamp (8 bytes)
    pub updated_at: i64,               // Last modification (8 bytes)
    pub version: u64,                  // Configuration version (8 bytes)
    pub reserved: [u8; 64],            // Future upgrades (64 bytes)
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AgentStatus {
    Active,    // Agent is operational
    Paused,    // Temporarily halted
    Closed,    // Permanently shut down
}
```

#### Protocol Configuration Account

```rust
#[account]
pub struct ProtocolConfig {
    pub authority: Pubkey,                   // Protocol admin (32 bytes)
    pub fees: FeeStructure,                  // Fee amounts (48 bytes)
    pub protocol_treasury: Pubkey,           // Protocol treasury wallet (32 bytes)
    pub validator_treasury: Pubkey,          // Validator treasury wallet (32 bytes)
    pub cloud_treasury: Pubkey,              // Operations treasury wallet (32 bytes)
    pub protocol_treasury_percent: u8,       // Default: 60% (1 byte)
    pub validator_treasury_percent: u8,      // Default: 30% (1 byte)
    pub cloud_treasury_percent: u8,          // Default: 10% (1 byte)
    pub paused: bool,                        // Emergency pause (1 byte)
    pub total_agents: u64,                   // Total agents created (8 bytes)
    pub partner_collections: Vec<PartnerCollection>, // Fee discounts (variable)
    pub reserved: [u8; 64],                  // Future upgrades (64 bytes)
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FeeStructure {
    pub create_agent: u64,          // 0.01 SOL in lamports
    pub update_config: u64,         // 0.0025 SOL in lamports
    pub transfer_agent: u64,        // 0.001 SOL in lamports
    pub pause_agent: u64,           // 0 SOL - Free
    pub close_agent: u64,           // 0 SOL - Free
    pub execute_action: u64,        // 0 SOL - Free
}
```

### PDA Derivation Strategy

```rust
// Agent account PDA (unique per NFT)
pub fn derive_agent_pda(nft_mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"agent", nft_mint.as_ref()],
        &PROGRAM_ID
    )
}

// Protocol configuration PDA (singleton)
pub fn derive_protocol_config_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"protocol_config"],
        &PROGRAM_ID
    )
}
```

**Benefits:**
- **Collision-Resistant**: Based on NFT mint ensures uniqueness
- **Deterministic**: No need to store addresses off-chain
- **Efficient Lookups**: O(1) address generation
- **Program Ownership**: Guaranteed program authority over accounts

## Instruction Flow

### Agent Lifecycle Instructions

```mermaid
graph TD
    A[create_agent] --> B[update_config]
    B --> C[transfer_agent]
    A --> D[pause_agent]
    D --> E[resume_agent]
    B --> F[close_agent]
    
    style A fill:#2ecc71,color:#fff
    style F fill:#e74c3c,color:#fff
```

### Transaction Flow

```mermaid
sequenceDiagram
    participant Client
    participant Program
    participant Accounts
    participant Events
    
    Client->>Program: create_agent(nft_mint, metadata_uri)
    Program->>Program: Validate NFT ownership
    Program->>Program: Calculate fee based on collection
    Program->>Program: Collect and distribute fees
    Program->>Accounts: Initialize agent PDA
    Program->>Events: Emit AgentCreated event
    Program-->>Client: Transaction success
```

### Core Instructions

#### Create Agent
```rust
pub fn create_agent(
    ctx: Context<CreateAgent>,
    nft_mint: Pubkey,
    metadata_uri: String,
) -> Result<()> {
    // 1. Validate NFT ownership and metadata
    // 2. Calculate fee based on collection tier
    // 3. Collect and distribute creation fee
    // 4. Initialize agent account with provided data
    // 5. Emit AgentCreated event for off-chain consumption
}
```

#### Update Configuration
```rust
pub fn update_config(
    ctx: Context<UpdateConfig>,
    new_metadata_uri: String,
) -> Result<()> {
    // 1. Verify caller is agent owner
    // 2. Collect update fee (if applicable)
    // 3. Update metadata URI and increment version
    // 4. Emit AgentUpdated event
}
```

## Fee Distribution System

### Fee Calculation Logic

```rust
impl ProtocolConfig {
    pub fn calculate_fee(&self, operation: &str, collection: &Option<Pubkey>) -> u64 {
        let base_fee = match operation {
            "create_agent" => self.fees.create_agent,
            "update_config" => self.fees.update_config,
            "transfer_agent" => self.fees.transfer_agent,
            _ => 0,
        };
        
        if let Some(collection_mint) = collection {
            // Genesis collection: zero fees
            if *collection_mint == MAIKERS_COLLECTIBLES_MINT {
                return 0;
            }
            
            // Partner collection discounts
            if let Some(discount) = self.get_partner_discount(collection_mint) {
                return base_fee * (100 - discount) / 100;
            }
        }
        
        base_fee
    }
}
```

### Automatic Distribution

```rust
pub fn distribute_fee(
    fee_amount: u64,
    protocol_percent: u8,
    validator_percent: u8,
    cloud_percent: u8,
) -> Result<()> {
    // Validate percentages sum to 100
    require!(protocol_percent + validator_percent + cloud_percent == 100);
    
    // Calculate distribution
    let protocol_fee = fee_amount * protocol_percent as u64 / 100;
    let validator_fee = fee_amount * validator_percent as u64 / 100;
    let cloud_fee = fee_amount * cloud_percent as u64 / 100;
    
    // Handle remainder (goes to protocol treasury)
    let remainder = fee_amount - (protocol_fee + validator_fee + cloud_fee);
    
    // Transfer to respective treasury accounts
    transfer_lamports(payer, protocol_treasury, protocol_fee + remainder);
    transfer_lamports(payer, validator_treasury, validator_fee);
    transfer_lamports(payer, cloud_treasury, cloud_fee);
}
```

## Event System

### Program Events

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

### Event Consumption Pattern

External systems can monitor program events to:
- Deploy agents when `AgentCreated` is emitted
- Update agent configurations when `AgentUpdated` is emitted
- Handle ownership changes when `AgentTransferred` is emitted
- Track agent lifecycle for analytics and reporting

## Security Model

### Access Control Matrix

| Operation | NFT Owner | Protocol Authority | Anyone |
|-----------|-----------|-------------------|--------|
| Create Agent | ✅ (must own NFT) | ❌ | ❌ |
| Update Config | ✅ (must own agent) | ❌ | ❌ |
| Transfer Agent | ✅ (both parties) | ❌ | ❌ |
| Pause/Resume | ✅ (must own agent) | ❌ | ❌ |
| Close Agent | ✅ (must own agent) | ❌ | ❌ |
| Update Fees | ❌ | ✅ | ❌ |
| Emergency Pause | ❌ | ✅ | ❌ |
| Add Partners | ❌ | ✅ | ❌ |

### Validation Checks

```rust
// NFT ownership validation
require!(
    nft_token_account.owner == signer.key() && 
    nft_token_account.amount == 1,
    MainframeError::NFTNotOwned
);

// Agent ownership validation  
require!(
    agent_account.owner == signer.key(),
    MainframeError::Unauthorized
);

// Protocol not paused
require!(
    !protocol_config.paused,
    MainframeError::ProtocolPaused
);
```

## Performance Characteristics

### Transaction Metrics

| Operation | Compute Units | Account Size | Constraints |
|-----------|---------------|--------------|-------------|
| Create Agent | ~15,000 CU | 398 bytes | Must own NFT |
| Update Config | ~8,000 CU | No change | Must own agent |
| Transfer Agent | ~12,000 CU | No change | Both parties sign |
| Pause/Resume | ~5,000 CU | No change | Owner only |
| Close Agent | ~6,000 CU | Status change | Irreversible |

### Scaling Considerations

**Account Design:**
- Fixed-size agent accounts (398 bytes) for predictable rent costs
- Efficient PDA derivation with minimal compute overhead
- Reserved space for future upgrades without account migrations

**Compute Optimization:**
- Early validation failures to save compute units
- Efficient fee calculations using integer arithmetic
- Minimal stack usage (well under 4KB limit)

## Error Handling

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
    #[msg("Invalid metadata URI")]
    InvalidMetadataUri,
    #[msg("Treasury percentages must sum to 100")]
    InvalidTreasuryDistribution,
    #[msg("Version counter overflow")]
    VersionOverflow,
}
```

## External Integration Points

### Client Applications
- Build transactions using SDK or direct program calls
- Monitor account changes for real-time agent status
- Handle transaction confirmation and error states
- Parse program events for application logic

### Agent Runtime Systems  
- Subscribe to `AgentCreated` and `AgentUpdated` events
- Fetch metadata from URIs provided in events
- Deploy and manage agent instances based on program state
- Sync agent status with on-chain state

### Analytics & Monitoring
- Track protocol metrics from program events and accounts
- Monitor fee collection and treasury distribution
- Generate usage reports and performance analytics
- Implement alerting for protocol health monitoring

This program architecture ensures the Mainframe protocol remains focused on its core responsibility of managing agent-to-NFT relationships while providing clear, secure interfaces for external systems to build upon.