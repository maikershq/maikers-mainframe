# Mainnet Deployment Checklist
## Maikers Mainframe - Production Deployment

**Date**: November 2, 2025
**Program ID**: `mnfm211AwTDA8fGvPezYs3jjxAXgoucHGuTMUbjFssE`
**Commit**: `65494192737217aa5228a9ff52d5002081b02447`

---

## ✅ Pre-Deployment Verification

### Code Quality & Build
- [x] security.txt properly configured with all fields
- [x] Source revision matches current commit
- [x] IDL generated and available
- [x] Verified build scripts ready
- [ ] Uncommitted changes committed
- [ ] Clean verified build completed
- [ ] Binary hash documented

### Security Requirements
- [x] security.txt includes:
  - Contact: security@maikers.com, Discord, Twitter
  - Policy: SECURITY.md on GitHub
  - Source code: GitHub repository
  - Source revision: Current commit hash
  - Auditors: TBA
  - Acknowledgements: Community thanks
- [x] SECURITY.md policy documented
- [ ] solana-security-txt CLI installed
- [ ] security.txt verified in binary

### Infrastructure
- [x] Wallet balance sufficient (11.94 SOL)
- [x] Program NOT yet deployed to mainnet
- [x] Anchor version: 0.31.1
- [x] Rust version: 1.88.0
- [ ] RPC endpoint switched to mainnet

### Documentation
- [x] README.md complete
- [x] SECURITY.md available
- [x] security.txt canonical file exists
- [x] Deployment checklist exists
- [x] Verified build setup documented

---

## 🚀 Deployment Steps

### Step 1: Prepare Environment
```bash
# Install solana-security-txt CLI
cargo install solana-security-txt

# Commit source_revision update
git add programs/mainframe/src/lib.rs
git commit -m "chore: update source_revision for mainnet deployment"
git push origin main
```

### Step 2: Verified Build
```bash
# Run verified build
yarn build:verified

# Verify security.txt in binary
yarn check:security

# Save verification data
cp target/build-verification.json production/mainnet-verification-v1.0.0.json
```

### Step 3: Pre-Deployment Verification
```bash
# Verify binary size (should be reasonable)
ls -lh target/deploy/mainframe.so

# Check program ID matches Anchor.toml
solana address -k target/deploy/mainframe-keypair.json

# Verify IDL exists
cat target/idl/mainframe.json | jq '.address'

# Document binary hash
shasum -a 256 target/deploy/mainframe.so
```

### Step 4: Deploy to Mainnet
```bash
# Switch to mainnet
solana config set --url mainnet-beta

# Verify wallet and balance
solana address
solana balance

# Deploy program
anchor deploy --provider.cluster mainnet-beta

# Expected output: Program deployed successfully
```

### Step 5: Post-Deployment Verification
```bash
# Verify program is deployed
solana program show mnfm211AwTDA8fGvPezYs3jjxAXgoucHGuTMUbjFssE --url mainnet-beta

# Verify security.txt on-chain
solana-security-txt mnfm211AwTDA8fGvPezYs3jjxAXgoucHGuTMUbjFssE --url mainnet-beta

# Verify IDL is included
# (Check program data account for IDL)

# Test basic RPC call
solana account mnfm211AwTDA8fGvPezYs3jjxAXgoucHGuTMUbjFssE --url mainnet-beta
```

### Step 6: Documentation & Announcement
```bash
# Tag the release
git tag -a v1.0.0-mainnet -m "Mainnet deployment v1.0.0"
git push origin v1.0.0-mainnet

# Update documentation with mainnet addresses
# Announce deployment to community
# Update SDK with mainnet configuration
```

---

## 📊 Verification Outputs to Save

### Build Verification Data
- Binary hash (SHA-256)
- Git commit hash
- Build timestamp
- Anchor version
- Rust version
- Program ID

### Deployment Verification
- Transaction signature
- Deployment timestamp
- On-chain security.txt validation
- Program data account address
- Executable account address

### File Locations
- `production/mainnet-verification-v1.0.0.json` - Build verification
- `target/build-verification.json` - Latest build data
- `target/deploy/mainframe.so` - Deployed binary
- `target/idl/mainframe.json` - Program IDL

---

## 🔐 Security Checklist

### Access Control
- [ ] Upgrade authority secured
- [ ] Deployment wallet secured (hardware wallet recommended)
- [ ] Backup of keypairs stored securely
- [ ] Access to security@maikers.com monitored

### On-Chain Verification
- [ ] security.txt readable on-chain
- [ ] Contact information accurate
- [ ] Policy URL accessible
- [ ] Source code URL correct
- [ ] Commit hash matches deployment

### Monitoring Setup
- [ ] Program logs monitoring configured
- [ ] Transaction monitoring active
- [ ] Error rate alerts set up
- [ ] Unusual activity detection ready

---

## 🎯 Post-Deployment Tasks

### Immediate (Day 1)
- [ ] Verify all on-chain data
- [ ] Test protocol initialization
- [ ] Update SDK configuration
- [ ] Announce deployment
- [ ] Monitor initial transactions

### Short Term (Week 1)
- [ ] Submit to Solana Verify
- [ ] Update all documentation
- [ ] Test all instructions
- [ ] Monitor for issues
- [ ] Community feedback collection

### Medium Term (Month 1)
- [ ] Security audit scheduled
- [ ] Performance monitoring
- [ ] Usage analytics
- [ ] Partnership integrations
- [ ] SDK adoption tracking

---

## 📞 Emergency Contacts

**Security Issues**
- Email: security@maikers.com
- Discord: https://discord.gg/maikers

**Deployment Team**
- Primary: [To be filled]
- Backup: [To be filled]

**Incident Response**
1. Pause protocol if critical issue found
2. Contact security team immediately
3. Document issue thoroughly
4. Prepare fix and verification plan
5. Coordinate disclosure timeline

---

## ✅ Final Checklist Before Deployment

**CRITICAL - Verify these before running `anchor deploy`:**

1. [ ] Git commit matches source_revision in code
2. [ ] Verified build completed successfully
3. [ ] security.txt verified in binary
4. [ ] Binary hash documented
5. [ ] IDL exists and is complete
6. [ ] Wallet has sufficient SOL
7. [ ] RPC endpoint is mainnet-beta
8. [ ] Backup of all keys secured
9. [ ] Team notified of deployment
10. [ ] Rollback plan documented

**Once deployed, CANNOT be undone without upgrade authority.**

---

## 🎉 Success Criteria

Deployment is successful when:
- ✅ Program deployed to correct address
- ✅ security.txt readable on-chain
- ✅ IDL accessible
- ✅ Basic instructions executable
- ✅ No immediate errors or issues
- ✅ Verification data saved
- ✅ Community announcement made

---

**Prepared by**: Maikers Development Team
**Last Updated**: November 2, 2025
**Status**: READY FOR REVIEW

