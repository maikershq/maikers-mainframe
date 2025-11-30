/**
 * Migrate all existing agents to Pending status
 *
 * This script:
 * 1. Fetches all agent accounts from the program
 * 2. For each agent with agent_nft = None, calls migrate_agent
 * 3. Updates status to Pending (awaiting Agent-NFT mint)
 *
 * Usage:
 *   ts-node scripts/migrate-agents-to-pending.ts [--network mainnet|devnet] [--dry-run]
 */

import { Connection, PublicKey, Keypair } from "@solana/web3.js";
import { AnchorProvider, Program, Wallet } from "@coral-xyz/anchor";
import { Mainframe } from "../target/types/mainframe";
import * as fs from "fs";

const MAINFRAME_PROGRAM_ID = new PublicKey(
  "mnfm211AwTDA8fGvPezYs3jjxAXgoucHGuTMUbjFssE"
);

interface MigrateOptions {
  network: "mainnet" | "devnet";
  dryRun: boolean;
}

async function parseArgs(): Promise<MigrateOptions> {
  const args = process.argv.slice(2);
  const options: MigrateOptions = {
    network: "mainnet",
    dryRun: false,
  };

  for (let i = 0; i < args.length; i++) {
    if (args[i] === "--network" && args[i + 1]) {
      options.network = args[i + 1] as "mainnet" | "devnet";
      i++;
    } else if (args[i] === "--dry-run") {
      options.dryRun = true;
    }
  }

  return options;
}

async function migrateAgents(options: MigrateOptions) {
  const rpcUrl =
    options.network === "mainnet"
      ? process.env.SOLANA_RPC_URL || "https://api.mainnet-beta.solana.com"
      : "https://api.devnet.solana.com";

  console.log(`🔄 Migrating agents on ${options.network}...`);
  console.log(`📡 RPC: ${rpcUrl}`);
  if (options.dryRun) console.log(`🧪 DRY RUN - No transactions will be sent`);
  console.log("");

  const connection = new Connection(rpcUrl, "confirmed");

  // Load authority keypair
  const keypairPath =
    process.env.AUTHORITY_KEYPAIR_PATH ||
    `${process.env.HOME}/.config/solana/id.json`;
  const keypairData = JSON.parse(fs.readFileSync(keypairPath, "utf8"));
  const authority = Keypair.fromSecretKey(new Uint8Array(keypairData));

  console.log(`🔑 Authority: ${authority.publicKey.toBase58()}`);
  console.log("");

  // Load IDL
  const idlPath = __dirname + "/../target/idl/mainframe.json";
  const idl = JSON.parse(fs.readFileSync(idlPath, "utf8"));

  const wallet = new Wallet(authority);
  const provider = new AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });

  // @ts-ignore
  const program = new Program(idl, provider);

  // Fetch all agent accounts (try to find any agent account)
  console.log("📜 Fetching all agent accounts...");

  // Try different account sizes (old and new format)
  let accounts = await connection.getProgramAccounts(MAINFRAME_PROGRAM_ID);

  // Filter for agent accounts (discriminator check)
  const agentDiscriminator = [241, 119, 69, 140, 233, 9, 112, 50];
  accounts = accounts.filter((account) => {
    const data = account.account.data;
    if (data.length < 8) return false;

    for (let i = 0; i < 8; i++) {
      if (data[i] !== agentDiscriminator[i]) return false;
    }
    return true;
  });

  console.log(`✅ Found ${accounts.length} agent accounts\n`);

  if (accounts.length === 0) {
    console.log("No agents found to migrate.");
    return;
  }

  // Derive protocol config PDA
  const [protocolConfigPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("protocol_config")],
    MAINFRAME_PROGRAM_ID
  );

  let migratedCount = 0;
  let skippedCount = 0;
  let errorCount = 0;

  console.log("═".repeat(60));

  // Migrate each agent
  for (let i = 0; i < accounts.length; i++) {
    const account = accounts[i];

    try {
      const accountData = account.account.data;
      const accountSize = accountData.length;

      console.log(
        `\n${i + 1}/${accounts.length} Agent: ${account.pubkey.toBase58()}`
      );
      console.log(`   Account size: ${accountSize} bytes`);

      // Parse basic info from raw account data (works for both old and new formats)
      const nftMintBytes = accountData.slice(8, 40);
      const ownerBytes = accountData.slice(40, 72);
      const nftMint = new PublicKey(nftMintBytes);
      const owner = new PublicKey(ownerBytes);

      console.log(`   NFT Mint: ${nftMint.toBase58()}`);
      console.log(`   Owner: ${owner.toBase58()}`);

      // Old format: 165 bytes (no agent_nft field)
      // New format: 198 bytes (with agent_nft field)
      const expectedNewSize = 198;

      if (accountSize >= expectedNewSize) {
        // Try to deserialize to check status
        try {
          // @ts-ignore
          const agentData = await program.account.agentAccount.fetch(
            account.pubkey
          );
          const isPending = agentData.status.pending !== undefined;

          if (isPending) {
            console.log(`   ⏭️  Skipped - Already Pending`);
            skippedCount++;
            continue;
          }

          console.log(
            `   🎯 Needs status update: ${JSON.stringify(
              agentData.status
            )} → Pending`
          );
        } catch (e) {
          console.log(
            `   ⚠️  New format but can't deserialize, will migrate anyway`
          );
        }
      } else {
        console.log(`   🎯 Old format - needs realloc + status update`);
      }

      if (options.dryRun) {
        console.log(`   🧪 Would migrate to Pending`);
        migratedCount++;
        continue;
      }

      // Migrate agent
      console.log(`   🔄 Migrating to Pending...`);

      const tx = await program.methods
        .migrateAgent()
        .accounts({
          agentAccount: account.pubkey,
          protocolConfig: protocolConfigPDA,
          signer: authority.publicKey,
        })
        .rpc();

      console.log(`   ✅ Migrated! Signature: ${tx}`);
      migratedCount++;

      // Rate limit to avoid overwhelming RPC
      await new Promise((resolve) => setTimeout(resolve, 500));
    } catch (error) {
      console.log(
        `   ❌ Error: ${error instanceof Error ? error.message : error}`
      );
      errorCount++;
    }
  }

  console.log("\n" + "═".repeat(60));
  console.log(`\n📊 Migration Summary:`);
  console.log(`   Total agents: ${accounts.length}`);
  console.log(`   Migrated: ${migratedCount}`);
  console.log(`   Skipped: ${skippedCount}`);
  console.log(`   Errors: ${errorCount}`);

  if (options.dryRun) {
    console.log(`\n🧪 DRY RUN complete - no changes made`);
    console.log(`   Run without --dry-run to apply changes`);
  } else {
    console.log(`\n✅ Migration complete!`);
  }
}

async function main() {
  try {
    const options = await parseArgs();
    await migrateAgents(options);
  } catch (error) {
    console.error("❌ Migration failed:", error);
    process.exit(1);
  }
}

main();
