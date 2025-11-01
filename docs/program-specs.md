# Mainframe: Anchor Program Specifications

## Overview

The Mainframe Anchor program is a permissionless Solana protocol managing AI agents linked to NFTs from verified collections.

## Program Architecture

**Components:**
- **mainframe-program**: On-chain protocol for permissionless agent management
- **mainframe-sdk**: Client library for metadata encryption and URI generation  
- **mainframe-node**: Off-chain execution layer with 200+ plugins
- **mainframe-web**: dApp interface

## Account Structures

### Agent Account

Stores agent state and NFT linkage:
- NFT mint address and owner
- Collection mint (for fee calculation)
- Metadata URI (encrypted configuration)
- Status (Active/Paused/Closed)
- Timestamps and version tracking

### Protocol Configuration

Global program settings:
- Authority and fee structure
- Treasury accounts and distribution percentages
- Partner collections with discounts
- Emergency pause flag

**See**: [Complete Account Structures](references.md#account-structures)

## PDA Derivation

**Agent PDA**: Derived from `["agent", nft_mint]`  
**Protocol Config PDA**: Derived from `["protocol_config"]`

Benefits: Deterministic addressing, collision-resistant, efficient lookups

**See**: [PDA Derivation Code](references.md#pda-derivation)

## Core Instructions

### Agent Lifecycle
- **initialize_config**: Setup protocol configuration
- **create_agent**: Link NFT to new agent
- **update_agent_config**: Modify agent settings
- **transfer_agent**: Transfer ownership
- **pause_agent**: Pause operations
- **close_agent**: Permanently close

### Protocol Management
- **pause**: Emergency protocol pause
- **update_fees**: Modify fee structure
- **update_treasury_distribution**: Change treasury splits
- **add_partner_collection**: Add fee discount collection
- **remove_partner_collection**: Remove partner

### Authority Management
- **propose_authority_transfer**: Initiate 2-step transfer
- **accept_authority_transfer**: Complete transfer
- **cancel_authority_transfer**: Cancel pending transfer

**See**: [Instruction Details](references.md#rust-program-examples)

## Fee System

### Base Fees
- Create agent: 0.05 SOL
- Update config: 0.005 SOL
- Transfer agent: 0.01 SOL
- Pause/close: FREE

### Collection-Based Discounts
- **Genesis**: 100% discount (free)
- **Partners**: 25-75% discount
- **Standard**: Full fees

### Distribution
- Protocol Treasury: 60%
- Validator Treasury: 30%
- Cloud Treasury: 10%

**See**: [Economics & Fees](economics.md)

## Events

### Core Events
- `AgentCreated`: New agent activation
- `AgentUpdated`: Configuration changes
- `AgentTransferred`: Ownership transfers
- `AffiliatePaid`: Affiliate commission payouts

Events enable off-chain systems to track agent lifecycle and process affiliate earnings.

**See**: [Event Structures](references.md#events)

## Security Model

### Access Control
- Owner verification for all agent operations
- NFT ownership validated via token account
- Authority checks for protocol changes
- Permissionless by design (no pre-approval needed)

### Data Integrity
- Metadata size limits and validation
- Version control for updates
- State consistency tracking
- Reserved space for future upgrades

### Economic Security
- Fee validation before execution
- Automatic treasury distribution
- Emergency pause mechanism
- Partner collection management

**See**: [Security Guide](security.md)

## Error Codes

| Code | Error | Description |
|------|-------|-------------|
| 6000 | NFTNotOwned | Signer doesn't own NFT |
| 6001 | AgentNotActive | Agent is paused or closed |
| 6002 | ProtocolPaused | Protocol emergency pause active |
| 6003 | Unauthorized | Missing required permissions |
| 6004 | InvalidNFTMetadata | NFT metadata validation failed |
| 6005 | VersionOverflow | Version counter overflow |
| 6006 | CounterOverflow | Agent counter overflow |
| 6007 | InvalidMetadataUri | Invalid metadata URI |
| 6008 | InvalidTreasuryDistribution | Percentages don't sum to 100% |
| ... | ... | ... |
| 6024 | AlreadyOwner | Agent already owned by new owner |
| 6025 | InvalidNFT | NFT mint doesn't match agent account |

## Integration

**For SDK Integration**: Use [@maikers/mainframe-sdk](https://github.com/maikershq/maikers-mainframe-sdk)

**For Direct Integration**: See [Code References](references.md)

**Code Examples**: See [References](references.md)

## Additional Resources

- **[Architecture](architecture.md)** - System design details
- **[Economics](economics.md)** - Fee calculations and distribution
- **[Security](security.md)** - Security model and best practices
- **[Quickstart](quickstart.md)** - Getting started guide
- **[Code References](references.md)** - Complete code examples
