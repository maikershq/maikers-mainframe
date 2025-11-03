# Affiliate Program

## Overview

The Mainframe affiliate program is a **permissionless, tier-based incentive system** that rewards anyone who drives agent activations. No registration required—just provide your wallet address and start earning instantly.

### 🚀 Quick Start

**Anyone can be an affiliate:**
1. Share your wallet address with users
2. They create agents using your address
3. You earn commission instantly (15-50% based on tier)
4. Your affiliate account auto-initializes on first sale
5. Climb tiers automatically as you sell more

**See [Tier System Guide](tiers.md) for tier progression details.**

## Key Features

- ✅ **Permissionless**: Anyone can be an affiliate with just their wallet address
- ✅ **Auto-Initialize**: First commission automatically creates your affiliate account
- ✅ **Tier-Based**: Earn 15-50% based on lifetime sales performance
- ✅ **Referrals**: Earn from your referrals' sales
- ✅ **Instant Payouts**: Commission paid on-chain in real-time

## Tier Structure

| Tier | Sales Threshold | Commission | Monthly (100 sales) | Monthly (500 sales) |
|------|----------------|------------|---------------------|---------------------|
| 🥉 Bronze | 0-99 | **15%** | 0.75 SOL | 3.75 SOL |
| 🥈 Silver | 100-499 | **20%** | 1 SOL | 5 SOL |
| 🥇 Gold | 500-1,999 | **30%** | 1.5 SOL | 7.5 SOL |
| 💠 Platinum | 2,000-9,999 | **40%** | 2 SOL | 10 SOL |
| 💎 Diamond | 10,000+ | **50%** | 2.5 SOL | 12.5 SOL |

*Based on 0.05 SOL standard activation fee

### Key Rules

- **No Demotion**: Tiers are permanent once achieved
- **Cumulative Sales**: Total lifetime sales, never decrease
- **Linear Progression**: Must pass through each tier sequentially
- **All Sales Equal**: Each agent activation = 1 sale

> ⚠️ **Exception**: Protocol reserves the right to demote or ban affiliates engaging in fraudulent activity, wash trading, or violating terms of service.

## Referral System

**Single-Level Referrals**: Affiliates can refer new affiliates and earn 5% of their referrals' commissions.

**How it works:**
1. Affiliate A registers with referrer: `register_affiliate(referrer: Some(Affiliate_B))`
2. When Affiliate A earns commission, Affiliate B automatically receives 5%
3. Referrer revenue is tracked separately in `referree_revenue` field

**Example:**
- User creates agent → 0.05 SOL fee
- Affiliate A earns 15% commission → 0.0075 SOL
- Referrer B earns 5% of commission → 0.000375 SOL (5% of 0.0075 SOL)

**Note**: Single-level only (no multi-level chains) to prevent referral saturation attacks.

## Advanced Features

For comprehensive guides, see:
- **[Economics & Fees](economics.md)** - Complete fee structure and distribution
- **[Program Specifications](program-specs.md)** - Technical implementation details

## Fee Distribution Model

- **Affiliate Commission**: 15-50% (tier-based + bonus)
- **Referrer Commission**: 5% of affiliate commission (if applicable)
- **Protocol Treasury**: 50% of remaining
- **Validator Treasury**: 30% of remaining
- **Network Treasury**: 20% of remaining

**Example**: Bronze affiliate (15%) receives 7.5M lamports from 50M lamport fee, referrer receives 0.375M lamports (5%), remainder distributed across treasuries.

## SDK Integration

For complete integration examples, see **[@maikers/mainframe-sdk](https://github.com/maikershq/maikers-mainframe-sdk)**

## Events

The `AffiliatePaid` event is emitted when affiliates receive commissions. Use this for:
- Tracking affiliate earnings
- Building dashboards
- Automating reconciliation

**See**: [Event Structures](references.md#events)

## Security

**Built-in Protections:**
- 50% maximum cap prevents excessive commissions
- Checked arithmetic prevents overflows
- Balance validation before transfers
- Atomic execution (all-or-nothing)
- Zero-balance accounts can receive payments

## Best Practices

**For Affiliates:**
- Use unique addresses per campaign
- Monitor earnings via events
- Maintain tax records
- No setup costs required

**For Users:**
- Affiliates don't increase costs
- Support creators via referral links
- Identical activation experience

## FAQ

**Q: Do affiliates need SOL to receive commissions?**  
A: No, zero-balance accounts can receive payments.

**Q: What's the maximum commission rate?**  
A: 50% (5000 basis points).

**Q: Can I change commission rates per activation?**  
A: Yes, commission is set per-transaction, allowing flexible rate structures.

**Q: Are there minimum payout thresholds?**  
A: No, affiliates receive payment immediately during activation.

**Q: How do I track my earnings?**  
A: Monitor `AffiliatePaid` events filtered by your affiliate address.

**Q: Does using an affiliate increase user costs?**  
A: No, fees remain constant; commission from protocol share.

**Q: How quickly are commissions paid?**  
A: Instantly, within the activation transaction.

## Additional Resources

- **[Code Examples](references.md)** - Complete SDK and Rust examples
- **[Economics & Fees](economics.md)** - Fee structure details
- **[Program Specs](program-specs.md)** - Technical implementation
- **[Mainframe SDK](https://github.com/maikershq/maikers-mainframe-sdk)** - TypeScript SDK

