# Mainframe Scripts

## 🚨 DEPLOYMENT POLICY

**ALWAYS DEPLOY TO DEVNET FIRST, THEN MAINNET!**

```bash
# Use the deployment script (handles devnet → mainnet flow)
./scripts/deploy.sh

# Or manual deployment:
# 1. Devnet first
anchor build
solana program deploy target/deploy/mainframe.so --url devnet --program-id mnfm...
# Test on devnet
# 2. Then mainnet (only if devnet works)
solana program deploy target/deploy/mainframe.so --url mainnet-beta --program-id mnfm...
```

---

## Event Monitoring Scripts

### 1. Fetch Historical Events

Fetches and decodes past events from the program.

```bash
# Fetch last 100 events on mainnet
yarn ts-node scripts/fetch-events.ts

# Fetch events on devnet
yarn ts-node scripts/fetch-events.ts --network devnet

# Fetch specific event type
yarn ts-node scripts/fetch-events.ts --event AgentCreated

# Fetch with custom limit
yarn ts-node scripts/fetch-events.ts --limit 500

# Fetch events before a signature (pagination)
yarn ts-node scripts/fetch-events.ts --before <signature>
```

**Examples:**
```bash
# Get last 50 agent creations on mainnet
yarn ts-node scripts/fetch-events.ts --event AgentCreated --limit 50

# Get affiliate registrations on devnet
yarn ts-node scripts/fetch-events.ts --network devnet --event AffiliateRegistered

# Get all recent events
yarn ts-node scripts/fetch-events.ts --limit 500
```

**Output:**
```
🔍 Fetching events from mainnet...
📊 Limit: 100

📜 Fetching transaction signatures...
✅ Found 45 transactions

📊 Found 32 events

═══════════════════════════════════════════════════════

1. AgentCreated
   Signature: 5xorje...
   Slot: 380288640
   Time: 2024-11-15T18:00:00.000Z
   Data:
     Agent: 8FzQ...
     NFT Mint: pszD...
     Owner: 7ytz...
     Collection: mA1K...
     Version: 1

2. AffiliatePaid
   Signature: 3NATX...
   Slot: 380288650
   Time: 2024-11-15T18:00:10.000Z
   Data:
     Agent: 8FzQ...
     Seller: DPmf...
     Amount: 0.0075 SOL
     Rate: 1500 bps

═══════════════════════════════════════════════════════

✅ Total: 32 events

📈 Summary:
   AgentCreated: 25
   AffiliatePaid: 5
   AffiliateRegistered: 2
```

---

### 2. Monitor Real-Time Events

Streams events as they happen.

```bash
# Monitor mainnet events
./scripts/monitor-events.sh mainnet

# Monitor devnet events
./scripts/monitor-events.sh devnet
```

**Output:**
```
🔍 Monitoring Mainframe events on mainnet
📡 RPC: https://api.mainnet-beta.solana.com
📋 Program: mnfm211AwTDA8fGvPezYs3jjxAXgoucHGuTMUbjFssE

Watching for events... (Press Ctrl+C to stop)
═══════════════════════════════════════════════════════

Transaction executed in slot 380288640:
  Signature: 5xorjedvVyfZa4Xq2gucanENDCkVn32ccnqevB7BgeUTyrp95BWYipBzyuF5qgFF6KnryfYzj5dGbB7N2i6B7tZo
  Status: Ok
  Log Messages:
    Program mnfm211AwTDA8fGvPezYs3jjxAXgoucHGuTMUbjFssE invoke [1]
    Program logged: "Instruction: CreateAgent"
    Program logged: "✓ SPL Token NFT ownership validated"
    Program logged: "Event: AgentCreated"
    Program mnfm211AwTDA8fGvPezYs3jjxAXgoucHGuTMUbjFssE success
```

---

## Event Types

The Mainframe program emits these events:

### Agent Events
- **AgentCreated** - New agent created
- **AgentUpdated** - Agent configuration updated
- **AgentTransferred** - Agent ownership transferred
- **AgentPaused** - Agent paused
- **AgentResumed** - Agent resumed
- **AgentClosed** - Agent permanently closed
- **AgentAccountClosed** - Agent account closed (rent recovery)

### Affiliate Events
- **AffiliateRegistered** - New affiliate registered
- **AffiliatePaid** - Affiliate received commission
- **AffiliateBonusSet** - Custom bonus rate set
- **TierUpgraded** - Affiliate tier upgraded

### Protocol Events
- **TreasuryAddressesUpdated** - Treasury addresses changed

---

## Event Data Structures

### AgentCreated
```typescript
{
  agentAccount: PublicKey,
  nftMint: PublicKey,
  owner: PublicKey,
  collectionMint: PublicKey | null,
  metadataUri: string,
  seller: PublicKey | null,
  timestamp: i64,
  version: u64
}
```

### AffiliatePaid
```typescript
{
  agentAccount: PublicKey,
  seller: PublicKey,
  affiliateAmount: u64,      // lamports
  affiliateBps: u16,         // basis points
  timestamp: i64
}
```

### AffiliateRegistered
```typescript
{
  affiliate: PublicKey,
  referrer: PublicKey | null,
  timestamp: i64
}
```

### TierUpgraded
```typescript
{
  affiliate: PublicKey,
  oldTier: u8,
  newTier: u8,
  totalSales: u64,
  timestamp: i64
}
```

---

## Filtering Examples

### Get All Agent Creations
```bash
yarn ts-node scripts/fetch-events.ts --event AgentCreated --limit 1000
```

### Get Recent Affiliate Payments
```bash
yarn ts-node scripts/fetch-events.ts --event AffiliatePaid --limit 100
```

### Get Events from Specific Time Range
```bash
# Get last 500 events
yarn ts-node scripts/fetch-events.ts --limit 500

# Then get next 500 (pagination)
yarn ts-node scripts/fetch-events.ts --limit 500 --before <last_signature>
```

---

## Use Cases

### Analytics
- Track total agents created
- Monitor affiliate earnings
- Analyze fee revenue
- Track partner collection usage

### Debugging
- Verify events emitted correctly
- Check event data accuracy
- Monitor for anomalies

### Monitoring
- Real-time agent creation tracking
- Affiliate performance monitoring
- Protocol health checks

---

## Requirements

### Dependencies
```bash
cd maikers-mainframe
yarn install
```

### Environment Variables (Optional)
```bash
# Custom RPC endpoint
export SOLANA_RPC_URL="https://mainnet.helius-rpc.com/..."

# Or use default public RPCs
# Mainnet: https://api.mainnet-beta.solana.com
# Devnet: https://api.devnet.solana.com
```

---

## Troubleshooting

### "No events found"
- Program might not have any recent transactions
- Try increasing `--limit`
- Check if program ID is correct

### "RPC rate limit"
- Use a private RPC endpoint
- Reduce `--limit`
- Add delays between fetches

### "TypeScript errors"
- Run `anchor build` first to generate types
- Ensure `target/types/mainframe.ts` exists

---

## Advanced Usage

### Export to CSV
```bash
yarn ts-node scripts/fetch-events.ts --event AgentCreated | \
  grep "Agent:" | \
  awk '{print $2}' > agent_addresses.txt
```

### Count Events by Type
```bash
yarn ts-node scripts/fetch-events.ts --limit 1000 | \
  grep "Summary:" -A 20
```

### Find Specific Agent
```bash
yarn ts-node scripts/fetch-events.ts --event AgentCreated | \
  grep -A 5 "NFT Mint: pszD291..."
```
