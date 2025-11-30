import { Connection, PublicKey } from "@solana/web3.js";
import { AnchorProvider, Program } from "@coral-xyz/anchor";
import * as fs from "fs";

const MAINFRAME_PROGRAM_ID = new PublicKey(
  "mnfm211AwTDA8fGvPezYs3jjxAXgoucHGuTMUbjFssE"
);
const NFT_MINT = new PublicKey("pszD291HiBAYTVpBDSe6BqfBDVNxJ9PbAWzF5SPXZai");

async function main() {
  const rpcUrl =
    process.env.SOLANA_RPC_URL || "https://api.mainnet-beta.solana.com";
  const connection = new Connection(rpcUrl, "confirmed");

  const idl = JSON.parse(
    fs.readFileSync(__dirname + "/../target/idl/mainframe.json", "utf8")
  );
  const program = new Program(
    idl,
    new AnchorProvider(connection, { publicKey: PublicKey.default } as any, {})
  );

  const [agentPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("agent"), NFT_MINT.toBuffer()],
    MAINFRAME_PROGRAM_ID
  );

  console.log("\n🔍 Checking agent format...");
  console.log("Agent PDA:", agentPDA.toBase58());
  console.log("");

  try {
    // @ts-ignore
    const agent = await program.account.agentAccount.fetch(agentPDA);

    console.log("✅ AGENT ACCOUNT (NEW FORMAT)\n");
    console.log("NFT Mint:", agent.nftMint.toBase58());
    console.log("Owner:", agent.owner.toBase58());
    console.log("Collection:", agent.collectionMint?.toBase58() || "None");
    console.log("Agent-NFT:", agent.agentNft?.toBase58() || "None");
    console.log("Status:", JSON.stringify(agent.status));
    console.log("Metadata URI:", agent.metadataUri);
    console.log("Version:", agent.version.toString());
    console.log(
      "Activated At:",
      new Date(agent.activatedAt.toNumber() * 1000).toISOString()
    );
    console.log("");

    console.log("📊 Format Verification:");
    console.log(
      "- Has agent_nft field:",
      agent.agentNft !== undefined ? "✅ YES" : "❌ NO"
    );
    console.log(
      "- agent_nft value:",
      agent.agentNft ? agent.agentNft.toBase58() : "null (Pending)"
    );
    console.log(
      "- Status is Pending:",
      agent.status.pending !== undefined ? "✅ YES" : "❌ NO"
    );
    console.log(
      "- Proper new format:",
      agent.agentNft !== undefined
        ? "✅ YES - Ready for Agent-NFT minting"
        : "❌ NO - Old format"
    );
    console.log("");

    if (agent.status.pending) {
      console.log("🟡 Status: PENDING");
      console.log("   Awaiting Agent-NFT to be minted");
      console.log("   Once Agent-NFT is minted, status will become Active");
    } else if (agent.status.active) {
      console.log("🟢 Status: ACTIVE");
      console.log("   Agent is running");
    }
  } catch (error) {
    console.error("❌ Failed to fetch agent:");
    console.error(error instanceof Error ? error.message : error);
    console.error("\nAgent might not exist or is in old format");
  }
}

main();
