#!/bin/bash

# Monitor Mainframe program events in real-time
# Usage: ./scripts/monitor-events.sh [mainnet|devnet]

NETWORK=${1:-mainnet}
PROGRAM_ID="mnfm211AwTDA8fGvPezYs3jjxAXgoucHGuTMUbjFssE"

if [ "$NETWORK" = "mainnet" ]; then
  RPC_URL=${SOLANA_RPC_URL:-"https://api.mainnet-beta.solana.com"}
else
  RPC_URL="https://api.devnet.solana.com"
fi

echo "🔍 Monitoring Mainframe events on $NETWORK"
echo "📡 RPC: $RPC_URL"
echo "📋 Program: $PROGRAM_ID"
echo ""
echo "Watching for events... (Press Ctrl+C to stop)"
echo "═══════════════════════════════════════════════════════"
echo ""

solana logs $PROGRAM_ID --url $RPC_URL

