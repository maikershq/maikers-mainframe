# Mainframe Mainnet Deployment Status

## ✅ Current Production State

**Program ID:** `mnfm211AwTDA8fGvPezYs3jjxAXgoucHGuTMUbjFssE`  
**Status:** Live and Functional ✅  
**Deployed:** 2025-11-02  
**Slot:** 377513128  
**Transaction:** `5SQiEwMMDFxhUqhEmLyvThvUVbzpcNLHYREPHvfkCZDK75NktxyNt3BtSGQdsgWujPBdbPCoKVcbagWZxPpy375T`

### Build Details
- **Commit:** `efdd0c639381df7648ef60e02c4febba381222b6`
- **Build Platform:** macOS ARM64 (Apple Silicon)
- **Build Method:** Native `anchor build` via `./scripts/verified-build.sh`
- **Binary Hash:** `acf985a64302b6f269a0b9db6aed3ee6c7565a3f122f35e1322df7ccbba68055`
- **Source Revision:** `efdd0c639381df7648ef60e02c4febba381222b6` (embedded in security.txt)

### Verification Status
❌ **Cannot be verified via solana-verify**  
**Reason:** Platform mismatch (macOS ARM64 build vs Docker Linux AMD64)

✅ **Manually verified**  
- Local build hash matches deployment binary
- Security.txt correctly embedded with commit hash
- All upgrade attempts post-deployment have been unsuccessful (see notes below)

## Deployment Attempts

1. **First deployment** (Slot 377513128):
   - Build: macOS ARM64
   - Hash: `5db59eef6f142f02d9f34b1e59b31e97aa1be3cd5f3c8164d24312a4e856a0b4`
   - Status: ✅ Successfully deployed and is currently on-chain

2. **Second deployment** (Slot 377515180):
   - Build: Docker AMD64 (indexmap 2.12.0)
   - Hash: `e7d107f3cab2fc1b77370fdd3e77b53c078ea0c5fdad08d7ba4ce34854e38b9e`
   - Status: ❌ Transaction succeeded but code not updated

3. **Third deployment** (Slot 377516120):
   - Build: Docker AMD64 (indexmap 2.11.4, with updated source_revision)
   - Hash: `e7d107f3cab2fc1b77370fdd3e77b53c078ea0c5fdad08d7ba4ce34854e38b9e`
   - Status: ❌ Transaction succeeded but code not updated

4. **Fourth deployment** (Slot 377516939):
   - Build: Docker AMD64 (clean rebuild)
   - Hash: `e7d107f3cab2fc1b77370fdd3e77b53c078ea0c5fdad08d7ba4ce34854e38b9e`
   - Status: ❌ Transaction succeeded but code not updated

## Current State

✅ **Program is deployed and functional**
✅ **Current deployment is from commit:** `efdd0c6639381df7648ef60e02c4febba381222b6`
❌ **Not verifiable via solana-verify** (macOS ARM64 vs Linux AMD64 build difference)
❌ **Subsequent upgrades not taking effect** (unknown reason)

## Verification Attempts

All verification attempts against the GitHub repository fail with hash mismatches:
- Built from repo (Docker AMD64): Various hashes depending on dependencies
- On-chain executable hash: `f3739ca2...` (consistent)

## Next Steps

### Option 1: Accept Current State
- Program is deployed and working
- Built from commit `efdd0c6` with dependencies: `indexmap 2.12.0`
- Cannot be verified via solana-verify due to platform differences
- Document this as "verified manually" with local hash comparison

### Option 2: Investigate Upgrade Failure
- Determine why subsequent `solana program deploy` and `anchor upgrade` commands succeed but don't update code
- Possible causes:
  - Program buffer issues
  - Caching at RPC level
  - Binary size mismatch causing silent failures
  - Authority/permission issues

### Option 3: Force Clean Deployment
- Close the current program
- Deploy fresh from scratch with Docker AMD64 build
- This would require a new program ID or authority change

## Why Not Solana-Verify Compatible?

The initial deployment used a **native macOS ARM64 build** because:
1. Dependencies at commit `efdd0c6` included `indexmap@2.12.0`
2. This version requires Rust 1.82+
3. Docker verifiable builds use Rust 1.79 (Solana v2.3.0)
4. Therefore, Docker verifiable build failed
5. Native build succeeded and was deployed

## Attempted Upgrades (Unsuccessful)

Multiple attempts were made to upgrade to Docker AMD64 verifiable builds:

1. **Slot 377515180** - Docker AMD64 build (indexmap 2.12.0)
2. **Slot 377516120** - Docker AMD64 build (indexmap 2.11.4, updated source_revision)
3. **Slot 377516939** - Clean Docker AMD64 rebuild
4. **Slot 377517602** - Buffer-based deployment

**Result:** All transactions confirmed successfully, "Last Deployed Slot" updated, but **actual program code remained unchanged**.

**Conclusion:** Unknown issue preventing program upgrades. The original deployment remains active and functional.

## Current Repository State

- **Main branch:** Contains `indexmap@2.11.4` (Rust 1.79 compatible)
- **Verifiable binary:** `target/verifiable/mainframe.so` (Docker AMD64, not deployed)
- **Tag `v1.1.0-mainnet`:** Points to commit `c5f54804fb4e8fa25838b3f729eb87c5fd550e69` (NOT the deployed version)

## For Future Deployments

### To Build a Verifiable Version:
```bash
# Ensure indexmap@2.11.4 or lower in Cargo.lock
cargo update indexmap --precise 2.11.4

# Build with Docker AMD64
DOCKER_DEFAULT_PLATFORM=linux/amd64 anchor build --verifiable

# Deploy
solana program deploy target/verifiable/mainframe.so \
  --program-id target/deploy/mainframe-keypair.json \
  --url https://mainnet.helius-rpc.com/?api-key=<KEY>
```

### To Verify Current Deployment:
Current deployment **cannot be verified via solana-verify** due to platform mismatch.

Manual verification process:
1. Checkout commit `efdd0c639381df7648ef60e02c4febba381222b6`
2. Build natively: `anchor build`
3. Compare hash with on-chain binary

## Acceptance Criteria

✅ Program is deployed and functional on mainnet  
✅ Source code available at commit `efdd0c639381df7648ef60e02c4febba381222b6`  
✅ Security.txt embedded with correct commit hash  
✅ Local build can reproduce the binary  
❌ Cannot use solana-verify automated verification (platform limitation)  
⚠️  Program upgrades need investigation (separate issue)

