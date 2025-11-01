# Mainframe: Quick Start Guide

## Introduction

Get started with the Mainframe Solana program for managing AI agents linked to verified NFT collections.

## Understanding Mainframe

**What it does:**
- Agent-to-NFT account management
- Automated fee calculation and distribution
- NFT ownership-based access control
- Event emission for external systems

**Core Operations:**

| Operation | Purpose | Fee | Access |
|-----------|---------|-----|--------|
| `create_agent` | Link NFT to agent | 0.05 SOL* | NFT owner |
| `update_agent_config` | Update settings | 0.005 SOL* | Agent owner |
| `transfer_agent` | Transfer ownership | 0.01 SOL* | Both parties |
| `pause_agent` | Pause operations | FREE | Agent owner |
| `close_agent` | Close permanently | FREE | Agent owner |

*Fees vary: Genesis (free), Partners (25-75% off)

## For Developers

### Prerequisites
- Solana CLI & Anchor framework (0.31.1+)
- Node.js 18+
- Basic Solana/Anchor knowledge

### Setup

```bash
git clone https://github.com/maikershq/maikers-mainframe
cd maikers-mainframe
yarn install
anchor build
anchor test
```

### Integration Options

**Option 1: Use SDK (Recommended)**
```bash
npm install @maikers/mainframe-sdk
```

The SDK handles encryption, metadata upload, and transaction building.

**Option 2: Direct Program Integration**

Interact with the program directly using Anchor or web3.js. See [Code References](references.md) for examples.

**See**: [Complete Code Examples](references.md)

## Key Concepts

### Program Accounts
- **Agent Account**: One PDA per NFT (derived from `["agent", nft_mint]`)
- **Protocol Config**: Global settings PDA (derived from `["protocol_config"]`)

### Fee Structure
- **Base fees**: 0.05 SOL (create), 0.005 SOL (update), 0.01 SOL (transfer)
- **Genesis collection**: 100% discount (free)
- **Partner collections**: 25-75% discount
- **Distribution**: Configurable basis points (default: 50% protocol, 30% validators, 20% network)

### Events
- `AgentCreated`: New agent activation
- `AgentUpdated`: Configuration changes
- `AffiliatePaid`: Commission payouts

Listen to events for off-chain processing and analytics.

## Common Workflows

### Creating an Agent

1. User owns NFT from verified collection
2. Prepare metadata (encrypted via SDK)
3. Call `create_agent` instruction
4. Pay fee (varies by collection)
5. Receive agent account
6. Event emitted for off-chain systems

### Updating Configuration

1. Agent owner calls `update_agent_config`
2. Provide new metadata URI
3. Pay update fee
4. Version incremented
5. Event emitted

### Transferring Ownership

1. NFT transferred on-chain first
2. New owner calls `transfer_agent` (one-sided operation)
3. New owner pays transfer fee
4. Agent ownership updated (no previous owner signature required)
5. Event emitted

**See**: [Code References](references.md)

## Testing

```bash
# Run all tests
anchor test

# Tests automatically clone required programs (Metaplex) from mainnet
```

Configuration is in `Anchor.toml` under the `[[test.validator.clone]]` section.

## Security

**Built-in protections:**
- NFT ownership verification
- Metadata validation
- Fee validation before execution
- Emergency pause mechanism

**See**: [Security Model](security.md)

## Fee Examples

### Standard Collection
- Create: 0.05 SOL
- Update: 0.005 SOL
- Transfer: 0.01 SOL

### Genesis Collection (maikers'collectibles)
- All operations: FREE

### Partner Collection (50% discount)
- Create: 0.025 SOL
- Update: 0.0025 SOL
- Transfer: 0.005 SOL

**See**: [Complete Economics](economics.md)

## Next Steps

**For App Developers:**
1. Review [Code References](references.md)
2. Check [SDK Documentation](https://github.com/maikershq/maikers-mainframe-sdk)
3. See [Quick Start Guide](quickstart.md)

**For Protocol Understanding:**
1. Read [Program Specifications](program-specs.md)
2. Review [Architecture](architecture.md)
3. Study [Economics & Fees](economics.md)

**For Partners:**
1. Contact team for partner collection setup
2. Review [Affiliate Program](affiliate.md) for revenue sharing

## Additional Resources

- **[References](references.md)** - Complete code examples
- **[Architecture](architecture.md)** - System design
- **[Program Specs](program-specs.md)** - Technical details
- **[Security](security.md)** - Security practices
- **[GitHub](https://github.com/maikershq/maikers-mainframe)** - Source code

## Support

- GitHub Issues: [maikers-mainframe/issues](https://github.com/maikers/mainframe/issues)
- Documentation: [docs.maikers.com](https://docs.maikers.com)
- Discord: [discord.gg/maikers](https://discord.gg/maikers)
