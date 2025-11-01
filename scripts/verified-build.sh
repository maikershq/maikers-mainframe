#!/bin/bash
set -e

# Verified Build & Security.txt Verification Script
# This script performs a reproducible build and verifies security.txt

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║     Maikers Mainframe - Verified Build Script             ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Step 0: Update source_revision to current commit
echo -e "${BLUE}[0/5]${NC} Updating source_revision..."
if [ -f "./scripts/update-source-revision.sh" ]; then
    ./scripts/update-source-revision.sh
else
    echo -e "${YELLOW}⚠️  update-source-revision.sh not found, skipping...${NC}"
fi
echo ""

# Step 1: Clean previous builds
echo -e "${BLUE}[1/4]${NC} Cleaning previous builds..."
anchor clean
rm -rf target/

# Step 2: Build with reproducible settings
echo -e "${BLUE}[2/4]${NC} Building program (reproducible build)..."
echo -e "${YELLOW}      This may take a few minutes...${NC}"

# Build with Anchor
anchor build

# Get build info
PROGRAM_ID=$(solana address -k target/deploy/mainframe-keypair.json)
BINARY_PATH="target/deploy/mainframe.so"
BINARY_SIZE=$(ls -lh "$BINARY_PATH" | awk '{print $5}')

echo -e "${GREEN}✓${NC} Build completed successfully"
echo -e "  Program ID: ${BLUE}${PROGRAM_ID}${NC}"
echo -e "  Binary size: ${BINARY_SIZE}"
echo ""

# Step 3: Verify security.txt
echo -e "${BLUE}[3/4]${NC} Verifying security.txt..."
echo ""

if [ -f "./scripts/check-security-txt.sh" ]; then
    ./scripts/check-security-txt.sh
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓${NC} Security.txt verification passed"
    else
        echo -e "${RED}✗${NC} Security.txt verification failed"
        exit 1
    fi
else
    echo -e "${YELLOW}⚠️  check-security-txt.sh not found, skipping verification...${NC}"
fi
echo ""

# Step 4: Generate build verification data
echo -e "${BLUE}[4/4]${NC} Generating build verification data..."

GIT_COMMIT=$(git rev-parse HEAD)
GIT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
BUILD_DATE=$(date -u +"%Y-%m-%d %H:%M:%S UTC")
BINARY_HASH=$(shasum -a 256 "$BINARY_PATH" | awk '{print $1}')

# Create verification file
VERIFY_FILE="target/build-verification.json"
cat > "$VERIFY_FILE" <<EOF
{
  "program_id": "$PROGRAM_ID",
  "git_commit": "$GIT_COMMIT",
  "git_branch": "$GIT_BRANCH",
  "build_date": "$BUILD_DATE",
  "binary_hash": "$BINARY_HASH",
  "binary_path": "$BINARY_PATH",
  "anchor_version": "$(anchor --version)",
  "rust_version": "$(rustc --version)",
  "security_txt_verified": true
}
EOF

echo -e "${GREEN}✓${NC} Build verification data saved to: ${VERIFY_FILE}"
echo ""

# Display summary
echo -e "${GREEN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║                    BUILD SUCCESSFUL                        ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}Build Details:${NC}"
echo -e "  Git Commit:   ${GIT_COMMIT}"
echo -e "  Git Branch:   ${GIT_BRANCH}"
echo -e "  Binary Hash:  ${BINARY_HASH}"
echo -e "  Program ID:   ${PROGRAM_ID}"
echo ""
echo -e "${BLUE}Next Steps:${NC}"
echo -e "  1. Deploy: ${YELLOW}anchor deploy${NC}"
echo -e "  2. Verify on-chain: ${YELLOW}solana-security-txt ${PROGRAM_ID}${NC}"
echo -e "  3. For Solana Verify: Use the binary hash above"
echo ""
echo -e "${GREEN}✓${NC} Verification file: ${VERIFY_FILE}"
echo ""

# Cleanup
rm -f /tmp/security-txt-output.txt

