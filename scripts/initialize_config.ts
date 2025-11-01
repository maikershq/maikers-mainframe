import * as anchor from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { Program, AnchorProvider, Wallet } from "@coral-xyz/anchor";
import { createMint, createAccount, mintTo } from "@solana/spl-token";
import * as fs from "fs";
import * as path from "path";
import bs58 from "bs58";

// Import the IDL type
import { Mainframe } from "../target/types/mainframe";

// Load IDL
const idlPath = path.join(__dirname, "..", "target", "idl", "mainframe.json");
const idl = JSON.parse(fs.readFileSync(idlPath, "utf8"));

/**
 * Parse Anchor.toml to extract program ID for the specified cluster
 */
function getProgramIdFromAnchorToml(cluster: string): PublicKey {
  const anchorTomlPath = path.join(__dirname, "..", "Anchor.toml");
  const anchorTomlContent = fs.readFileSync(anchorTomlPath, "utf8");

  // Simple TOML parsing for program IDs
  // Look for [programs.{cluster}] section
  const sectionRegex = new RegExp(`\\[programs\\.${cluster}\\]`, "i");
  const lines = anchorTomlContent.split("\n");

  let inProgramsSection = false;
  for (const line of lines) {
    const trimmedLine = line.trim();

    // Check if we're entering the correct programs section
    if (sectionRegex.test(trimmedLine)) {
      inProgramsSection = true;
      continue;
    }

    // If we hit another section, exit
    if (trimmedLine.startsWith("[") && inProgramsSection) {
      break;
    }

    // Look for mainframe = "program_id"
    if (inProgramsSection && trimmedLine.includes("mainframe")) {
      const match = trimmedLine.match(/mainframe\s*=\s*"([^"]+)"/);
      if (match && match[1]) {
        return new PublicKey(match[1]);
      }
    }
  }

  throw new Error(
    `Program ID for cluster '${cluster}' not found in Anchor.toml`
  );
}

// Constants
interface Config {
  cluster: string;
  authorityKeypairPath?: string;
  fees: {
    createAgent: number;
    updateAgentConfig: number;
    transferAgent: number;
    pauseAgent: number;
    closeAgent: number;
    executeAction: number;
  };
  treasuries?: {
    protocol?: PublicKey | string;
    validator?: PublicKey | string;
    network?: PublicKey | string;
  };
  treasuryKeypairPaths?: {
    protocol?: string;
    validator?: string;
    network?: string;
  };
  treasuryDistribution: {
    protocolBps: number;
    validatorBps: number;
    networkBps: number;
  };
  protocolLimits: {
    maxPartnerCollections: number;
    maxAffiliateBps: number;
  };
}

// Default configuration
const defaultConfig: Config = {
  cluster: "localnet",
  fees: {
    createAgent: 50000000, // 0.05 SOL - Agent activation fee
    updateAgentConfig: 5000000, // 0.005 SOL - Configuration updates
    transferAgent: 10000000, // 0.01 SOL - Ownership transfers
    pauseAgent: 0, // 0 SOL - Always free
    closeAgent: 0, // 0 SOL - Always free
    executeAction: 0, // 0 SOL - Always free
  },
  // treasuries: Optional - will be auto-generated if not provided
  // treasuryKeypairPaths: Optional - specify existing keypair paths
  treasuryDistribution: {
    protocolBps: 5000, // 50%
    validatorBps: 3000, // 30%
    networkBps: 2000, // 20%
  },
  protocolLimits: {
    maxPartnerCollections: 100, // Default: 100 partner collections max
    maxAffiliateBps: 5000, // Default: 50% maximum affiliate commission
  },
};

async function loadOrCreateKeypair(
  name: string,
  keypairPath?: string
): Promise<Keypair> {
  // First, check if a specific keypair path is provided
  if (keypairPath && fs.existsSync(keypairPath)) {
    console.log(`Loading ${name} keypair from: ${keypairPath}`);
    const keypairData = JSON.parse(fs.readFileSync(keypairPath, "utf8"));
    return Keypair.fromSecretKey(new Uint8Array(keypairData));
  }

  // If no specific path provided, check for existing keypair in keys directory
  const keysDir = path.join(__dirname, "..", "keys");
  const defaultKeyPath = path.join(keysDir, `${name}.json`);

  if (fs.existsSync(defaultKeyPath)) {
    console.log(`Loading existing ${name} keypair from: ${defaultKeyPath}`);
    const keypairData = JSON.parse(fs.readFileSync(defaultKeyPath, "utf8"));
    return Keypair.fromSecretKey(new Uint8Array(keypairData));
  }

  // Generate new keypair only if none exists
  console.log(`Generating new ${name} keypair...`);
  const keypair = Keypair.generate();

  // Create keys directory if it doesn't exist
  if (!fs.existsSync(keysDir)) {
    fs.mkdirSync(keysDir, { recursive: true });
  }

  fs.writeFileSync(
    defaultKeyPath,
    JSON.stringify(Array.from(keypair.secretKey), null, 2)
  );

  console.log(`${name} keypair saved to: ${defaultKeyPath}`);
  return keypair;
}

async function createGenesisCollection(
  connection: Connection,
  payer: Keypair,
  owner: PublicKey
): Promise<Keypair> {
  console.log("\n🎨 Setting up Genesis Collection reference...");

  const collectionMintKeypair = await loadOrCreateKeypair("genesis-collection");

  try {
    const accountInfo = await connection.getAccountInfo(
      collectionMintKeypair.publicKey
    );
    if (accountInfo) {
      console.log(
        `✅ Genesis collection mint exists on-chain: ${collectionMintKeypair.publicKey.toBase58()}`
      );
      return collectionMintKeypair;
    }
  } catch (error) {}

  console.log(
    `⚠️  Genesis collection mint not yet created on-chain: ${collectionMintKeypair.publicKey.toBase58()}`
  );
  console.log(
    `   This is OK for testing - the protocol only needs the address for configuration.`
  );
  console.log(
    `   You can create the actual NFT collection later using Metaplex.`
  );

  return collectionMintKeypair;
}

async function loadOrCreateAuthority(keypairPath?: string): Promise<Keypair> {
  return loadOrCreateKeypair("authority", keypairPath);
}

async function loadOrCreateTreasuries(config: Config): Promise<{
  protocol: Keypair;
  validator: Keypair;
  network: Keypair;
}> {
  const keysDir = path.join(__dirname, "..", "keys");

  // Load or create protocol treasury
  let protocol: Keypair;
  if (config.treasuryKeypairPaths?.protocol) {
    protocol = await loadOrCreateKeypair(
      "protocol-treasury",
      config.treasuryKeypairPaths.protocol
    );
  } else if (config.treasuries?.protocol) {
    // If public key provided but no keypair, we can't use it for signing
    console.log(
      `⚠️  Protocol treasury public key provided but no keypair path. Generating new keypair.`
    );
    protocol = await loadOrCreateKeypair("protocol-treasury");
  } else {
    protocol = await loadOrCreateKeypair("protocol-treasury");
  }

  // Load or create validator treasury
  let validator: Keypair;
  if (config.treasuryKeypairPaths?.validator) {
    validator = await loadOrCreateKeypair(
      "validator-treasury",
      config.treasuryKeypairPaths.validator
    );
  } else if (config.treasuries?.validator) {
    console.log(
      `⚠️  Validator treasury public key provided but no keypair path. Generating new keypair.`
    );
    validator = await loadOrCreateKeypair("validator-treasury");
  } else {
    validator = await loadOrCreateKeypair("validator-treasury");
  }

  // Load or create network treasury
  let network: Keypair;
  if (config.treasuryKeypairPaths?.network) {
    network = await loadOrCreateKeypair(
      "network-treasury",
      config.treasuryKeypairPaths.network
    );
  } else if (config.treasuries?.network) {
    console.log(
      `⚠️  Network treasury public key provided but no keypair path. Generating new keypair.`
    );
    network = await loadOrCreateKeypair("network-treasury");
  } else {
    network = await loadOrCreateKeypair("network-treasury");
  }

  return { protocol, validator, network };
}

function deriveProtocolConfigPDA(programId: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("protocol_config")],
    programId
  );
}

function deriveMaikersCredentialPDA(programId: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("maikers_credential")],
    programId
  );
}

function deriveMaikersSchemaInfo(
  authority: PublicKey,
  programId: PublicKey
): {
  schemaPDA: PublicKey;
  schemaMint: PublicKey;
} {
  // Derive schema PDA using the actual program ID
  const [schemaPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("maikers_schema"), authority.toBuffer()],
    programId
  );

  return {
    schemaPDA,
    schemaMint: authority, // In practice, this would be a proper mint
  };
}

async function initializeConfig(config: Config): Promise<void> {
  console.log("\n🚀 Initializing Mainframe Configuration");
  console.log("=====================================");

  // Show keypair reuse behavior
  const keysDir = path.join(__dirname, "..", "keys");
  const existingKeypairs = fs.existsSync(keysDir)
    ? fs
        .readdirSync(keysDir)
        .filter((f) => f.endsWith(".json") && !f.startsWith("backup-"))
    : [];

  if (existingKeypairs.length > 0) {
    console.log(
      `💾 Found ${existingKeypairs.length} existing keypairs - will reuse them:`
    );
    existingKeypairs.forEach((keypair) => console.log(`   • ${keypair}`));
  } else {
    console.log("🔑 No existing keypairs found - will generate new ones");
  }
  console.log("");

  // Get program ID dynamically from Anchor.toml
  const PROGRAM_ID = getProgramIdFromAnchorToml(config.cluster);

  // Setup connection and provider
  const connection = new Connection(
    config.cluster === "mainnet"
      ? "https://api.mainnet-beta.solana.com"
      : config.cluster === "devnet"
      ? "https://api.devnet.solana.com"
      : "http://127.0.0.1:8899"
  );

  const authority = await loadOrCreateAuthority(config.authorityKeypairPath);

  // Generate or load treasury keypairs based on config
  const treasuries = await loadOrCreateTreasuries(config);

  // Create or load genesis collection
  const genesisCollection = await createGenesisCollection(
    connection,
    authority,
    authority.publicKey
  );

  const wallet = new Wallet(authority);
  const provider = new AnchorProvider(connection, wallet, {});

  // Create program instance
  const program = new Program(idl as Mainframe, provider) as Program<Mainframe>;

  // Derive PDAs
  const [protocolConfigPDA, protocolConfigBump] =
    deriveProtocolConfigPDA(PROGRAM_ID);
  const [credentialPDA, credentialBump] =
    deriveMaikersCredentialPDA(PROGRAM_ID);
  const schemaInfo = deriveMaikersSchemaInfo(authority.publicKey, PROGRAM_ID);

  console.log("\n=== Mainframe Configuration ===");
  console.log(`Program ID: ${PROGRAM_ID.toBase58()}`);
  console.log(`Authority: ${authority.publicKey.toBase58()}`);
  console.log(`Genesis Collection: ${genesisCollection.publicKey.toBase58()}`);
  console.log(`Protocol Treasury: ${treasuries.protocol.publicKey.toBase58()}`);
  console.log(
    `Validator Treasury: ${treasuries.validator.publicKey.toBase58()}`
  );
  console.log(`Network Treasury: ${treasuries.network.publicKey.toBase58()}`);
  console.log(`Protocol Config PDA: ${protocolConfigPDA.toBase58()}`);
  console.log(`Credential PDA: ${credentialPDA.toBase58()}`);
  console.log(`Schema PDA: ${schemaInfo.schemaPDA.toBase58()}`);
  console.log(`Schema Mint: ${schemaInfo.schemaMint.toBase58()}`);

  // Check if already initialized
  try {
    const configAccount = await program.account.protocolConfig.fetch(
      protocolConfigPDA
    );
    console.log("\n⚠️  Protocol already initialized!");
    console.log(`Current authority: ${configAccount.authority.toBase58()}`);

    // Still output environment variables
    outputEnvironmentVariables(
      authority,
      genesisCollection.publicKey,
      credentialPDA,
      schemaInfo
    );

    console.log(
      "\n💡 Tip: Keypairs are automatically reused from the keys/ directory."
    );
    console.log("   Use --force-new-keypairs to generate new ones if needed.");
    return;
  } catch (error) {
    // Not initialized yet, proceed
    console.log("\n✅ Protocol not initialized, proceeding with setup...");
  }

  // Ensure authority has sufficient balance
  const balance = await connection.getBalance(authority.publicKey);
  const minBalance = 10000000; // 0.01 SOL

  if (balance < minBalance) {
    console.log(`\n💰 Authority balance too low (${balance / 1e9} SOL)`);
    console.log(
      "Please fund the authority account or use 'solana airdrop' for localnet/devnet"
    );
    return;
  }

  console.log(`Authority balance: ${balance / 1e9} SOL`);

  // Prepare fee structure (Anchor converts camelCase to snake_case for Rust)
  const fees = {
    createAgent: new anchor.BN(config.fees.createAgent),
    updateAgentConfig: new anchor.BN(config.fees.updateAgentConfig),
    transferAgent: new anchor.BN(config.fees.transferAgent),
    pauseAgent: new anchor.BN(config.fees.pauseAgent),
    closeAgent: new anchor.BN(config.fees.closeAgent),
    executeAction: new anchor.BN(config.fees.executeAction),
  };

  // Validate treasury distribution
  const totalBps =
    config.treasuryDistribution.protocolBps +
    config.treasuryDistribution.validatorBps +
    config.treasuryDistribution.networkBps;

  if (totalBps !== 10000) {
    throw new Error(
      `Treasury distribution must sum to 10000 bps, got ${totalBps}`
    );
  }

  console.log("\n📦 Initializing protocol configuration...");

  try {
    const treasuryParams = {
      protocolTreasury: treasuries.protocol.publicKey,
      validatorTreasury: treasuries.validator.publicKey,
      networkTreasury: treasuries.network.publicKey,
      protocolTreasuryBps: config.treasuryDistribution.protocolBps,
      validatorTreasuryBps: config.treasuryDistribution.validatorBps,
      networkTreasuryBps: config.treasuryDistribution.networkBps,
    };

    const configParams = {
      genesisCollectionMint: genesisCollection.publicKey,
      maxPartnerCollections: new anchor.BN(config.protocolLimits.maxPartnerCollections),
      maxAffiliateBps: config.protocolLimits.maxAffiliateBps,
      manager: authority.publicKey,
    };

    const tx = await program.methods
      .initializeConfig(
        fees,
        treasuryParams,
        configParams
      )
      .accountsPartial({
        protocolConfig: protocolConfigPDA,
        authority: authority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([authority])
      .rpc();

    console.log(`✅ Protocol initialized successfully!`);
    console.log(`Transaction signature: ${tx}`);

    // Output environment variables
    outputEnvironmentVariables(
      authority,
      genesisCollection.publicKey,
      credentialPDA,
      schemaInfo
    );

    console.log(
      "\n💡 Tip: Keypairs are automatically reused from the keys/ directory."
    );
    console.log("   Use --force-new-keypairs to generate new ones if needed.");
  } catch (error) {
    console.error("❌ Failed to initialize protocol:", error);
    throw error;
  }
}

function backupExistingKeypairs(): void {
  const keysDir = path.join(__dirname, "..", "keys");
  if (!fs.existsSync(keysDir)) {
    return;
  }

  const timestamp = new Date().toISOString().replace(/[:.]/g, "-");
  const backupDir = path.join(keysDir, `backup-${timestamp}`);

  const keyFiles = fs
    .readdirSync(keysDir)
    .filter((file) => file.endsWith(".json") && !file.startsWith("backup-"));

  if (keyFiles.length > 0) {
    fs.mkdirSync(backupDir, { recursive: true });

    keyFiles.forEach((file) => {
      const sourcePath = path.join(keysDir, file);
      const destPath = path.join(backupDir, file);
      fs.copyFileSync(sourcePath, destPath);
      fs.unlinkSync(sourcePath); // Remove original to force regeneration
    });

    console.log(
      `📦 Backed up ${keyFiles.length} existing keypairs to: ${backupDir}`
    );
  }
}

function printUsage(): void {
  console.log(`
Mainframe Configuration Initializer

Usage: npm run init-config [options]

Options:
  --cluster <cluster>                 Target cluster (localnet, devnet, mainnet) [default: localnet]
  --authority-keypair <path>          Path to existing authority keypair
  --protocol-treasury <pubkey>        Public key for protocol treasury
  --validator-treasury <pubkey>       Public key for validator treasury  
  --network-treasury <pubkey>         Public key for network treasury
  --protocol-treasury-keypair <path>  Path to protocol treasury keypair
  --validator-treasury-keypair <path> Path to validator treasury keypair
  --network-treasury-keypair <path>   Path to network treasury keypair
  --generate-new-treasuries          Force generation of new treasury keypairs only
  --force-new-keypairs               Force generation of all new keypairs (backs up existing)
  --help, -h                         Show this help message

Examples:
  npm run init-config                                    # Use/create default keypairs
  npm run init-config --cluster devnet                  # Initialize on devnet
  npm run init-config --force-new-keypairs              # Generate all new keypairs
  npm run init-config --authority-keypair ./my-auth.json # Use specific authority

Note: The script will automatically reuse existing keypairs in the keys/ directory
unless you specify --force-new-keypairs or provide specific keypair paths.
`);
}

function outputEnvironmentVariables(
  authority: Keypair,
  genesisCollectionMint: PublicKey,
  credentialPDA: PublicKey,
  schemaInfo: { schemaPDA: PublicKey; schemaMint: PublicKey }
): void {
  console.log("\n=== Environment Variables ===");
  console.log("Add these to your .env file:");
  console.log("");

  const authorityPrivateKey = bs58.encode(authority.secretKey);

  console.log(`MAIKERS_AUTHORITY_PRIVATE_KEY=${authorityPrivateKey}`);
  console.log(`MAIKERS_GENESIS_COLLECTION=${genesisCollectionMint.toBase58()}`);
  console.log(`MAIKERS_CREDENTIAL_PDA=${credentialPDA.toBase58()}`);
  console.log(`MAIKERS_SCHEMA_PDA=${schemaInfo.schemaPDA.toBase58()}`);
  console.log(`MAIKERS_SCHEMA_MINT=${schemaInfo.schemaMint.toBase58()}`);
  console.log(`MAIKERS_PROTOCOL_TREASURY=${authority.publicKey.toBase58()}`);
  console.log("");

  // Also save to .env file
  const envContent = `# Maikers Mainframe Configuration
MAIKERS_AUTHORITY_PRIVATE_KEY=${authorityPrivateKey}
MAIKERS_GENESIS_COLLECTION=${genesisCollectionMint.toBase58()}
MAIKERS_CREDENTIAL_PDA=${credentialPDA.toBase58()}
MAIKERS_SCHEMA_PDA=${schemaInfo.schemaPDA.toBase58()}
MAIKERS_SCHEMA_MINT=${schemaInfo.schemaMint.toBase58()}
MAIKERS_PROTOCOL_TREASURY=${authority.publicKey.toBase58()}
`;

  const envPath = path.join(__dirname, "..", ".env");
  fs.writeFileSync(envPath, envContent);
  console.log(`📄 Environment variables saved to: ${envPath}`);
}

// CLI handling
async function main() {
  const args = process.argv.slice(2);
  const config = { ...defaultConfig };

  // Parse command line arguments
  for (let i = 0; i < args.length; i += 2) {
    const arg = args[i];
    const value = args[i + 1];

    switch (arg) {
      case "--cluster":
        config.cluster = value;
        break;
      case "--authority-keypair":
        config.authorityKeypairPath = value;
        break;
      case "--protocol-treasury":
        if (!config.treasuries) config.treasuries = {};
        config.treasuries.protocol = value;
        break;
      case "--validator-treasury":
        if (!config.treasuries) config.treasuries = {};
        config.treasuries.validator = value;
        break;
      case "--network-treasury":
        if (!config.treasuries) config.treasuries = {};
        config.treasuries.network = value;
        break;
      case "--protocol-treasury-keypair":
        if (!config.treasuryKeypairPaths) config.treasuryKeypairPaths = {};
        config.treasuryKeypairPaths.protocol = value;
        break;
      case "--validator-treasury-keypair":
        if (!config.treasuryKeypairPaths) config.treasuryKeypairPaths = {};
        config.treasuryKeypairPaths.validator = value;
        break;
      case "--network-treasury-keypair":
        if (!config.treasuryKeypairPaths) config.treasuryKeypairPaths = {};
        config.treasuryKeypairPaths.network = value;
        break;
      case "--generate-new-treasuries":
        // Force generation of new treasury keypairs
        config.treasuryKeypairPaths = {};
        config.treasuries = {};
        break;
      case "--force-new-keypairs":
        // Force generation of all new keypairs by backing up existing ones
        backupExistingKeypairs();
        config.treasuryKeypairPaths = {};
        config.treasuries = {};
        break;
      case "--help":
      case "-h":
        printUsage();
        process.exit(0);
        break;
      default:
        if (arg.startsWith("--")) {
          console.log(`Unknown argument: ${arg}`);
          console.log("Use --help to see available options.");
        }
        break;
    }
  }

  try {
    await initializeConfig(config);
  } catch (error) {
    console.error("Initialization failed:", error);
    process.exit(1);
  }
}

if (require.main === module) {
  main().catch(console.error);
}

export { initializeConfig, defaultConfig };
