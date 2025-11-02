import * as anchor from "@coral-xyz/anchor";
import { Connection, PublicKey } from "@solana/web3.js";
import * as fs from "fs";
import * as path from "path";

async function fetchConfig() {
  const connection = new Connection('https://mainnet.helius-rpc.com/?api-key=d7e3151f-0aad-4b8a-9224-c3817e537e4a');
  const programId = new PublicKey('mnfm211AwTDA8fGvPezYs3jjxAXgoucHGuTMUbjFssE');
  
  // Load IDL
  const idlPath = path.join(__dirname, "..", "target", "idl", "mainframe.json");
  const idl = JSON.parse(fs.readFileSync(idlPath, "utf8"));
  
  // Create a dummy provider (read-only)
  const provider = new anchor.AnchorProvider(
    connection,
    { publicKey: PublicKey.default } as any,
    { commitment: 'confirmed' }
  );
  
  const program = new anchor.Program(idl, provider);
  
  // Derive protocol config PDA
  const [protocolConfigPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from('protocol_config')],
    programId
  );
  
  console.log('Protocol Config PDA:', protocolConfigPDA.toBase58());
  console.log('');
  
  try {
    const config = await program.account.protocolConfig.fetch(protocolConfigPDA);
    
    console.log('╔═══════════════════════════════════════════════════════════╗');
    console.log('║         MAINFRAME PROTOCOL CONFIGURATION                  ║');
    console.log('╚═══════════════════════════════════════════════════════════╝\n');
    
    console.log('🔐 AUTHORITY & MANAGEMENT');
    console.log('   Authority:', config.authority.toBase58());
    console.log('   Manager:', config.manager.toBase58());
    console.log('');
    
    console.log('💵 FEE STRUCTURE');
    console.log('   Create Agent:', config.fees.createAgent.toString(), 'lamports', `(${config.fees.createAgent / 1e9} SOL)`);
    console.log('   Update Agent Config:', config.fees.updateAgentConfig.toString(), 'lamports', `(${config.fees.updateAgentConfig / 1e9} SOL)`);
    console.log('   Transfer Agent:', config.fees.transferAgent.toString(), 'lamports', `(${config.fees.transferAgent / 1e9} SOL)`);
    console.log('   Pause Agent:', config.fees.pauseAgent.toString(), 'lamports');
    console.log('   Close Agent:', config.fees.closeAgent.toString(), 'lamports');
    console.log('   Execute Action:', config.fees.executeAction.toString(), 'lamports');
    console.log('');
    
    console.log('🏦 TREASURY DISTRIBUTION');
    console.log('   Protocol Treasury:', config.protocolTreasury.toBase58());
    console.log('      Share:', config.protocolTreasuryBps, `BPS (${config.protocolTreasuryBps / 100}%)`);
    console.log('');
    console.log('   Validator Treasury:', config.validatorTreasury.toBase58());
    console.log('      Share:', config.validatorTreasuryBps, `BPS (${config.validatorTreasuryBps / 100}%)`);
    console.log('');
    console.log('   Network Treasury:', config.networkTreasury.toBase58());
    console.log('      Share:', config.networkTreasuryBps, `BPS (${config.networkTreasuryBps / 100}%)`);
    console.log('');
    
    console.log('⚙️  PROTOCOL LIMITS');
    console.log('   Max Partner Collections:', config.maxPartnerCollections.toString());
    console.log('   Max Affiliate BPS:', config.maxAffiliateBps, `(${config.maxAffiliateBps / 100}%)`);
    console.log('');
    
    console.log('🎨 COLLECTIONS');
    console.log('   Genesis Collection Mint:', config.genesisCollectionMint.toBase58());
    console.log('   Partner Collections Count:', config.partnerCollections.length);
    console.log('');
    
    console.log('╔═══════════════════════════════════════════════════════════╗');
    console.log('║  ✅ PROTOCOL IS INITIALIZED AND OPERATIONAL               ║');
    console.log('╚═══════════════════════════════════════════════════════════╝');
    
  } catch (error: any) {
    console.error('❌ Error fetching config:', error.message);
    console.log('\n⚠️  Protocol may not be initialized');
    process.exit(1);
  }
}

fetchConfig().catch(console.error);
