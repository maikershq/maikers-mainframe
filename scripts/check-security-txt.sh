#!/bin/bash

# Security.txt Verification Script
# Checks security.txt in local binary source code

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║        Security.txt Verification Tool                      ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check source file for security_txt macro
SOURCE_FILE="programs/mainframe/src/lib.rs"

if [ ! -f "$SOURCE_FILE" ]; then
    echo -e "${RED}✗${NC} Source file not found: $SOURCE_FILE"
    exit 1
fi

echo -e "${BLUE}Checking source file:${NC} $SOURCE_FILE"
echo ""
echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"

# Check if security_txt macro is present
if grep -q "security_txt!" "$SOURCE_FILE"; then
    echo -e "${GREEN}✓${NC} security_txt! macro found"
    echo ""
    
    # Extract and display security.txt content
    echo -e "${BLUE}Security.txt content:${NC}"
    echo ""
    
    # Extract the security_txt macro block
    SECURITY_BLOCK=$(sed -n '/security_txt!/,/}/p' "$SOURCE_FILE")
    
    # Parse and display fields
    if echo "$SECURITY_BLOCK" | grep -q "name:"; then
        NAME=$(echo "$SECURITY_BLOCK" | grep "name:" | sed 's/.*name: *"\([^"]*\)".*/\1/')
        echo -e "${GREEN}✓${NC} Name: $NAME"
    fi
    
    if echo "$SECURITY_BLOCK" | grep -q "project_url:"; then
        PROJECT_URL=$(echo "$SECURITY_BLOCK" | grep "project_url:" | sed 's/.*project_url: *"\([^"]*\)".*/\1/')
        echo -e "${GREEN}✓${NC} Project URL: $PROJECT_URL"
    fi
    
    if echo "$SECURITY_BLOCK" | grep -q "contacts:"; then
        CONTACTS=$(echo "$SECURITY_BLOCK" | grep "contacts:" | sed 's/.*contacts: *"\([^"]*\)".*/\1/')
        echo -e "${GREEN}✓${NC} Contacts: $CONTACTS"
    else
        echo -e "${RED}✗${NC} Contacts: Missing (required field)"
        exit 1
    fi
    
    if echo "$SECURITY_BLOCK" | grep -q "policy:"; then
        POLICY=$(echo "$SECURITY_BLOCK" | grep "policy:" | sed 's/.*policy: *"\([^"]*\)".*/\1/')
        echo -e "${GREEN}✓${NC} Policy: $POLICY"
    else
        echo -e "${YELLOW}⚠${NC}  Policy: Not specified"
    fi
    
    if echo "$SECURITY_BLOCK" | grep -q "source_code:"; then
        SOURCE=$(echo "$SECURITY_BLOCK" | grep "source_code:" | sed 's/.*source_code: *"\([^"]*\)".*/\1/')
        echo -e "${GREEN}✓${NC} Source Code: $SOURCE"
    else
        echo -e "${YELLOW}⚠${NC}  Source Code: Not specified"
    fi
    
    if echo "$SECURITY_BLOCK" | grep -q "source_revision:"; then
        REVISION=$(echo "$SECURITY_BLOCK" | grep "source_revision:" | sed 's/.*source_revision: *"\([^"]*\)".*/\1/')
        echo -e "${GREEN}✓${NC} Source Revision: $REVISION"
    else
        echo -e "${YELLOW}⚠${NC}  Source Revision: Not specified (recommended for verified builds)"
    fi
    
    if echo "$SECURITY_BLOCK" | grep -q "auditors:"; then
        AUDITORS=$(echo "$SECURITY_BLOCK" | grep "auditors:" | sed 's/.*auditors: *"\([^"]*\)".*/\1/')
        echo -e "${GREEN}✓${NC} Auditors: $AUDITORS"
    else
        echo -e "${YELLOW}⚠${NC}  Auditors: Not specified"
    fi
    
    echo ""
    echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
    echo ""
    echo -e "${GREEN}✓${NC} Security.txt validation passed"
    echo ""
    echo -e "${BLUE}Note:${NC} This verifies source code only."
    echo -e "       To verify deployed program, use: solana program show <PROGRAM_ID>"
    
else
    echo -e "${RED}✗${NC} security_txt! macro not found in source"
    echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
    echo ""
    echo -e "${YELLOW}To add security.txt, include in your lib.rs:${NC}"
    echo ""
    echo -e '#[cfg(not(feature = "no-entrypoint"))]'
    echo -e 'security_txt! {'
    echo -e '    name: "Your Program Name",'
    echo -e '    project_url: "https://your-project.com",'
    echo -e '    contacts: "email:security@your-project.com",'
    echo -e '    policy: "https://your-project.com/security",'
    echo -e '    source_code: "https://github.com/your-org/your-repo"'
    echo -e '}'
    exit 1
fi

echo ""

