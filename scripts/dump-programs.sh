#!/bin/bash

# Script to dump required programs for local testing
# This allows testing with Metaplex and other dependencies locally

set -e

PROGRAM_DIR="./test-programs"
mkdir -p "$PROGRAM_DIR"

echo "📦 Dumping programs from mainnet for local testing..."

# Metaplex Token Metadata Program
METAPLEX_METADATA="metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
echo "Dumping Metaplex Token Metadata ($METAPLEX_METADATA)..."
solana program dump -u m "$METAPLEX_METADATA" "$PROGRAM_DIR/mpl_token_metadata.so"

echo ""
echo "✅ Programs dumped successfully!"
echo ""
echo "Programs saved to:"
echo "  - $PROGRAM_DIR/mpl_token_metadata.so"
echo ""
echo "To use these in tests, run: anchor test"
echo "The validator will automatically load these programs from Anchor.toml"

