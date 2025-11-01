#!/bin/bash

# Update source_revision in security.txt with current git commit

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

SOURCE_FILE="programs/mainframe/src/lib.rs"

if [ ! -f "$SOURCE_FILE" ]; then
    echo -e "${RED}✗${NC} Source file not found: $SOURCE_FILE"
    exit 1
fi

# Get current git commit hash
GIT_COMMIT=$(git rev-parse HEAD)

if [ -z "$GIT_COMMIT" ]; then
    echo -e "${RED}✗${NC} Failed to get git commit hash"
    exit 1
fi

echo -e "${BLUE}Updating security.txt source_revision...${NC}"
echo -e "Commit: ${YELLOW}$GIT_COMMIT${NC}"

# Check if source_revision already exists
if grep -q "source_revision:" "$SOURCE_FILE"; then
    # Update existing source_revision
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        sed -i '' "s/source_revision: \"[^\"]*\"/source_revision: \"$GIT_COMMIT\"/" "$SOURCE_FILE"
    else
        # Linux
        sed -i "s/source_revision: \"[^\"]*\"/source_revision: \"$GIT_COMMIT\"/" "$SOURCE_FILE"
    fi
    echo -e "${GREEN}✓${NC} Updated source_revision in $SOURCE_FILE"
else
    # Add source_revision after source_code line
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        sed -i '' "/source_code:/a\\
    source_revision: \"$GIT_COMMIT\",
" "$SOURCE_FILE"
    else
        # Linux
        sed -i "/source_code:/a\\    source_revision: \"$GIT_COMMIT\"," "$SOURCE_FILE"
    fi
    echo -e "${GREEN}✓${NC} Added source_revision to $SOURCE_FILE"
fi

# Verify the change
if grep -q "source_revision: \"$GIT_COMMIT\"" "$SOURCE_FILE"; then
    echo -e "${GREEN}✓${NC} Verification passed"
else
    echo -e "${RED}✗${NC} Verification failed - source_revision not updated correctly"
    exit 1
fi

