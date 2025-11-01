import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider, Wallet } from "@coral-xyz/anchor";
import { Connection, PublicKey, Keypair } from "@solana/web3.js";
import * as fs from "fs";
import * as path from "path";
import { Mainframe } from "../target/types/mainframe";

/**
 * Update Treasury Addresses Script
 * 
 * This script allows the protocol authority to update the three treasury addresses.
 * 
 * SECURITY WARNING: Only the protocol authority can execute this operation.
 * Triple-check all addresses before executing on mainnet.
 */

interface UpdateConfig {
  cluster: string;
  newProtocolTreasury: string;
  newValidatorTreasury: string;
  newNetworkTreasury: string;
  dryRun?: boolean;
}

async function updateTreasuryAddresses(config: UpdateConfig) {
  console.log("\n🔐 Treasury Address Update");
  console.log("==========================\n");

  // Validate all three addresses are different
  const addresses = [
    config.newProtocolTreasury,
    config.newValidatorTreasury,
    config.newNetworkTreasury,
  ];
  
  const uniqueAddresses = new Set(addresses);
  if (uniqueAddresses.size !== 3) {
    throw new Error("❌ ERROR: All three treasury addresses must be different!");
  }

  // Setup connection
  const rpcUrl = config.cluster === "mainnet" 
    ? "https://api.mainnet-beta.solana.com"
    : config.cluster === "devnet"
    ? "https://api.devnet.solana.com"
    : "http://127.0.0.1:8899";
  
  const connection = new Connection(rpcUrl, "confirmed");
  
  // Load authority keypair
  const authorityPath = path.join(__dirname, "..", "keys", "authority.json");
  if (!fs.existsSync(authorityPath)) {
    throw new Error("Authority keypair not found. Cannot proceed.");
  }
  
  const authorityData = JSON.parse(fs.readFileSync(authorityPath, "utf8"));
  const authority = Keypair.fromSecretKey(new Uint8Array(authorityData));
  
  const wallet = new Wallet(authority);
  const provider = new AnchorProvider(connection, wallet, {});
  
  // Load IDL and create program instance
  const idlPath = path.join(__dirname, "..", "target", "idl", "mainframe.json");
  const idl = JSON.parse(fs.readFileSync(idlPath, "utf8"));
  const program = new Program(idl as Mainframe, provider) as Program<Mainframe>;
  
  const PROGRAM_ID = program.programId;
  
  console.log(`📡 Cluster: ${config.cluster}`);
  console.log(`   RPC: ${rpcUrl}`);
  console.log(`   Program ID: ${PROGRAM_ID.toBase58()}`);
  console.log(`   Authority: ${authority.publicKey.toBase58()}\n`);

  // Derive protocol config PDA
  const [protocolConfigPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("protocol_config")],
    PROGRAM_ID
  );

  // Fetch current config
  console.log("📋 Current Treasury Addresses:");
  const currentConfig = await program.account.protocolConfig.fetch(protocolConfigPda);
  console.log(`   Protocol:  ${currentConfig.protocolTreasury.toBase58()}`);
  console.log(`   Validator: ${currentConfig.validatorTreasury.toBase58()}`);
  console.log(`   Network:   ${currentConfig.networkTreasury.toBase58()}\n`);

  // Validate authority
  if (!currentConfig.authority.equals(authority.publicKey)) {
    throw new Error(
      `❌ ERROR: Authority mismatch!\n` +
      `   Expected: ${currentConfig.authority.toBase58()}\n` +
      `   Provided: ${authority.publicKey.toBase58()}`
    );
  }

  // Parse new addresses
  const newProtocolTreasury = new PublicKey(config.newProtocolTreasury);
  const newValidatorTreasury = new PublicKey(config.newValidatorTreasury);
  const newNetworkTreasury = new PublicKey(config.newNetworkTreasury);

  console.log("🎯 New Treasury Addresses:");
  console.log(`   Protocol:  ${newProtocolTreasury.toBase58()}`);
  console.log(`   Validator: ${newValidatorTreasury.toBase58()}`);
  console.log(`   Network:   ${newNetworkTreasury.toBase58()}\n`);

  // Validate addresses are valid and exist
  console.log("🔍 Validating new addresses...");
  const [protocolInfo, validatorInfo, networkInfo] = await Promise.all([
    connection.getAccountInfo(newProtocolTreasury),
    connection.getAccountInfo(newValidatorTreasury),
    connection.getAccountInfo(newNetworkTreasury),
  ]);

  // Note: It's OK if accounts don't exist yet (they'll be created on first SOL transfer)
  console.log(`   Protocol:  ${protocolInfo ? "✅ Exists" : "⚠️  New (will be created)"}`);
  console.log(`   Validator: ${validatorInfo ? "✅ Exists" : "⚠️  New (will be created)"}`);
  console.log(`   Network:   ${networkInfo ? "✅ Exists" : "⚠️  New (will be created)"}\n`);

  // Dry run check
  if (config.dryRun) {
    console.log("🔍 DRY RUN MODE - No changes will be made");
    console.log("\nValidation complete! All addresses are valid.\n");
    console.log("To execute the update, run again without --dry-run flag.\n");
    return;
  }

  // Final confirmation for mainnet
  if (config.cluster === "mainnet") {
    console.log("⚠️  WARNING: You are about to update treasury addresses on MAINNET!");
    console.log("   Please triple-check all addresses above.");
    console.log("   This operation is IRREVERSIBLE.\n");
    console.log("   Press Ctrl+C to cancel, or continue in 5 seconds...\n");
    await new Promise(resolve => setTimeout(resolve, 5000));
  }

  // Execute update
  console.log("📤 Submitting treasury address update transaction...\n");

  try {
    const tx = await program.methods
      .updateTreasuryAddresses(
        newProtocolTreasury,
        newValidatorTreasury,
        newNetworkTreasury
      )
      .accountsPartial({
        protocolConfig: protocolConfigPda,
        authority: authority.publicKey,
        newProtocolTreasury: newProtocolTreasury,
        newValidatorTreasury: newValidatorTreasury,
        newNetworkTreasury: newNetworkTreasury,
      })
      .signers([authority])
      .rpc();

    console.log("✅ Treasury addresses updated successfully!");
    console.log(`   Transaction: ${tx}\n`);

    // Fetch and display new config
    console.log("📋 Updated Treasury Addresses:");
    const newConfig = await program.account.protocolConfig.fetch(protocolConfigPda);
    console.log(`   Protocol:  ${newConfig.protocolTreasury.toBase58()}`);
    console.log(`   Validator: ${newConfig.validatorTreasury.toBase58()}`);
    console.log(`   Network:   ${newConfig.networkTreasury.toBase58()}\n`);

    console.log("🎉 Update complete! Future fees will be distributed to the new addresses.");

  } catch (error: any) {
    console.error("\n❌ Transaction failed:");
    
    if (error.message?.includes("TreasuriesMustBeDifferent")) {
      console.error("   ERROR: All three treasury addresses must be different.");
    } else if (error.message?.includes("InvalidTreasuryAddress")) {
      console.error("   ERROR: One or more addresses are invalid (system program, config PDA, or program ID).");
    } else if (error.message?.includes("TreasuryAccountMismatch")) {
      console.error("   ERROR: Account mismatch - internal error, please report.");
    } else if (error.message?.includes("Unauthorized")) {
      console.error("   ERROR: Only the protocol authority can update treasury addresses.");
    } else {
      console.error("   ", error.message || error);
    }
    
    throw error;
  }
}

// CLI argument parsing
async function main() {
  const args = process.argv.slice(2);
  
  if (args.includes("--help") || args.includes("-h")) {
    console.log(`
Treasury Address Update Script

Usage: ts-node scripts/update-treasury-addresses.ts [options]

Options:
  --cluster <cluster>              Target cluster (localnet, devnet, mainnet) [required]
  --protocol <address>             New protocol treasury address [required]
  --validator <address>            New validator treasury address [required]
  --network <address>              New network treasury address [required]
  --dry-run                        Validate only, don't execute transaction
  --help, -h                       Show this help message

Examples:
  # Dry run on devnet (validate only)
  ts-node scripts/update-treasury-addresses.ts \\
    --cluster devnet \\
    --protocol ABC...xyz \\
    --validator DEF...xyz \\
    --network GHI...xyz \\
    --dry-run

  # Execute update on devnet
  ts-node scripts/update-treasury-addresses.ts \\
    --cluster devnet \\
    --protocol ABC...xyz \\
    --validator DEF...xyz \\
    --network GHI...xyz

Security:
  - Only the protocol authority can execute this operation
  - Authority keypair must be in keys/authority.json
  - All three addresses must be different
  - Mainnet updates have a 5-second confirmation delay
`);
    process.exit(0);
  }

  const config: UpdateConfig = {
    cluster: "",
    newProtocolTreasury: "",
    newValidatorTreasury: "",
    newNetworkTreasury: "",
    dryRun: false,
  };

  for (let i = 0; i < args.length; i++) {
    switch (args[i]) {
      case "--cluster":
        config.cluster = args[++i];
        break;
      case "--protocol":
        config.newProtocolTreasury = args[++i];
        break;
      case "--validator":
        config.newValidatorTreasury = args[++i];
        break;
      case "--network":
        config.newNetworkTreasury = args[++i];
        break;
      case "--dry-run":
        config.dryRun = true;
        break;
    }
  }

  // Validate required args
  if (!config.cluster || !config.newProtocolTreasury || 
      !config.newValidatorTreasury || !config.newNetworkTreasury) {
    console.error("❌ ERROR: Missing required arguments");
    console.error("   Run with --help to see usage\n");
    process.exit(1);
  }

  try {
    await updateTreasuryAddresses(config);
  } catch (error: any) {
    console.error("\n❌ Update failed:", error.message);
    process.exit(1);
  }
}

if (require.main === module) {
  main();
}

export { updateTreasuryAddresses };

