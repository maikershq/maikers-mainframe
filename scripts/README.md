# Mainframe Build & Security Scripts

Scripts for verified builds and security.txt verification.

## Scripts

### 🔨 `verified-build.sh`

Performs a reproducible build with complete security verification.

**What it does:**
1. Cleans previous builds
2. Builds the program with reproducible settings
3. Extracts and verifies security.txt from binary
4. Validates all required security fields
5. Generates build verification data with hashes

**Usage:**
```bash
./scripts/verified-build.sh

# Or via yarn
yarn build:verified
```

**Output:**
- Built binary: `target/deploy/mainframe.so`
- Verification file: `target/build-verification.json`
- Security.txt display in console

### 🔍 `check-security-txt.sh`

Verifies security.txt in local binary or deployed program.

**Usage:**
```bash
# Check local binary
./scripts/check-security-txt.sh

# Check deployed program
./scripts/check-security-txt.sh <PROGRAM_ID>

# Or via yarn
yarn check:security
yarn check:security C7LUs9yKERYLs9TBnTVjoL7rtWQGHB7BFvKhjrS5Q6Ze
```

**Validates:**
- ✓ Contact information
- ✓ Security policy
- ✓ Source code URL
- ✓ Source revision (if present)

## Requirements

### Install solana-security-txt CLI

```bash
cargo install solana-security-txt
```

The verified-build script will auto-install if missing.

## Build Verification Workflow

```
┌─────────────────────────────────────────┐
│ 1. Run verified build                   │
│    $ yarn build:verified                │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ 2. Review security.txt output           │
│    Check all fields are correct         │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ 3. Save binary hash                     │
│    From: target/build-verification.json │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ 4. Deploy program                       │
│    $ anchor deploy                      │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ 5. Verify on-chain                      │
│    $ yarn check:security <PROGRAM_ID>   │
└─────────────────────────────────────────┘
```

## Build Verification File

After running `verified-build.sh`, you'll get `target/build-verification.json`:

```json
{
  "program_id": "4J4x...",
  "git_commit": "abc123...",
  "git_branch": "main",
  "build_date": "2025-10-31 12:00:00 UTC",
  "binary_hash": "sha256...",
  "binary_path": "target/deploy/mainframe.so",
  "anchor_version": "0.31.1",
  "rust_version": "1.75.0",
  "security_txt_verified": true
}
```

**Use this for:**
- 📝 Release notes
- 🔐 Verified build submission (Solana Verify, OtterSec)
- 📊 Audit trail
- 🏷️ Git tags

## Integration with Solana Verify

To submit for verified builds:

1. Run verified build:
   ```bash
   yarn build:verified
   ```

2. Note the binary hash from `target/build-verification.json`

3. Submit to Solana Verify:
   ```bash
   # Using Solana Verify CLI
   solana-verify verify-from-repo \
     --program-id C7LUs9yKERYLs9TBnTVjoL7rtWQGHB7BFvKhjrS5Q6Ze \
     --repo https://github.com/maikershq/maikers-mainframe \
     --commit-hash <GIT_COMMIT>
   ```

## CI/CD Integration

Add to GitHub Actions workflow:

```yaml
- name: Verified Build
  run: |
    cd maikers-mainframe
    ./scripts/verified-build.sh
    
- name: Upload Build Artifacts
  uses: actions/upload-artifact@v3
  with:
    name: build-verification
    path: target/build-verification.json
```

## Troubleshooting

### `solana-security-txt: command not found`

Install the CLI:
```bash
cargo install solana-security-txt
```

### `Binary not found: target/deploy/mainframe.so`

Run a build first:
```bash
anchor build
# or
yarn build:verified
```

### `Failed to extract security.txt`

1. Check that `solana-security-txt` is in `Cargo.toml`
2. Verify the macro is in `src/lib.rs`
3. Rebuild the program

### On-chain verification fails

1. Ensure program is deployed
2. Check program ID is correct
3. Verify you're on the right network (devnet/mainnet)

## Security Best Practices

✅ **DO:**
- Run verified build before each deployment
- Save build-verification.json for each release
- Tag releases with git commit hash
- Keep security.txt contacts up to date
- Submit to Solana Verify for transparency

❌ **DON'T:**
- Deploy without running verified build
- Modify binary after build verification
- Deploy from unknown builds
- Skip security.txt validation

## Support

Questions about these scripts?
- 📧 Email: security@maikers.com
- 💬 Discord: https://discord.gg/maikers
- 🐦 Twitter: @TheMaikers

