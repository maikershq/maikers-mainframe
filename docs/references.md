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
    seller_affiliate_bps: u16,
    collection_mint: Option<Pubkey>,
) -> Result<()>
```

### Fee Distribution Logic

```rust
// Calculate affiliate commission
let affiliate_fee = total_fee * seller_affiliate_bps / 10_000;
let remaining_fee = total_fee - affiliate_fee;

// Transfer to seller (supports 0 balance)
**seller.lamports() += affiliate_fee;

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

### Create Agent Context

```rust
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
    
    /// NFT token account owned by the user
    #[account(
        constraint = nft_token_account.mint == nft_mint,
        constraint = nft_token_account.owner == owner.key(),
        constraint = nft_token_account.amount == 1
    )]
    pub nft_token_account: Account<'info, TokenAccount>,
    
    /// NFT metadata account
    pub nft_metadata: Account<'info, MetadataAccount>,
    
    /// Protocol configuration
    #[account(
        mut,
        seeds = [b"protocol_config"],
        bump,
        constraint = !protocol_config.paused
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,
    
    // Fee distribution accounts
    #[account(mut)]
    pub protocol_treasury: SystemAccount<'info>,
    #[account(mut)]
    pub validator_treasury: SystemAccount<'info>,
    #[account(mut)]
    pub network_treasury: SystemAccount<'info>,
    
    /// Optional seller account that receives affiliate fee
    #[account(mut)]
    pub seller: Option<AccountInfo<'info>>,
    
    pub token_metadata_program: Program<'info, Metadata>,
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

## TypeScript Integration Examples

### PDA Derivation

```typescript
import { PublicKey } from '@solana/web3.js';

export function deriveAgentPDA(nftMint: PublicKey, programId: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('agent'), nftMint.toBuffer()],
    programId
  );
}

export function deriveProtocolConfigPDA(programId: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('protocol_config')],
    programId
  );
}
```

### Building Transactions

```typescript
import * as anchor from '@coral-xyz/anchor';

async function buildCreateAgentTransaction(
  program: anchor.Program,
  nftMint: PublicKey,
  metadataUri: string,
  userWallet: PublicKey
): Promise<Transaction> {
  const [agentPDA] = deriveAgentPDA(nftMint, program.programId);
  const [protocolConfigPDA] = deriveProtocolConfigPDA(program.programId);
  
  const nftTokenAccount = await getAssociatedTokenAddress(nftMint, userWallet);
  
  const [metadataAccount] = PublicKey.findProgramAddressSync(
    [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(), nftMint.toBuffer()],
    TOKEN_METADATA_PROGRAM_ID
  );
  
  return await program.methods
    .createAgent(nftMint, metadataUri)
    .accounts({
      agentAccount: agentPDA,
      owner: userWallet,
      nftTokenAccount,
      nftMetadata: metadataAccount,
      protocolConfig: protocolConfigPDA,
      systemProgram: SystemProgram.programId,
    })
    .transaction();
}
```

### Reading Account Data

```typescript
interface AgentAccount {
  nftMint: PublicKey;
  owner: PublicKey;
  collectionMint?: PublicKey;
  metadataUri: string;
  status: AgentStatus;
  activatedAt: number;
  updatedAt: number;
  version: number;
}

enum AgentStatus {
  Active = 0,
  Paused = 1,
  Closed = 2
}

async function getAgentAccount(
  program: anchor.Program,
  nftMint: PublicKey
): Promise<AgentAccount | null> {
  try {
    const [agentPDA] = deriveAgentPDA(nftMint, program.programId);
    const account = await program.account.agentAccount.fetch(agentPDA);
    return account as AgentAccount;
  } catch (error) {
    return null;
  }
}
```

### Event Monitoring

```typescript
class ProgramEventMonitor {
  private connection: Connection;
  private programId: PublicKey;
  private listeners: Map<string, (event: any) => void> = new Map();

  constructor(connection: Connection, programId: PublicKey) {
    this.connection = connection;
    this.programId = programId;
  }

  async subscribeToEvents(): Promise<void> {
    this.connection.onLogs(
      this.programId,
      (logs) => this.handleProgramLogs(logs),
      'processed'
    );
  }

  private handleProgramLogs(logs: Logs): void {
    for (const log of logs.logs) {
      const eventMatch = log.match(/Program log: (\w+): (.+)/);
      if (eventMatch) {
        const [, eventName, eventData] = eventMatch;
        this.emitEvent(eventName, eventData);
      }
    }
  }

  onAgentCreated(callback: (event: any) => void): void {
    this.listeners.set('AgentCreated', callback);
  }
}
```

### Fee Calculation

```typescript
async function calculateAgentCreationFee(
  program: anchor.Program,
  collectionMint?: PublicKey
): Promise<number> {
  const config = await getProtocolConfig(program);
  let baseFee = config.fees.createAgent;
  
  if (collectionMint) {
    if (collectionMint.equals(MAIKERS_COLLECTIBLES_MINT)) {
      return 0;
    }
    
    const partner = config.partnerCollections.find(
      p => p.collectionMint.equals(collectionMint)
    );
    
    if (partner) {
      const discountMultiplier = (100 - partner.discountPercent) / 100;
      baseFee = Math.floor(baseFee * discountMultiplier);
    }
  }
  
  return baseFee;
}
```

### Error Handling

```typescript
async function handleTransactionError(error: any): Promise<void> {
  if (error.code === 6000) {
    throw new Error('NFT ownership verification failed. Check your wallet.');
  } else if (error.code === 6008) {
    throw new Error('Insufficient SOL balance for transaction fees.');
  } else if (error.code === 6002) {
    throw new Error('Protocol is temporarily paused. Try again later.');
  } else {
    throw new Error(`Transaction failed: ${error.message}`);
  }
}
```

---

## Additional Resources

- **[Mainframe SDK](https://github.com/maikershq/maikers-mainframe-sdk)** - TypeScript SDK
- **[Program Source](https://github.com/maikershq/maikers-mainframe)** - Anchor program code
- **[Architecture Guide](architecture.md)** - System design
- **[Economics & Fees](economics.md)** - Fee structure
- **[Security Model](security.md)** - Security practices

