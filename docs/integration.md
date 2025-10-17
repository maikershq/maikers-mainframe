# Mainframe: Program Integration Guide

## Overview

This guide shows developers how to interact with the Mainframe Solana program, build transactions, monitor events, and integrate agent functionality into their applications.

## Integration Approaches

### Direct Program Integration
Interact directly with the Solana program using Anchor client libraries for maximum control and customization.

### SDK Integration  
Use the Mainframe SDK for simplified integration with built-in encryption, storage, and transaction management.

### Event-Based Integration
Monitor program events to build reactive applications that respond to agent lifecycle changes.

### Data Integration
Query program accounts for analytics, monitoring, and reporting applications.

## Direct Program Integration

### Prerequisites
- Anchor framework knowledge
- Solana/web3.js experience
- Understanding of PDAs and program accounts

### Program Interface

#### Core Instructions
```typescript
// Program instruction interface
interface MainframeProgram {
  // Agent lifecycle management
  createAgent(nftMint: PublicKey, metadataUri: string): Promise<TransactionSignature>;
  updateConfig(agentAccount: PublicKey, newMetadataUri: string): Promise<TransactionSignature>;
  transferAgent(agentAccount: PublicKey, newOwner: PublicKey): Promise<TransactionSignature>;
  pauseAgent(agentAccount: PublicKey): Promise<TransactionSignature>;
  closeAgent(agentAccount: PublicKey): Promise<TransactionSignature>;
  
  // Protocol administration (authority only)
  updateFees(newFees: FeeStructure): Promise<TransactionSignature>;
  addPartnerCollection(collection: PublicKey, discount: number): Promise<TransactionSignature>;
  emergencyPause(paused: boolean): Promise<TransactionSignature>;
}
```

#### Account Derivation
```typescript
import { PublicKey } from '@solana/web3.js';

// Derive agent account PDA
export function deriveAgentPDA(nftMint: PublicKey, programId: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('agent'), nftMint.toBuffer()],
    programId
  );
}

// Derive protocol config PDA
export function deriveProtocolConfigPDA(programId: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('protocol_config')],
    programId
  );
}
```

### Building Transactions

#### Create Agent Transaction
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
  
  // Get NFT token account
  const nftTokenAccount = await getAssociatedTokenAddress(nftMint, userWallet);
  
  // Get NFT metadata account
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
      // Treasury accounts would be loaded from protocol config
      systemProgram: SystemProgram.programId,
    })
    .transaction();
}
```

#### Update Agent Configuration
```typescript
async function buildUpdateConfigTransaction(
  program: anchor.Program,
  agentAccount: PublicKey,
  newMetadataUri: string,
  userWallet: PublicKey
): Promise<Transaction> {
  
  const [protocolConfigPDA] = deriveProtocolConfigPDA(program.programId);
  
  return await program.methods
    .updateConfig(newMetadataUri)
    .accounts({
      agentAccount,
      owner: userWallet,
      protocolConfig: protocolConfigPDA,
      // Treasury accounts for fee collection
      systemProgram: SystemProgram.programId,
    })
    .transaction();
}
```

### Account Data Reading

#### Reading Agent Account
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
    // Agent doesn't exist
    return null;
  }
}
```

#### Reading Protocol Configuration
```typescript
interface ProtocolConfig {
  authority: PublicKey;
  fees: FeeStructure;
  protocolTreasury: PublicKey;
  validatorTreasury: PublicKey;
  cloudTreasury: PublicKey;
  treasuryPercentages: {
    protocol: number;
    validator: number;
    cloud: number;
  };
  paused: boolean;
  totalAgents: number;
  partnerCollections: PartnerCollection[];
}

async function getProtocolConfig(program: anchor.Program): Promise<ProtocolConfig> {
  const [configPDA] = deriveProtocolConfigPDA(program.programId);
  return await program.account.protocolConfig.fetch(configPDA);
}
```

## Event Monitoring

### Program Event Types
```typescript
interface ProgramEvents {
  AgentCreated: {
    agentAccount: PublicKey;
    nftMint: PublicKey;
    owner: PublicKey;
    collectionMint?: PublicKey;
    metadataUri: string;
    timestamp: number;
    version: number;
  };
  
  AgentUpdated: {
    agentAccount: PublicKey;
    owner: PublicKey;
    metadataUri: string;
    oldVersion: number;
    newVersion: number;
    timestamp: number;
  };
  
  AgentTransferred: {
    agentAccount: PublicKey;
    nftMint: PublicKey;
    oldOwner: PublicKey;
    newOwner: PublicKey;
    timestamp: number;
  };
}
```

### Event Subscription
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
    // Subscribe to program logs
    this.connection.onLogs(
      this.programId,
      (logs) => this.handleProgramLogs(logs),
      'processed'
    );
  }

  private handleProgramLogs(logs: Logs): void {
    for (const log of logs.logs) {
      // Parse Anchor events from logs
      const eventMatch = log.match(/Program log: (\w+): (.+)/);
      if (eventMatch) {
        const [, eventName, eventData] = eventMatch;
        this.emitEvent(eventName, eventData);
      }
    }
  }

  onAgentCreated(callback: (event: ProgramEvents['AgentCreated']) => void): void {
    this.listeners.set('AgentCreated', callback);
  }

  onAgentUpdated(callback: (event: ProgramEvents['AgentUpdated']) => void): void {
    this.listeners.set('AgentUpdated', callback);
  }
}
```

### Real-time Account Monitoring
```typescript
class AgentAccountMonitor {
  private connection: Connection;
  private subscriptions: Map<string, number> = new Map();

  constructor(connection: Connection) {
    this.connection = connection;
  }

  subscribeToAgent(
    agentAccount: PublicKey,
    callback: (account: AgentAccount) => void
  ): () => void {
    const subscriptionId = this.connection.onAccountChange(
      agentAccount,
      (accountInfo) => {
        if (accountInfo.data) {
          // Decode account data using Anchor
          const decoded = decodeAgentAccount(accountInfo.data);
          callback(decoded);
        }
      },
      'processed'
    );

    this.subscriptions.set(agentAccount.toBase58(), subscriptionId);

    // Return unsubscribe function
    return () => {
      const id = this.subscriptions.get(agentAccount.toBase58());
      if (id !== undefined) {
        this.connection.removeAccountChangeListener(id);
        this.subscriptions.delete(agentAccount.toBase58());
      }
    };
  }
}
```

## SDK Integration

**For simplified integration with built-in encryption, storage, and transaction management:**

📦 **[Mainframe SDK Repository](https://github.com/maikershq/maikers-mainframe-sdk)**

### SDK Features
- **TypeScript Support**: Full type safety and IntelliSense
- **Built-in Encryption**: Automatic configuration encryption/decryption
- **Storage Abstraction**: IPFS and cloud storage support
- **Transaction Management**: Simplified transaction building and error handling
- **Event Monitoring**: Real-time agent lifecycle tracking

### Documentation & Examples
- **[Installation Guide](https://github.com/maikershq/maikers-mainframe-sdk#installation)**
- **[API Reference](https://github.com/maikershq/maikers-mainframe-sdk/blob/main/docs/api-reference.md)**
- **[Usage Examples](https://github.com/maikershq/maikers-mainframe-sdk/tree/main/examples)**
- **[Configuration Schema](https://github.com/maikershq/maikers-mainframe-sdk/blob/main/docs/agent-config.md)**

## Integration Patterns

### Agent Deployment Workflow
1. **Account Creation**: Create agent account on Solana using program instruction
2. **Event Emission**: Program emits AgentCreated event with metadata URI
3. **Event Consumption**: Off-chain systems monitor for events
4. **Metadata Retrieval**: Fetch encrypted metadata from storage URI
5. **Agent Deployment**: Deploy agent runtime with decrypted configuration

### Fee Calculation Integration
```typescript
async function calculateAgentCreationFee(
  program: anchor.Program,
  collectionMint?: PublicKey
): Promise<number> {
  const config = await getProtocolConfig(program);
  
  let baseFee = config.fees.createAgent;
  
  if (collectionMint) {
    // Check for genesis collection
    if (collectionMint.equals(MAIKERS_COLLECTIBLES_MINT)) {
      return 0; // Free for genesis collection
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

### Batch Operations
```typescript
async function batchCreateAgents(
  program: anchor.Program,
  requests: Array<{nftMint: PublicKey, metadataUri: string}>,
  userWallet: PublicKey
): Promise<TransactionSignature[]> {
  
  const transactions = await Promise.all(
    requests.map(req => 
      buildCreateAgentTransaction(program, req.nftMint, req.metadataUri, userWallet)
    )
  );
  
  // Send transactions in parallel
  const signatures = await Promise.allSettled(
    transactions.map(tx => 
      program.provider.connection.sendTransaction(tx, [userWallet])
    )
  );
  
  return signatures
    .filter(result => result.status === 'fulfilled')
    .map(result => (result as PromiseFulfilledResult<string>).value);
}
```

## Error Handling

### Transaction Error Recovery
```typescript
async function handleTransactionError(error: any): Promise<void> {
  if (error.code === 6000) {
    // NFT not owned - user needs to check wallet
    throw new Error('NFT ownership verification failed. Check your wallet.');
  } else if (error.code === 6008) {
    // Insufficient balance
    throw new Error('Insufficient SOL balance for transaction fees.');
  } else if (error.code === 6002) {
    // Protocol paused
    throw new Error('Protocol is temporarily paused. Try again later.');
  } else {
    // Generic error
    throw new Error(`Transaction failed: ${error.message}`);
  }
}
```

### Account State Validation
```typescript
async function validateAgentState(
  program: anchor.Program,
  nftMint: PublicKey
): Promise<AgentValidationResult> {
  const agent = await getAgentAccount(program, nftMint);
  
  if (!agent) {
    return { valid: false, error: 'Agent does not exist for this NFT' };
  }
  
  if (agent.status === AgentStatus.Closed) {
    return { valid: false, error: 'Agent is permanently closed' };
  }
  
  return { valid: true, agent };
}
```

## Testing Integration

### Local Development Setup
```typescript
// Test program deployment
const program = anchor.workspace.Mainframe as Program<Mainframe>;

// Create test agent
const testNftMint = new PublicKey("TestNFTMintAddress");
const testMetadataUri = "https://example.com/test-metadata.json";

describe('Mainframe Integration Tests', () => {
  it('should create agent successfully', async () => {
    const tx = await buildCreateAgentTransaction(
      program,
      testNftMint,
      testMetadataUri,
      provider.wallet.publicKey
    );
    
    const signature = await program.provider.connection.sendTransaction(tx);
    await program.provider.connection.confirmTransaction(signature);
    
    // Verify agent was created
    const agent = await getAgentAccount(program, testNftMint);
    expect(agent).toBeDefined();
    expect(agent.metadataUri).toBe(testMetadataUri);
  });
});
```

This integration guide provides the essential patterns for building applications that interact with the Mainframe program while maintaining focus on the core protocol functionality.