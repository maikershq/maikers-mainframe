import { Connection, PublicKey, Keypair } from "@solana/web3.js";
import { AnchorProvider, Program, Wallet } from "@coral-xyz/anchor";
import * as fs from "fs";

const MAINFRAME_PROGRAM_ID = new PublicKey(
  "mnfm211AwTDA8fGvPezYs3jjxAXgoucHGuTMUbjFssE"
);
const AGENT_ACCOUNT = new PublicKey(
  "9Aa8fgJ7Hm1guab1L5Bs1tNxhLVVRa2SbsJNeAcvDJaJ"
);

async function main() {
  const rpcUrl =
    process.env.SOLANA_RPC_URL || "https://api.mainnet-beta.solana.com";
  const connection = new Connection(rpcUrl, "confirmed");

  const keypairPath = `${process.env.HOME}/.config/solana/id.json`;
  const keypairData = JSON.parse(fs.readFileSync(keypairPath, "utf8"));
  const authority = Keypair.fromSecretKey(new Uint8Array(keypairData));

  const idlPath = __dirname + "/../target/idl/mainframe.json";
  const idl = JSON.parse(fs.readFileSync(idlPath, "utf8"));

  const wallet = new Wallet(authority);
  const provider = new AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });
  const program = new Program(idl, provider);

  const [protocolConfigPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("protocol_config")],
    MAINFRAME_PROGRAM_ID
  );

  console.log("🔄 Migrating agent:", AGENT_ACCOUNT.toBase58());
  console.log("");

  try {
    const tx = await program.methods
      .migrateAgent()
      .accounts({
        agentAccount: AGENT_ACCOUNT,
        protocolConfig: protocolConfigPDA,
        signer: authority.publicKey,
      })
      .rpc();

    console.log("✅ Migration successful!");
    console.log("Signature:", tx);
    console.log("");
    console.log("Explorer:", `https://explorer.solana.com/tx/${tx}`);
  } catch (error) {
    console.error("❌ Migration failed:", error);
    process.exit(1);
  }
}

main();
