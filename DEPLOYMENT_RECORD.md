# Mainframe Production Deployment Record

## Current Mainnet Deployment

### Program Information
- **Program ID:** `mnfm211AwTDA8fGvPezYs3jjxAXgoucHGuTMUbjFssE`
- **Status:** ✅ Live and Operational
- **Version:** v1.0.0
- **Deployment Date:** November 2, 2025
- **Network:** Solana Mainnet

### Deployment Details
- **Transaction Signature:** `5SQiEwMMDFxhUqhEmLyvThvUVbzpcNLHYREPHvfkCZDK75NktxyNt3BtSGQdsgWujPBdbPCoKVcbagWZxPpy375T`
- **Slot:** 377513128
- **Block Time:** 2025-11-02 22:08:37 UTC
- **Transaction Link:** [View on Solscan](https://solscan.io/tx/5SQiEwMMDFxhUqhEmLyvThvUVbzpcNLHYREPHvfkCZDK75NktxyNt3BtSGQdsgWujPBdbPCoKVcbagWZxPpy375T)

### Build Information
- **Source Commit:** `efdd0c639381df7648ef60e02c4febba381222b6`
- **Git Tag:** `v1.0.0-mainnet`
- **Build Platform:** macOS ARM64 (Apple Silicon)
- **Build Method:** Native `anchor build` via `./scripts/verified-build.sh`
- **Binary Hash (SHA-256):** `acf985a64302b6f269a0b9db6aed3ee6c7565a3f122f35e1322df7ccbba68055`

### Security Information
- **Security.txt:** ✅ Embedded in binary
- **Source Revision:** `efdd0c639381df7648ef60e02c4febba381222b6` (embedded)
- **Security Contact:** security@maikers.com
- **Discord:** https://discord.gg/maikers
- **Twitter:** @TheMaikers
- **Security Policy:** [SECURITY.md](../SECURITY.md)

### Verification Status
- **Solana Verify Compatible:** ❌ No (platform mismatch)
- **Manual Verification:** ✅ Yes (local build hash matches on-chain)
- **Security.txt Verified:** ✅ Yes
- **Source Code Available:** ✅ Yes (GitHub)

### Authority Information
- **Upgrade Authority:** `DPmf2Tx7SQsTLgEEEPC3kUznTRLu5KFrArEB7V5SHN27`
- **Program Data Account:** `tBcUh1AHJ6sKTpCwwfkn9CZ9JCvotHJ6saK3ok7odeC`

### Configuration (At Deployment)
Refer to [config.md](config.md) for current mainnet configuration values.

## Deployment History

### v1.0.0 - Initial Mainnet Deployment
**Date:** November 2, 2025  
**Commit:** efdd0c639381df7648ef60e02c4febba381222b6  
**Status:** ✅ Active

**Changes:**
- Initial mainnet deployment
- Full agent lifecycle management
- Tiered fee structure with collection discounts
- Affiliate revenue sharing system
- PDA-based agent accounts

**Notes:**
- Deployed using macOS ARM64 native build
- Not compatible with solana-verify due to platform difference
- Manually verified with local build comparison
- All subsequent upgrade attempts were unsuccessful (see VERIFICATION_STATUS.md)

## Important Links

- **Complete Deployment Documentation:** [../VERIFICATION_STATUS.md](../VERIFICATION_STATUS.md)
- **Deployment Checklist:** [../MAINNET_DEPLOYMENT_CHECKLIST.md](../MAINNET_DEPLOYMENT_CHECKLIST.md)
- **Configuration Details:** [config.md](config.md)
- **Initialization Plan:** [INITIALIZATION_PLAN.md](INITIALIZATION_PLAN.md)
- **Program Explorer:** [View on Solscan](https://solscan.io/account/mnfm211AwTDA8fGvPezYs3jjxAXgoucHGuTMUbjFssE)

## Monitoring

### Health Checks
- Program account exists: ✅
- Upgrade authority set correctly: ✅
- Security.txt embedded: ✅
- Configuration initialized: ⏳ Pending

### Key Metrics to Monitor
- Transaction success rate
- Agent creation count
- Fee collection and distribution
- Affiliate participation
- Program account balance

## Future Deployments

For future verifiable deployments, see instructions in [VERIFICATION_STATUS.md](../VERIFICATION_STATUS.md#for-future-deployments).

**Key requirement:** Use Docker AMD64 builds with `indexmap@2.11.4` or lower for Rust 1.79 compatibility.

---

**Last Updated:** 2025-11-02  
**Maintained By:** Maikers Engineering Team  
**Contact:** security@maikers.com

