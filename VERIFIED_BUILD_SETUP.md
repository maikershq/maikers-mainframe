# Verified Build & Security.txt Setup

Complete setup for reproducible builds and security.txt verification for Maikers Mainframe.

## ✅ What's Been Set Up

### 1. Scripts Created

#### `scripts/verified-build.sh`
Comprehensive build script that:
- Cleans previous builds
- Performs reproducible Anchor build
- Extracts security.txt from binary
- Validates all required fields
- Generates build verification JSON with hashes
- Displays build summary

#### `scripts/check-security-txt.sh`
Verification tool that:
- Checks local binaries or deployed programs
- Validates security.txt fields
- Reports on contact info, policy, source code
- Warns about missing optional fields

#### `scripts/README.md`
Complete documentation covering:
- Script usage
- Requirements
- Workflow diagrams
- CI/CD integration
- Troubleshooting guide

### 2. Package.json Commands

New commands added:
```json
{
  "build:verified": "./scripts/verified-build.sh",
  "check:security": "./scripts/check-security-txt.sh",
  "verify": "yarn build:verified && yarn check:security"
}
```

### 3. GitHub Actions Workflow

File: `.github/workflows/verified-build.yml`

Automated CI/CD that:
- Runs verified builds on every push/PR
- Performs security audits
- Comments build hashes on PRs
- Uploads artifacts with 30-day retention
- Creates release assets with verification data

## 🚀 Quick Start

### Local Development

```bash
# Run verified build
yarn build:verified

# Check security.txt in local binary
yarn check:security

# Or do both
yarn verify
```

### Check Deployed Program

```bash
yarn check:security <PROGRAM_ID>

# Example
yarn check:security C7LUs9yKERYLs9TBnTVjoL7rtWQGHB7BFvKhjrS5Q6Ze
```

## 📋 Prerequisites

Install the security.txt CLI:
```bash
cargo install solana-security-txt
```

The verified-build script will auto-install if missing.

## 🔍 Security.txt Configuration

Current configuration in `programs/mainframe/src/lib.rs`:

```rust
#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "Maikers Mainframe",
    project_url: "https://mainframe.maikers.com",
    contacts: "email:security@maikers.com,discord:https://discord.gg/maikers,twitter:TheMaikers",
    policy: "https://github.com/maikershq/maikers-mainframe/blob/main/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/maikershq/maikers-mainframe",
    auditors: "To be announced",
    acknowledgements: "Thank you to our security researchers and the Solana community!"
}
```

### Recommended Enhancement

For verified builds, add these optional fields:

```rust
security_txt! {
    // ... existing fields ...
    source_release: "v0.1.0",           // Git tag
    source_revision: "<GIT_COMMIT_SHA>", // Commit hash
}
```

This enables automated build verification tools to match your binary to source code.

## 📦 Build Verification Output

After running `yarn build:verified`, you get:

### Console Output
```
╔════════════════════════════════════════════════════════════╗
║                    BUILD SUCCESSFUL                        ║
╚════════════════════════════════════════════════════════════╝

Build Details:
  Git Commit:   abc123...
  Git Branch:   main
  Binary Hash:  sha256:def456...
  Program ID:   4J4xEFhp...

Next Steps:
  1. Deploy: anchor deploy
  2. Verify on-chain: solana-security-txt 4J4xEFhp...
  3. For Solana Verify: Use the binary hash above
```

### Verification File: `target/build-verification.json`
```json
{
  "program_id": "C7LUs9yKERYLs9TBnTVjoL7rtWQGHB7BFvKhjrS5Q6Ze",
  "git_commit": "abc123def456...",
  "git_branch": "main",
  "build_date": "2025-10-31 12:00:00 UTC",
  "binary_hash": "sha256:def456abc789...",
  "binary_path": "target/deploy/mainframe.so",
  "anchor_version": "0.31.1",
  "rust_version": "1.75.0",
  "security_txt_verified": true
}
```

## 🔄 Recommended Workflow

### Development Cycle

```bash
# 1. Make changes to the program
vim programs/mainframe/src/...

# 2. Run verified build
yarn build:verified

# 3. Deploy to devnet
anchor deploy --provider.cluster devnet

# 4. Verify on-chain
yarn check:security <DEVNET_PROGRAM_ID>

# 5. Test functionality
anchor test
```

### Release Process

```bash
# 1. Final verified build
yarn build:verified

# 2. Save verification file
cp target/build-verification.json release/mainframe-v0.1.0-verification.json

# 3. Tag release with commit hash
git tag -a v1.0.0 -m "Release v0.1.0"
git push origin v0.1.0

# 4. Deploy to mainnet
anchor deploy --provider.cluster mainnet

# 5. Verify on-chain
yarn check:security <MAINNET_PROGRAM_ID>

# 6. Submit to Solana Verify
# (Use hash from verification file)
```

## 🔐 Integration with Solana Verify

To submit your program for verified builds:

### Option 1: Using Solana Verify CLI

```bash
# Install
npm install -g @solana-developers/solana-verify

# Verify
solana-verify verify-from-repo \
  --program-id C7LUs9yKERYLs9TBnTVjoL7rtWQGHB7BFvKhjrS5Q6Ze \
  --repo https://github.com/maikershq/maikers-mainframe \
  --commit-hash <GIT_COMMIT_FROM_BUILD_VERIFICATION>
```

### Option 2: OtterSec Verified

Submit through their platform with:
- Repository URL
- Commit hash (from build-verification.json)
- Binary hash (from build-verification.json)

## 📊 CI/CD Integration

The GitHub Actions workflow automatically:

1. **On every push/PR:**
   - Runs verified build
   - Extracts security.txt
   - Validates configuration
   - Runs security audit
   - Comments results on PR

2. **On releases:**
   - Creates verified build artifact
   - Attaches to release
   - Includes verification JSON

## 🔍 Verification Examples

### Example 1: Successful Verification
```
╔════════════════════════════════════════════════════════════╗
║        Security.txt Verification Tool                      ║
╚════════════════════════════════════════════════════════════╝

Checking local binary: target/deploy/mainframe.so

════════════════════════════════════════════════════════════
name: Maikers Mainframe
project_url: https://mainframe.maikers.com
contacts: email:security@maikers.com,discord:https://discord.gg/maikers
policy: https://github.com/maikershq/maikers-mainframe/blob/main/SECURITY.md
source_code: https://github.com/maikershq/maikers-mainframe
════════════════════════════════════════════════════════════

✓ Security.txt verified successfully

Validating fields...
✓ Contacts: email:security@maikers.com,discord:https://discord.gg/maikers
✓ Policy: https://github.com/maikershq/maikers-mainframe/blob/main/SECURITY.md
✓ Source: https://github.com/maikershq/maikers-mainframe
⚠  Revision: Not specified (recommended for verified builds)

✓ All checks passed
```

### Example 2: On-Chain Verification
```bash
yarn check:security C7LUs9yKERYLs9TBnTVjoL7rtWQGHB7BFvKhjrS5Q6Ze
```

Queries the deployed program and validates the embedded security.txt.

## 🛡️ Security Benefits

### 1. **Transparency**
- Anyone can verify security contact info
- No need to search documentation
- Embedded in the program itself

### 2. **Discoverability**
- Security researchers can find you
- Automated tools can extract contacts
- Visible on block explorers

### 3. **Build Verification**
- Reproducible builds with hashes
- Match binary to source code
- Audit trail for every deployment

### 4. **Trust**
- Shows commitment to security
- Professional security practices
- Community confidence

## 📝 Best Practices

### DO:
✅ Run `yarn build:verified` before every deployment
✅ Save build-verification.json for each release
✅ Tag releases with commit hashes
✅ Keep security.txt contacts current
✅ Submit to Solana Verify for transparency
✅ Update security.txt when contacts change

### DON'T:
❌ Deploy without verified build
❌ Modify binary after verification
❌ Use outdated contact information
❌ Skip security.txt validation
❌ Deploy from unverified sources

## 🔧 Troubleshooting

### Issue: `solana-security-txt: command not found`
**Solution:** Install the CLI
```bash
cargo install solana-security-txt
```

### Issue: `Binary not found`
**Solution:** Build first
```bash
yarn build:verified
```

### Issue: `Failed to extract security.txt`
**Solutions:**
1. Check `Cargo.toml` has `solana-security-txt = "1.1.1"`
2. Verify macro is in `src/lib.rs`
3. Rebuild: `anchor clean && yarn build:verified`

### Issue: On-chain check fails
**Solutions:**
1. Ensure program is deployed
2. Verify correct program ID
3. Check you're on right network (devnet/mainnet)
4. Wait a few seconds after deployment

## 📚 Additional Resources

- **Solana Security.txt Docs:** https://docs.rs/solana-security-txt
- **Solana Verify:** https://github.com/Ellipsis-Labs/solana-verifiable-build
- **OtterSec Verified:** https://docs.osec.io/audits/verified
- **Anchor Security:** https://www.anchor-lang.com/docs/security

## 🤝 Support

For questions or issues with these scripts:

- 📧 Email: security@maikers.com
- 💬 Discord: https://discord.gg/maikers
- 🐦 Twitter: @TheMaikers
- 📖 Docs: scripts/README.md

---

**Setup completed on:** 2025-10-31
**Last updated:** 2025-10-31
**Status:** ✅ Production Ready

