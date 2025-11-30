/**
 * Close legacy agent account (old format without agent_nft field)
 *
 * Usage:
 *   ts-node scripts/close-legacy-agent.ts <agent_account> [--network mainnet|devnet]
 */

import { Connection, PublicKey, Keypair } from "@solana/web3.js";
import { AnchorProvider, Program, Wallet } from "@coral-xyz/anchor";
import * as fs from "fs";

const MAINFRAME_PROGRAM_ID = new PublicKey(
  "mnfm211AwTDA8fGvPezYs3jjxAXgoucHGuTMUbjFssE"
);

async function main() {
  const args = process.argv.slice(2);

  if (args.length < 1) {
    console.log(
      "Usage: ts-node scripts/close-legacy-agent.ts <agent_account> [--network mainnet|devnet]"
    );
    console.log("");
    console.log("Example:");
    console.log(
      "  yarn ts-node scripts/close-legacy-agent.ts 9Aa8fgJ7Hm1guab1L5Bs1tNxhLVVRa2SbsJNeAcvDJaJ --network mainnet"
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

  console.log(`🔴 Closing legacy agent on ${network}...`);
  console.log(`📡 RPC: ${rpcUrl}`);
  console.log(`🎯 Agent: ${agentAccountAddr}`);
  console.log("");

  const connection = new Connection(rpcUrl, "confirmed");

  // Load owner keypair
  const keypairPath =
    process.env.AUTHORITY_KEYPAIR_PATH ||
    `${process.env.HOME}/.config/solana/id.json`;

  console.log(`🔑 Loading keypair from: ${keypairPath}`);
  const keypairData = JSON.parse(fs.readFileSync(keypairPath, "utf8"));
  const owner = Keypair.fromSecretKey(new Uint8Array(keypairData));

  console.log(`🔑 Owner: ${owner.publicKey.toBase58()}`);
  console.log("");

  // Load IDL
  const idlPath = __dirname + "/../target/idl/mainframe.json";
  const idl = JSON.parse(fs.readFileSync(idlPath, "utf8"));

  const wallet = new Wallet(owner);
  const provider = new AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });

  // @ts-ignore
  const program = new Program(idl, provider);

  const agentAccountPubkey = new PublicKey(agentAccountAddr);

  // Derive protocol config PDA
  const [protocolConfigPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("protocol_config")],
    MAINFRAME_PROGRAM_ID
  );

  try {
    console.log("🔴 Closing legacy agent...");

    const tx = await program.methods
      .closeLegacyAgent()
      .accounts({
        agentAccount: agentAccountPubkey,
        owner: owner.publicKey,
        protocolConfig: protocolConfigPDA,
      })
      .rpc();

    console.log("");
    console.log("✅ Legacy agent closed successfully!");
    console.log(`Signature: ${tx}`);
    console.log(
      `Explorer: https://explorer.solana.com/tx/${tx}${
        network === "mainnet" ? "" : "?cluster=devnet"
      }`
    );
    console.log("");
    console.log("ℹ️  Rent refunded to your wallet");
    console.log("ℹ️  NFT is now free to create a new agent");
  } catch (error) {
    console.error("");
    console.error("❌ Failed to close legacy agent:");
    console.error(error);
    process.exit(1);
  }
}

main();
