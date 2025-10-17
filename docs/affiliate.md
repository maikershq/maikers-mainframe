# Affiliate Program

## Overview

The Mainframe affiliate program enables sellers to earn commissions on agent activations. When users create agents through an affiliate link, the seller receives a percentage of the activation fee, with the remainder distributed to protocol treasuries.

## Key Features

- **Configurable Commission**: 0-50% per activation (5000 basis points maximum)
- **Zero-Balance Support**: Affiliates can receive payments to unfunded accounts
- **Permissionless**: No registration required, any valid Solana address can be an affiliate
- **Atomic Payments**: All transfers occur atomically within the activation transaction
- **Event Tracking**: `AffiliatePaid` events enable affiliate analytics and accounting

## Fee Distribution Model

```
Total Activation Fee (100%)
├─ Affiliate Commission (0-50%, configurable)
└─ Protocol Distribution (remaining fee)
   ├─ Protocol Treasury (50% default)
   ├─ Validator Treasury (30% default)
   └─ Network Treasury (20% default)
```

### Example Calculation

For a 50,000,000 lamport activation fee with 10% affiliate:

```
Affiliate:          5,000,000 lamports (10.0%)
Protocol Treasury: 22,500,000 lamports (50% of remaining)
Validator Treasury: 13,500,000 lamports (30% of remaining)
Network Treasury:    9,000,000 lamports (20% of remaining)
Total:             50,000,000 lamports (100%)
```

## Usage

### Basic Integration

```typescript
import { MainframeSDK } from '@maikers/mainframe-sdk';

const sdk = new MainframeSDK(config);
await sdk.initialize();

// Create agent with affiliate
const result = await sdk.createAgent(
  nftMint,
  agentConfig,
  {
    seller: 'SellerPublicKeyBase58',
    affiliateBps: 1000  // 10%
  }
);
```

### Parameters

| Parameter | Type | Range | Description |
|-----------|------|-------|-------------|
| `seller` | `string \| undefined` | Valid Solana address | Affiliate wallet (optional) |
| `affiliateBps` | `number \| undefined` | 0-5000 | Commission in basis points (optional) |

### Validation Rules

- `affiliateBps` must be between 0 and 5000 (0-50%)
- If `affiliateBps` > 0, `seller` must be provided
- Payer must have sufficient balance for total fee
- Seller account must be included in transaction accounts

## Technical Implementation

### Program Instruction

```rust
pub fn create_agent(
    ctx: Context<CreateAgent>,
    nft_mint: Pubkey,
    metadata_uri: String,
    seller_affiliate_bps: u16,
    collection_mint: Option<Pubkey>,
) -> Result<()>
```

### Account Structure

```rust
#[derive(Accounts)]
pub struct CreateAgent<'info> {
    // ... standard accounts
    
    /// Optional seller account that receives affiliate fee
    /// Supports zero-balance accounts
    #[account(mut)]
    pub seller: Option<AccountInfo<'info>>,
}
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

## Events

### AffiliatePaid Event

Emitted when an affiliate receives a commission:

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

### Event Indexing

Listen for `AffiliatePaid` events to:
- Track affiliate earnings
- Generate affiliate dashboards
- Automate payout reconciliation
- Build referral analytics

## Security

### Built-in Protections

1. **Maximum Cap**: 50% hard limit prevents excessive commissions
2. **Checked Arithmetic**: All calculations use overflow-safe operations
3. **Balance Validation**: Payer balance verified before transfers
4. **Atomic Execution**: Entire transaction reverts on any failure
5. **Optional Design**: Backward compatible, doesn't affect non-affiliate flows

### Zero-Balance Safety

Affiliates with 0 SOL can receive payments because:
- Solana requires all transaction accounts to exist
- Zero-balance system accounts are valid recipients
- `checked_add(0, amount)` safely handles initial funding
- No special rent-exemption requirements

## Integration Patterns

### Referral Links

Generate unique referral tracking:

```typescript
const referralUrl = `https://app.maikers.com/activate?ref=${affiliateAddress}`;
```

### Multi-Tier Affiliates

Implement tiered commission structures:

```typescript
const tiers = {
  bronze: 500,   // 5%
  silver: 1000,  // 10%
  gold: 2000,    // 20%
  platinum: 3000 // 30%
};

const affiliateBps = tiers[affiliateTier];
```

### Batch Tracking

Process multiple affiliates in sequence:

```typescript
for (const activation of pendingActivations) {
  await sdk.createAgent(
    activation.nftMint,
    activation.config,
    {
      seller: activation.affiliateAddress,
      affiliateBps: activation.commissionRate
    }
  );
}
```

## Best Practices

### For Protocol Integrators

1. **Validate Addresses**: Verify affiliate addresses before submission
2. **Store Mappings**: Maintain affiliate → user relationships off-chain
3. **Monitor Events**: Index `AffiliatePaid` for real-time tracking
4. **Set Defaults**: Use reasonable default commission rates (5-15%)
5. **Display Transparency**: Show fee breakdown to users before confirmation

### For Affiliates

1. **Unique Tracking**: Use distinct addresses per campaign for analytics
2. **Share Links**: Distribute referral links through appropriate channels
3. **Monitor Earnings**: Track `AffiliatePaid` events for your address
4. **Tax Compliance**: Maintain records for regulatory requirements
5. **Zero Setup**: No need to fund account before first commission

### For End Users

1. **Fee Disclosure**: Affiliates don't increase user costs (taken from protocol share)
2. **Support Creators**: Using affiliate links helps support content creators
3. **Same Experience**: Activation flow identical with or without affiliates

## Economics

### Revenue Sharing Model

The affiliate program creates aligned incentives:

```
Traditional Model:
└─ 100% to Protocol

Affiliate Model:
├─ 0-50% to Seller (growth driver)
└─ 50-100% to Protocol (sustainable)
```

### Network Effects

Affiliates drive growth by:
- Reducing customer acquisition costs
- Enabling grassroots marketing
- Incentivizing education and support
- Creating sustainable revenue streams
- Building community engagement

## Error Handling

### Common Errors

| Error | Cause | Solution |
|-------|-------|----------|
| `InvalidAffiliate` | Commission > 50% | Use 0-5000 bps |
| `InsufficientBalance` | Payer lacks funds | Ensure adequate balance |
| Validation Error | Missing seller with bps > 0 | Provide seller address |
| Account Error | Seller not in transaction | Include seller in accounts |

### Error Examples

```typescript
try {
  await sdk.createAgent(nftMint, config, {
    affiliateBps: 6000  // Invalid: > 50%
  });
} catch (error) {
  // Error: InvalidAffiliate
}

try {
  await sdk.createAgent(nftMint, config, {
    affiliateBps: 1000  // Missing seller
  });
} catch (error) {
  // Error: Seller required when affiliateBps > 0
}
```

## Monitoring & Analytics

### Key Metrics

Track affiliate program health:

```typescript
// Total commissions paid
SELECT SUM(affiliate_amount) FROM affiliate_paid_events;

// Top performers
SELECT seller, COUNT(*), SUM(affiliate_amount) 
FROM affiliate_paid_events 
GROUP BY seller 
ORDER BY SUM(affiliate_amount) DESC;

// Average commission rate
SELECT AVG(affiliate_bps / 100.0) FROM affiliate_paid_events;

// Conversion tracking
SELECT seller, 
       COUNT(*) as activations,
       SUM(affiliate_amount) as earnings
FROM affiliate_paid_events
WHERE timestamp >= NOW() - INTERVAL '30 days'
GROUP BY seller;
```

## FAQ

**Q: Do affiliates need SOL to receive commissions?**  
A: No, affiliates can receive their first commission with a 0-balance account.

**Q: What's the maximum commission rate?**  
A: 50% (5000 basis points) to ensure protocol sustainability.

**Q: Can I change commission rates per activation?**  
A: Yes, commission is set per-transaction, allowing flexible rate structures.

**Q: Are there minimum payout thresholds?**  
A: No, affiliates receive payment immediately during activation.

**Q: How do I track my earnings?**  
A: Monitor `AffiliatePaid` events filtered by your affiliate address.

**Q: Can multiple affiliates share one activation?**  
A: No, only one affiliate per activation. Implement multi-tier off-chain if needed.

**Q: Does using an affiliate increase user costs?**  
A: No, the activation fee remains constant; commission comes from protocol share.

**Q: How quickly are commissions paid?**  
A: Instantly, within the activation transaction (atomic).

## Additional Resources

- **Technical Specs**: See `AFFILIATE_FEATURE.md` for implementation details
- **Examples**: Check `examples/affiliate-example.ts` for code samples
- **Testing**: Review `tests/test-zero-balance-transfer.md` for edge cases
- **SDK Docs**: Refer to `@maikers/mainframe-sdk` documentation
- **Program Code**: Browse `programs/mainframe/src/` for source

## Support

For integration assistance:
- GitHub Issues: [maikers-mainframe/issues](https://github.com/maikers/mainframe/issues)
- Documentation: [docs.maikers.com](https://docs.maikers.com)
- Discord: [discord.gg/maikers](https://discord.gg/maikers)

