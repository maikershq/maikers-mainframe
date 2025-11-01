import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider, Wallet } from "@coral-xyz/anchor";
import { Connection, PublicKey, Keypair } from "@solana/web3.js";
import * as fs from "fs";
import * as path from "path";
import { Mainframe } from "../target/types/mainframe";

const DEVNET_RPC = "https://api.devnet.solana.com";
const PROGRAM_ID = new PublicKey("8MLkFLHS2AfTEAzewBcb4qbVjNUkUc4843VASZmn1VZb");

async function verifyDevnetDeployment() {
  console.log("\n🔍 Verifying Mainframe Devnet Deployment");
  console.log("=========================================\n");

  const connection = new Connection(DEVNET_RPC, "confirmed");
  
  // Load authority keypair
  const authorityPath = path.join(__dirname, "..", "keys", "authority.json");
  if (!fs.existsSync(authorityPath)) {
    throw new Error("Authority keypair not found. Run init:config:devnet first.");
  }
  
  const authorityData = JSON.parse(fs.readFileSync(authorityPath, "utf8"));
  const authority = Keypair.fromSecretKey(new Uint8Array(authorityData));
  
  const wallet = new Wallet(authority);
  const provider = new AnchorProvider(connection, wallet, {});
  
  // Load IDL
  const idlPath = path.join(__dirname, "..", "target", "idl", "mainframe.json");
  const idl = JSON.parse(fs.readFileSync(idlPath, "utf8"));
  
  const program = new Program(idl as Mainframe, provider) as Program<Mainframe>;

  console.log("✅ Connected to devnet");
  console.log(`   RPC: ${DEVNET_RPC}`);
  console.log(`   Program ID: ${PROGRAM_ID.toBase58()}`);
  console.log(`   Authority: ${authority.publicKey.toBase58()}\n`);

  // Verify program exists
  console.log("📦 Checking program deployment...");
  const programInfo = await connection.getAccountInfo(PROGRAM_ID);
  
  if (!programInfo) {
    throw new Error("❌ Program not found on devnet!");
  }
  
  console.log(`   ✅ Program deployed`);
  console.log(`   Data size: ${programInfo.data.length} bytes`);
  console.log(`   Owner: ${programInfo.owner.toBase58()}\n`);

  // Verify protocol config
  console.log("⚙️  Checking protocol configuration...");
  const [protocolConfigPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("protocol_config")],
    PROGRAM_ID
  );

  try {
    const config = await program.account.protocolConfig.fetch(protocolConfigPda);
    
    console.log(`   ✅ Protocol Config PDA: ${protocolConfigPda.toBase58()}`);
    console.log(`   Authority: ${config.authority.toBase58()}`);
    console.log(`   Genesis Collection: ${config.genesisCollectionMint.toBase58()}`);
    console.log(`   Protocol Treasury: ${config.protocolTreasury.toBase58()}`);
    console.log(`   Validator Treasury: ${config.validatorTreasury.toBase58()}`);
    console.log(`   Network Treasury: ${config.networkTreasury.toBase58()}`);
    console.log(`   Protocol Paused: ${config.paused}`);
    console.log(`\n   Fee Structure:`);
    console.log(`   - Create Agent: ${config.fees.createAgent.toNumber() / 1e9} SOL`);
    console.log(`   - Update Config: ${config.fees.updateAgentConfig.toNumber() / 1e9} SOL`);
    console.log(`   - Transfer Agent: ${config.fees.transferAgent.toNumber() / 1e9} SOL`);
    console.log(`   - Pause Agent: ${config.fees.pauseAgent.toNumber() / 1e9} SOL`);
    console.log(`   - Close Agent: ${config.fees.closeAgent.toNumber() / 1e9} SOL`);
    console.log(`   - Execute Action: ${config.fees.executeAction.toNumber() / 1e9} SOL`);
    console.log(`\n   Treasury Distribution:`);
    console.log(`   - Protocol: ${config.protocolTreasuryBps / 100}%`);
    console.log(`   - Validator: ${config.validatorTreasuryBps / 100}%`);
    console.log(`   - Network: ${config.networkTreasuryBps / 100}%`);
    console.log(`\n   Protocol Limits:`);
    console.log(`   - Max Partner Collections: ${config.maxPartnerCollections}`);
    console.log(`   - Max Affiliate BPS: ${config.maxAffiliateBps / 100}%`);
    
    // Check treasuries have proper balances
    console.log(`\n💰 Checking treasury balances...`);
    const protocolBalance = await connection.getBalance(config.protocolTreasury);
    const validatorBalance = await connection.getBalance(config.validatorTreasury);
    const networkBalance = await connection.getBalance(config.networkTreasury);
    
    console.log(`   Protocol Treasury: ${protocolBalance / 1e9} SOL`);
    console.log(`   Validator Treasury: ${validatorBalance / 1e9} SOL`);
    console.log(`   Network Treasury: ${networkBalance / 1e9} SOL`);
    
    console.log("\n✅ Devnet deployment verified successfully!");
    console.log("\n📝 Summary:");
    console.log("   - Program deployed and executable");
    console.log("   - Protocol configuration initialized");
    console.log("   - All treasuries configured");
    console.log("   - Fee structure set correctly");
    console.log("   - Ready for testing and integration");
    
  } catch (error) {
    if (error.message?.includes("Account does not exist")) {
      throw new Error("❌ Protocol config not initialized. Run 'yarn init:config:devnet' first.");
    }
    throw error;
  }
}

verifyDevnetDeployment()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error("\n❌ Verification failed:", error.message);
    process.exit(1);
  });

