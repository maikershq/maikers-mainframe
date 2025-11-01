#!/bin/bash

# Test script to verify security.txt setup is working correctly

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║     Security.txt Setup Verification Test                   ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

FAILED=0

# Test 1: Check if security.txt dependency exists
echo -e "${BLUE}[1/6]${NC} Checking Cargo.toml dependency..."
if grep -q "solana-security-txt" programs/mainframe/Cargo.toml; then
    echo -e "${GREEN}✓${NC} solana-security-txt dependency found"
else
    echo -e "${RED}✗${NC} solana-security-txt dependency missing"
    FAILED=$((FAILED + 1))
fi
echo ""

# Test 2: Check if security_txt macro is in lib.rs
echo -e "${BLUE}[2/6]${NC} Checking lib.rs for security_txt macro..."
if grep -q "security_txt!" programs/mainframe/src/lib.rs; then
    echo -e "${GREEN}✓${NC} security_txt macro found"
else
    echo -e "${RED}✗${NC} security_txt macro missing"
    FAILED=$((FAILED + 1))
fi
echo ""

# Test 3: Check required fields in security.txt
echo -e "${BLUE}[3/6]${NC} Checking required security.txt fields..."

REQUIRED_FIELDS=("name:" "contacts:" "policy:" "source_code:")
for field in "${REQUIRED_FIELDS[@]}"; do
    if grep -A 15 "security_txt!" programs/mainframe/src/lib.rs | grep -q "$field"; then
        echo -e "${GREEN}✓${NC} $field present"
    else
        echo -e "${RED}✗${NC} $field missing"
        FAILED=$((FAILED + 1))
    fi
done
echo ""

# Test 4: Check if scripts exist and are executable
echo -e "${BLUE}[4/6]${NC} Checking build scripts..."

SCRIPTS=("scripts/verified-build.sh" "scripts/check-security-txt.sh")
for script in "${SCRIPTS[@]}"; do
    if [ -f "$script" ]; then
        if [ -x "$script" ]; then
            echo -e "${GREEN}✓${NC} $script exists and is executable"
        else
            echo -e "${YELLOW}⚠${NC}  $script exists but not executable"
            chmod +x "$script"
            echo -e "     Fixed: Made executable"
        fi
    else
        echo -e "${RED}✗${NC} $script missing"
        FAILED=$((FAILED + 1))
    fi
done
echo ""

# Test 5: Check package.json scripts
echo -e "${BLUE}[5/6]${NC} Checking package.json scripts..."

YARN_SCRIPTS=("build:verified" "check:security" "verify")
for script in "${YARN_SCRIPTS[@]}"; do
    if grep -q "\"$script\"" package.json; then
        echo -e "${GREEN}✓${NC} yarn $script command available"
    else
        echo -e "${RED}✗${NC} yarn $script command missing"
        FAILED=$((FAILED + 1))
    fi
done
echo ""

# Test 6: Check if solana-security-txt CLI is installed
echo -e "${BLUE}[6/6]${NC} Checking solana-security-txt CLI..."
if command -v solana-security-txt &> /dev/null; then
    VERSION=$(solana-security-txt --version 2>&1 || echo "version unknown")
    echo -e "${GREEN}✓${NC} solana-security-txt CLI installed ($VERSION)"
else
    echo -e "${YELLOW}⚠${NC}  solana-security-txt CLI not installed"
    echo -e "     Install with: ${YELLOW}cargo install solana-security-txt${NC}"
fi
echo ""

# Summary
echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✓ All tests passed! Security.txt setup is correct.${NC}"
    echo ""
    echo -e "${BLUE}Next steps:${NC}"
    echo -e "  1. Run: ${YELLOW}yarn build:verified${NC}"
    echo -e "  2. Check output for security.txt verification"
    echo -e "  3. Deploy and verify: ${YELLOW}yarn check:security <PROGRAM_ID>${NC}"
    exit 0
else
    echo -e "${RED}✗ $FAILED test(s) failed. Please fix the issues above.${NC}"
    exit 1
fi

