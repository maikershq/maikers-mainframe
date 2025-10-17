# Mainframe: Program Quick Start Guide

## Introduction

This guide helps you get started with the Mainframe Solana program, whether you're a developer building applications or someone wanting to understand how the program works.

## Understanding the Program

### What is Mainframe?

Mainframe is a Solana Anchor program that manages AI agents linked to NFTs from verified collections. The program handles:

- **Agent Account Management**: Creating and maintaining agent-to-NFT relationships
- **Fee Calculation & Distribution**: Automated fee processing with collection-based tiers
- **Access Controls**: Ensuring only NFT owners can manage their agents
- **Event Emission**: Notifying external systems of agent lifecycle changes

### Program Structure

```
Mainframe Program
├── Agent Accounts (PDA per NFT)
├── Protocol Configuration (Global settings)
├── Instructions (Agent lifecycle operations)
└── Events (External system notifications)
```

### Core Operations

| Operation | Purpose | Fee | Access Required |
|-----------|---------|-----|-----------------|
| `create_agent` | Link NFT to agent | 0.05 SOL* | NFT ownership |
| `update_config` | Update agent settings | 0.005 SOL* | Agent ownership |
| `transfer_agent` | Transfer ownership | 0.01 SOL* | Both parties |
| `pause_agent` | Pause operations | FREE | Agent ownership |
| `close_agent` | Permanent shutdown | FREE | Agent ownership |

*Fees vary by collection tier (Genesis: free, Partners: discounted)

## Program Accounts

### Agent Account Structure

Each NFT can have one associated agent account:

```rust
pub struct AgentAccount {
    pub nft_mint: Pubkey,              // NFT this agent represents
    pub owner: Pubkey,                 // Current agent owner
    pub collection_mint: Option<Pubkey>, // For fee calculation
    pub metadata_uri: String,          // Off-chain configuration
    pub status: AgentStatus,           // Active/Paused/Closed
    pub activated_at: i64,             // Creation timestamp
    pub updated_at: i64,               // Last modification
    pub version: u64,                  // Configuration version
}
```

### Protocol Configuration Account

Global program settings:

```rust
pub struct ProtocolConfig {
    pub authority: Pubkey,             // Protocol admin
    pub fees: FeeStructure,            // Fee amounts
    pub protocol_treasury: Pubkey,     // Treasury wallets
    pub validator_treasury: Pubkey,
    pub cloud_treasury: Pubkey,
    pub treasury_percentages: [u8; 3], // Distribution: [60, 30, 10]
    pub paused: bool,                  // Emergency pause
    pub total_agents: u64,             // Global counter
    pub partner_collections: Vec<PartnerCollection>, // Fee discounts
}
```

## Getting Started as a Developer

### Prerequisites

- Solana CLI tools
- Anchor framework (0.31.1+)
- Node.js 18+
- Basic Solana/Anchor knowledge

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

### Program Interaction

#### Account Derivation

```typescript
import { PublicKey } from '@solana/web3.js';

// Derive agent PDA from NFT mint
function deriveAgentPDA(nftMint: PublicKey, programId: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('agent'), nftMint.toBuffer()],
    programId
  );
}

// Derive protocol config PDA  
function deriveProtocolConfigPDA(programId: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('protocol_config')],
    programId
  );
}
```

#### Reading Account Data

```typescript
import * as anchor from '@coral-xyz/anchor';

// Read agent account
async function getAgentAccount(
  program: anchor.Program,
  nftMint: PublicKey
): Promise<AgentAccount | null> {
  try {
    const [agentPDA] = deriveAgentPDA(nftMint, program.programId);
    return await program.account.agentAccount.fetch(agentPDA);
  } catch {
    return null; // Agent doesn't exist
  }
}

// Read protocol configuration
async function getProtocolConfig(program: anchor.Program): Promise<ProtocolConfig> {
  const [configPDA] = deriveProtocolConfigPDA(program.programId);
  return await program.account.protocolConfig.fetch(configPDA);
}
```

#### Building Transactions

```typescript
// Create agent transaction
async function buildCreateAgentTx(
  program: anchor.Program,
  nftMint: PublicKey,
  metadataUri: string,
  userWallet: PublicKey
): Promise<Transaction> {
  
  const [agentPDA] = deriveAgentPDA(nftMint, program.programId);
  const [configPDA] = deriveProtocolConfigPDA(program.programId);
  
  return await program.methods
    .createAgent(nftMint, metadataUri)
    .accounts({
      agentAccount: agentPDA,
      owner: userWallet,
      nftTokenAccount: await getAssociatedTokenAddress(nftMint, userWallet),
      nftMetadata: deriveMetadataAccount(nftMint),
      protocolConfig: configPDA,
      systemProgram: SystemProgram.programId,
    })
    .transaction();
}
```

## Program Events

### Event Types

The program emits events for external systems to consume:

```rust
#[event]
pub struct AgentCreated {
    pub agent_account: Pubkey,
    pub nft_mint: Pubkey, 
    pub owner: Pubkey,
    pub collection_mint: Option<Pubkey>,
    pub metadata_uri: String,
    pub timestamp: i64,
}

#[event]
pub struct AgentUpdated {
    pub agent_account: Pubkey,
    pub metadata_uri: String,
    pub old_version: u64,
    pub new_version: u64,
    pub timestamp: i64,
}
```

### Event Monitoring

```typescript
// Monitor program events
function subscribeToEvents(
  connection: Connection,
  programId: PublicKey,
  callback: (event: any) => void
): void {
  connection.onLogs(
    programId,
    (logs) => {
      // Parse Anchor events from logs
      logs.logs.forEach(log => {
        const match = log.match(/Program log: (\w+): (.+)/);
        if (match) {
          const [, eventName, eventData] = match;
          callback({ eventName, data: parseEventData(eventData) });
        }
      });
    },
    'processed'
  );
}
```

## Fee Calculation

### Understanding Fee Tiers

```typescript
// Calculate fee for operation
async function calculateFee(
  program: anchor.Program,
  operation: string,
  collectionMint?: PublicKey
): Promise<number> {
  const config = await getProtocolConfig(program);
  
  let baseFee = config.fees[operation] || 0;
  
  if (collectionMint) {
    // Check for genesis collection (free)
    if (collectionMint.equals(MAIKERS_COLLECTIBLES_MINT)) {
      return 0;
    }
    
    // Check for partner discount
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

## Testing

### Local Testing

```typescript
describe('Mainframe Program Tests', () => {
  let program: anchor.Program;
  let provider: anchor.Provider;
  
  before(async () => {
    provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    program = anchor.workspace.Mainframe;
  });

  it('Creates an agent account', async () => {
    const nftMint = new PublicKey("TestNFTMintAddress");
    const metadataUri = "https://example.com/metadata.json";
    
    const tx = await buildCreateAgentTx(
      program,
      nftMint, 
      metadataUri,
      provider.wallet.publicKey
    );
    
    await provider.sendAndConfirm(tx);
    
    // Verify agent was created
    const agent = await getAgentAccount(program, nftMint);
    expect(agent).toBeDefined();
    expect(agent.metadataUri).toBe(metadataUri);
  });
});
```

### Error Handling

```typescript
async function handleProgramError(error: any): Promise<void> {
  if (error.code === 6000) {
    throw new Error('NFT ownership verification failed');
  } else if (error.code === 6008) {
    throw new Error('Insufficient SOL balance for fees');
  } else if (error.code === 6002) {
    throw new Error('Protocol is paused');
  } else {
    throw new Error(`Program error: ${error.message}`);
  }
}
```

## Security Considerations

### Access Control Validation

Always validate account ownership and relationships:

```typescript
// Validate NFT ownership before operations
async function validateNFTOwnership(
  connection: Connection,
  nftMint: PublicKey,
  expectedOwner: PublicKey
): Promise<boolean> {
  const tokenAccount = await getAssociatedTokenAddress(nftMint, expectedOwner);
  const accountInfo = await connection.getTokenAccountBalance(tokenAccount);
  
  return accountInfo.value.uiAmount === 1;
}
```

### Transaction Safety

```typescript
// Safe transaction building with validation
async function safeCreateAgent(
  program: anchor.Program,
  nftMint: PublicKey,
  metadataUri: string,
  userWallet: PublicKey
): Promise<TransactionSignature> {
  
  // Pre-flight checks
  await validateNFTOwnership(program.provider.connection, nftMint, userWallet);
  
  const fee = await calculateFee(program, 'create_agent');
  const balance = await program.provider.connection.getBalance(userWallet);
  
  if (balance < fee) {
    throw new Error('Insufficient balance for transaction');
  }
  
  const tx = await buildCreateAgentTx(program, nftMint, metadataUri, userWallet);
  return await program.provider.sendAndConfirm(tx);
}
```

## Next Steps

### For Application Developers
1. Study the program account structures and instruction interfaces
2. Build transaction construction and error handling
3. Implement event monitoring for real-time updates
4. Test thoroughly on devnet before mainnet deployment

### For Integration Partners
1. Review the partner collection integration process
2. Understand fee calculation and distribution mechanisms
3. Plan event consumption architecture for your use case
4. Test integration patterns with the program

### For Protocol Contributors
1. Review the program security model and access controls
2. Understand the economic incentives and fee structures
3. Study the upgrade path and governance mechanisms
4. Contribute to testing and documentation improvements

This guide provides the foundation for working with the Mainframe program. For detailed technical specifications, see the complete documentation in the other docs files.