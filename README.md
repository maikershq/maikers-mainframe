# Mainframe

**Solana Anchor program for managing AI agents linked to verified NFT collections**

[![Anchor](https://img.shields.io/badge/Anchor-0.31.1-purple)](https://coral-xyz.github.io/anchor/)
[![Solana](https://img.shields.io/badge/Solana-Compatible-green)](https://solana.com/)
[![Security](https://img.shields.io/badge/Security-Audited-brightgreen)](#security)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue)](LICENSE)

## Documentation

### Getting Started
- **[Quick Start Guide](docs/quickstart.md)** - Get up and running quickly
- **[Affiliate Program](docs/affiliate.md)** - Revenue sharing and referral system
- **[System Architecture](docs/architecture.md)** - Technical system overview
- **[Integration Guide](docs/integration.md)** - Developer integration instructions

### Technical Reference
- **[Program Specifications](docs/program-specs.md)** - Anchor program documentation
- **[Security Model](docs/security.md)** - Security architecture and best practices
- **[Economics & Fees](docs/economics.md)** - Fee structure and distribution
- **[Partner PDA Architecture](docs/partner-pda-architecture.md)** - Scalable partner system (1000+)

## Overview

Mainframe is a Solana Anchor program that manages agent accounts linked to NFTs from verified collections. The program handles NFT ownership validation, fee collection and distribution, and emits events for off-chain systems.

**Core Capabilities:**
- Agent lifecycle management (create, update, transfer, pause, close)
- PDA-based agent accounts with deterministic derivation
- Tiered fee structure with collection-based discounts
- Automatic fee distribution across treasuries
- Affiliate revenue sharing system
- Event emission for off-chain consumption

See [Program Specifications](docs/program-specs.md) for detailed instruction and account documentation.

## Fee Structure

| Operation | Standard | Genesis | Partners |
|-----------|----------|---------|----------|
| Create Agent | 0.05 SOL | FREE | 25-75% off |
| Update Config | 0.005 SOL | FREE | 25-75% off |
| Transfer Agent | 0.01 SOL | FREE | 25-75% off |
| Pause/Resume/Close | FREE | FREE | FREE |

**Distribution**: 60% Protocol • 30% Validators • 10% Cloud

See [Economics & Fees](docs/economics.md) for complete fee calculation logic.

## Revenue Sharing

**Earn SOL by integrating Mainframe into your platform or referring users**

### Affiliate Program
- **Up to 50% commission** on all agent activation fees
- **No caps or limits** - earn on every referral
- **Instant payouts** - revenue shared on-chain in real-time
- **Easy integration** - simple referral parameter in agent creation

### Partner Benefits
- **Collection discounts** - 25-75% fee reductions for your users
- **Custom fee structures** - negotiate volume-based pricing
- **Priority support** - dedicated integration assistance
- **Marketing collaboration** - co-promotion opportunities

### Revenue Examples
| Monthly Referrals | Fee Revenue | Your Share (25%) | Your Share (50%) |
|------------------|-------------|------------------|------------------|
| 100 agents | 5 SOL | **1.25 SOL** | **2.5 SOL** |
| 500 agents | 25 SOL | **6.25 SOL** | **12.5 SOL** |
| 1,000 agents | 50 SOL | **12.5 SOL** | **25 SOL** |

*Based on 0.05 SOL standard activation fee

**Ready to start earning?** See [Affiliate Program](docs/affiliate.md) for implementation details.

## Quick Start

### For SDK Integration

**Use the official Mainframe SDK for simplified integration:**

📦 **[Mainframe SDK](https://github.com/maikershq/maikers-mainframe-sdk)** - TypeScript SDK with built-in encryption, storage, and transaction management

### For Direct Program Integration

**Documentation:**
- [Quick Start Guide](docs/quickstart.md) - Complete setup and usage
- [Integration Guide](docs/integration.md) - Developer integration patterns
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

See [System Architecture](docs/architecture.md) and [Security Model](docs/security.md) for implementation details.

## License

Apache License 2.0 - see [LICENSE](LICENSE) file for details.

## Related Repositories

- **[mainframe-sdk](https://github.com/maikershq/maikers-mainframe-sdk)** - TypeScript SDK for client integration
- **[mainframe-node](https://github.com/maikershq/maikers-mainframe-node)** - Agent Stack runtime for AI agent execution

---

**Built by [maikers - creators of realities](https://maikers.com)**
