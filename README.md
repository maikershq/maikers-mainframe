# Mainframe

**Your NFT is probably worthless right now.**

> We know. You know. The market knows.

Mainframe transforms NFTs into an autonomous AI agent.

[![Anchor](https://img.shields.io/badge/Anchor-0.31.1-purple)](https://coral-xyz.github.io/anchor/)
[![Solana](https://img.shields.io/badge/Solana-Compatible-green)](https://solana.com/)
[![Security](https://img.shields.io/badge/Security-Audited-brightgreen)](#security)
[![Security.txt](https://img.shields.io/badge/Security.txt-Verified-success)](#security-txt)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue)](LICENSE)

## Documentation

### Getting Started
- **[Quick Start Guide](docs/quickstart.md)** - Get up and running quickly
- **[Affiliate Program](docs/affiliate.md)** - Revenue sharing and referral system
- **[System Architecture](docs/architecture.md)** - Technical system overview
- **[Code References](docs/references.md)** - Integration examples and code samples

### Technical Reference
- **[Program Specifications](docs/program-specs.md)** - Anchor program documentation
- **[🔐 Security Audit 2025](docs/SECURITY_AUDIT_2025.md)**
- **[Economics & Fees](docs/economics.md)** - Fee structure and distribution

## Overview

Mainframe is a Solana Anchor program that manages agent accounts linked to NFTs from verified collections. The program handles NFT ownership validation, fee collection and distribution, and emits events for the **[Mainframe Nodes](https://github.com/maikershq/maikers-mainframe-node)**.

**Core Capabilities:**
- Agent lifecycle management (create, update, transfer, pause, close)
- PDA-based agent accounts with deterministic derivation
- Tiered fee structure with collection-based discounts
- Automatic fee distribution across treasuries
- Affiliate revenue sharing system (15-50% commission)
- Event emission for off-chain consumption

See [Program Specifications](docs/program-specs.md) for detailed instruction and account documentation.

## Fee Structure

| Operation | Standard | Genesis | Partners |
|-----------|----------|---------|----------|
| Create Agent | 0.05 SOL | FREE | 25-75% off |
| Update Config | 0.005 SOL | FREE | 25-75% off |
| Transfer Agent | 0.01 SOL | FREE | 25-75% off |
| Pause/Resume/Close | FREE | FREE | FREE |

**Distribution**: Configurable via basis points (default: 50% Protocol • 30% Validators • 20% Network)

See [Economics & Fees](docs/economics.md) for complete fee calculation logic.

## 🎯 Advanced Affiliate Incentive System

**Permissionless participation - anyone can earn by referring users!**

### ✨ Key Features
- ✅ **Permissionless** - No pre-registration required, just provide wallet address
- ✅ **Auto-Initialize** - First commission automatically creates your affiliate account
- ✅ **Tier-Based Commission** - Earn 15-50% based on performance
- ✅ **Streak Bonuses** - Up to +15% for consistent sales
- ✅ **Milestone Rewards** - Bonuses from 0.1 SOL to 1000 SOL
- ✅ **Multi-Level Referrals** - Earn from your referrals' sales
- ✅ **Competition Seasons** - Win from prize pools
- ✅ **Achievement NFTs** - Earn badges for milestones
- ✅ **Instant Payouts** - Commission paid on-chain in real-time

### 💰 Commission Tiers

| Tier | Sales Threshold | Commission Rate | + Streak Bonus |
|------|----------------|-----------------|----------------|
| 🥉 Bronze | 0-99 | **15%** | up to +15% |
| 🥈 Silver | 100-499 | **20%** | up to +15% |
| 🥇 Gold | 500-1,999 | **30%** | up to +15% |
| 💎 Platinum | 2,000-9,999 | **40%** | up to +15% |
| 💎💎 Diamond | 10,000+ | **50%** | up to +15% |

### 🎁 Milestone Bonuses

| Sales Milestone | Bonus Reward |
|-----------------|--------------|
| 10 sales | 0.1 SOL |
| 50 sales | 1 SOL |
| 100 sales | 5 SOL |
| 500 sales | 50 SOL |
| 1,000 sales | 150 SOL |
| 5,000 sales | 1,000 SOL |

### 🔗 Multi-Level Referrals

- **Level 1:** Earn 10% of your direct referrals' commissions
- **Level 2:** Earn 5% of second-level referrals' commissions
- Build passive income streams by growing your network!

### 📊 Revenue Examples (Bronze → Diamond)

| Monthly Sales | Bronze (15%) | Silver (20%) | Gold (30%) | Platinum (40%) | Diamond (50%) |
|--------------|-------------|-------------|-----------|---------------|---------------|
| 100 agents | 0.75 SOL | 1 SOL | 1.5 SOL | 2 SOL | **2.5 SOL** |
| 500 agents | 3.75 SOL | 5 SOL | 7.5 SOL | 10 SOL | **12.5 SOL** |
| 1,000 agents | 7.5 SOL | 10 SOL | 15 SOL | 20 SOL | **25 SOL** |
| 5,000 agents | 37.5 SOL | 50 SOL | 75 SOL | 100 SOL | **125 SOL** |

*Based on 0.05 SOL standard activation fee

### 🚀 Getting Started

**It's dead simple:**
1. Share your referral link: `https://mainframe.maikers.com?ref=WALLET_ADDRESS`
2. Users create agents with your address
3. Get paid commission instantly
4. Your affiliate account auto-initializes
5. Climb tiers automatically as you sell

**Optional:** Register a vanity referral code for marketing

```typescript
// Users just include your wallet
await sdk.createAgent(nftMint, config, {
  affiliate: "YourWallet..."  // That's it!
});
```

**Ready to start earning?** See [Affiliate Documentation](docs/affiliate.md) for complete details.

## Quick Start

### For SDK Integration

**Use the official Mainframe SDK for simplified integration:**

📦 **[Mainframe SDK](https://github.com/maikershq/maikers-mainframe-sdk)** - TypeScript SDK with built-in encryption, storage, and transaction management

### For Direct Program Integration

**Documentation:**
- [Quick Start Guide](docs/quickstart.md) - Complete setup and usage
- [Code References](docs/references.md) - Integration examples and patterns
- [Affiliate Program](docs/affiliate.md) - Revenue sharing implementation


## Development

**Prerequisites**: Anchor 0.31.1 • Solana CLI 1.18.26+ • Node.js 18+ • Rust

```bash
# Clone and build
git clone https://github.com/maikershq/maikers-mainframe
cd maikers-mainframe
yarn install
anchor build
anchor test
```

### Local Testing with Dependencies

All tests run locally with Metaplex Token Metadata automatically cloned from mainnet:

```bash
anchor test  # Automatically clones required programs
```

The Anchor.toml configuration automatically clones required programs for testing.

See [System Architecture](docs/architecture.md) and [Security Model](docs/security.md) for implementation details.

## License

Apache License 2.0 - see [LICENSE](LICENSE) file for details.

## Security & Verified Builds

### Security.txt

This program embeds security contact information directly in the binary using Solana's security.txt standard:

```bash
# Update source_revision to current commit
yarn update:revision

# Check security info in binary
yarn check:security

# Check deployed program
yarn check:security <PROGRAM_ID>
```

**Security Contacts:**
- 📧 Email: security@maikers.com
- 💬 Discord: https://discord.gg/maikers  
- 🐦 Twitter: @TheMaikers
- 📋 Policy: [SECURITY.md](SECURITY.md)

**Note:** The `source_revision` field in security.txt is automatically updated during CI/CD builds to match the git commit being deployed. For local builds, run `yarn update:revision` before building.

### Verified Builds

Run reproducible builds with verification:

```bash
# Build with verification (auto-updates source_revision)
yarn build:verified

# Full verification pipeline
yarn verify
```

This generates:
- ✅ Build verification JSON with binary hash
- ✅ Git commit and timestamp
- ✅ Security.txt validation with source_revision verification

**Documentation:**
- [Verified Build Setup](VERIFIED_BUILD_SETUP.md) - Complete setup guide
- [Scripts README](scripts/README.md) - Script documentation

## Related Repositories

- **[mainframe-sdk](https://github.com/maikershq/maikers-mainframe-sdk)** - TypeScript SDK for client integration
- **[mainframe-node](https://github.com/maikershq/maikers-mainframe-node)** - Agent Stack runtime for AI agent execution

---

**Built by [maikers - creators of realities](https://maikers.com)**
