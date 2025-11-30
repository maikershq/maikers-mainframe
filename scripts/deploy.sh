#!/bin/bash

# Deployment Script with Devnet-First Policy
# Always deploys to devnet first, then mainnet

set -e

PROGRAM_ID="mnfm211AwTDA8fGvPezYs3jjxAXgoucHGuTMUbjFssE"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$SCRIPT_DIR/.."

echo "🚀 Mainframe Deployment Script"
echo "═══════════════════════════════════════════════════════"
echo ""
echo "⚠️  DEVNET-FIRST POLICY: Always deploy to devnet first!"
echo ""

# Build
echo "🔨 Step 1/4: Building program..."
cd "$PROJECT_DIR"
anchor build

if [ $? -ne 0 ]; then
  echo "❌ Build failed"
  exit 1
fi

echo "✅ Build successful"
echo ""

# Deploy to devnet FIRST
echo "🧪 Step 2/4: Deploying to DEVNET..."
solana program deploy target/deploy/mainframe.so \
  --url devnet \
  --program-id $PROGRAM_ID

if [ $? -ne 0 ]; then
  echo "❌ Devnet deployment failed"
  exit 1
fi

echo "✅ Devnet deployment successful"
echo ""

# Confirm before mainnet
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "⚠️  Ready to deploy to MAINNET"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
read -p "Continue with mainnet deployment? (yes/no): " confirm

if [ "$confirm" != "yes" ]; then
  echo "❌ Mainnet deployment cancelled"
  exit 0
fi

# Deploy to mainnet
echo ""
echo "🌐 Step 3/4: Deploying to MAINNET..."
solana program deploy target/deploy/mainframe.so \
  --url mainnet-beta \
  --program-id $PROGRAM_ID

if [ $? -ne 0 ]; then
  echo "❌ Mainnet deployment failed"
  exit 1
fi

echo "✅ Mainnet deployment successful"
echo ""

# Sync IDL to web app
echo "🔄 Step 4/4: Syncing IDL to web app..."
cd "$PROJECT_DIR/../maikers-mainframe-web"
./scripts/sync-idl.sh

echo ""
echo "🎉 DEPLOYMENT COMPLETE!"
echo "═══════════════════════════════════════════════════════"
echo "✅ Devnet deployed"
echo "✅ Mainnet deployed"
echo "✅ IDL synced to web app"
echo ""
echo "Next steps:"
echo "1. Test on devnet"
echo "2. Run migration if needed"
echo "3. Deploy web app"

