/**
 * Close an agent account
 *
 * Usage:
 *   ts-node scripts/close-agent.ts <agent_account> [--network mainnet|devnet]
 */

import { Connection, PublicKey, Keypair } from "@solana/web3.js";
import { AnchorProvider, Program, Wallet } from "@coral-xyz/anchor";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
import * as fs from "fs";

const MAINFRAME_PROGRAM_ID = new PublicKey(
  "mnfm211AwTDA8fGvPezYs3jjxAXgoucHGuTMUbjFssE"
);

async function main() {
  const args = process.argv.slice(2);

  if (args.length < 1) {
    console.log(
      "Usage: ts-node scripts/close-agent.ts <agent_account> [--network mainnet|devnet]"
    );
    process.exit(1);
  }

  const agentAccountAddr = args[0];
  const network = args.includes("--network")
    ? args[args.indexOf("--network") + 1]
    : "mainnet";

  const rpcUrl =
    network === "mainnet"
      ? process.env.SOLANA_RPC_URL || "https://api.mainnet-beta.solana.com"
      : "https://api.devnet.solana.com";

  console.log(`🔴 Closing agent on ${network}...`);
  console.log(`📡 RPC: ${rpcUrl}`);
  console.log(`🎯 Agent: ${agentAccountAddr}`);
  console.log("");

  const connection = new Connection(rpcUrl, "confirmed");

  // Load owner keypair
  const keypairPath = `${process.env.HOME}/.config/solana/id.json`;
  const keypairData = JSON.parse(fs.readFileSync(keypairPath, "utf8"));
  const owner = Keypair.fromSecretKey(new Uint8Array(keypairData));

  console.log(`🔑 Owner: ${owner.publicKey.toBase58()}`);

  // Load IDL
  const idlPath = __dirname + "/../target/idl/mainframe.json";
  const idl = JSON.parse(fs.readFileSync(idlPath, "utf8"));

  const wallet = new Wallet(owner);
  const provider = new AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });

  // @ts-ignore
  const program = new Program(idl, provider);

  try {
    // Fetch agent account to get NFT mint
    console.log("📜 Fetching agent data...");
    const agentAccountPubkey = new PublicKey(agentAccountAddr);

    // Get account data directly
    const accountInfo = await connection.getAccountInfo(agentAccountPubkey);

    if (!accountInfo) {
      throw new Error("Agent account not found");
    }

    // Parse NFT mint from account data (bytes 8-40)
    const nftMintBytes = accountInfo.data.slice(8, 40);
    const nftMint = new PublicKey(nftMintBytes);

    console.log(`   NFT Mint: ${nftMint.toBase58()}`);

    // Get NFT token account
    const nftTokenAccount = getAssociatedTokenAddressSync(
      nftMint,
      owner.publicKey
    );

    console.log(`   NFT Token Account: ${nftTokenAccount.toBase58()}`);

    // Derive protocol config PDA
    const [protocolConfigPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("protocol_config")],
      MAINFRAME_PROGRAM_ID
    );

    console.log("");
    console.log("🔴 Closing agent...");

    const tx = await program.methods
      .closeAgent()
      .accounts({
        agentAccount: agentAccountPubkey,
        nftTokenAccount,
        owner: owner.publicKey,
        protocolConfig: protocolConfigPDA,
      })
      .rpc();

    console.log("");
    console.log("✅ Agent closed successfully!");
    console.log(`Signature: ${tx}`);
    console.log(
      `Explorer: https://explorer.solana.com/tx/${tx}${
        network === "mainnet" ? "" : "?cluster=devnet"
      }`
    );
    console.log("");
    console.log("ℹ️  Agent status set to Closed");
    console.log("ℹ️  NFT is now free to create a new agent");
  } catch (error) {
    console.error("");
    console.error("❌ Failed to close agent:", error);
    process.exit(1);
  }
}

main();
