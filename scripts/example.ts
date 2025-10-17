import { PublicKey } from "@solana/web3.js";
import { initializeConfig } from "./initialize_config";

// Example configuration for different environments

async function initializeDevnet() {
  console.log("🚀 Initializing Mainframe on Devnet...");
  
  await initializeConfig({
    cluster: "devnet",
    fees: {
      createAgent: 50000000,  // 0.05 SOL - Agent activation
      updateConfig: 5000000,  // 0.005 SOL - Configuration updates
      transferAgent: 10000000, // 0.01 SOL - Ownership transfers
      pauseAgent: 0,          // 0 SOL - Always free
      closeAgent: 0,          // 0 SOL - Always free
      executeAction: 0,       // 0 SOL - Always free
    },
    treasuries: {
      // These are ignored - treasuries are auto-generated in keys folder
      protocol: new PublicKey("11111111111111111111111111111112"),
      validator: new PublicKey("11111111111111111111111111111112"), 
      network: new PublicKey("11111111111111111111111111111112"),
    },
    treasuryDistribution: {
      protocolBps: 5000,    // 50%
      validatorBps: 3000,   // 30%
      networkBps: 2000,     // 20%
    }
  });
}

async function initializeMainnet() {
  console.log("🚀 Initializing Mainframe on Mainnet...");
  
  await initializeConfig({
    cluster: "mainnet",
    fees: {
      createAgent: 50000000,  // 0.05 SOL - Standard mainnet fee
      updateConfig: 5000000,  // 0.005 SOL - Standard mainnet fee
      transferAgent: 10000000, // 0.01 SOL - Standard mainnet fee
      pauseAgent: 0,          // 0 SOL - Always free
      closeAgent: 0,          // 0 SOL - Always free
      executeAction: 0,       // 0 SOL - Always free
    },
    treasuries: {
      // These are ignored - treasuries are auto-generated in keys folder  
      protocol: new PublicKey("11111111111111111111111111111112"),
      validator: new PublicKey("11111111111111111111111111111112"),
      network: new PublicKey("11111111111111111111111111111112"),
    },
    treasuryDistribution: {
      protocolBps: 4000,    // 40%
      validatorBps: 3500,   // 35%
      networkBps: 2500,     // 25%
    }
  });
}

async function initializeWithCustomAuthority() {
  console.log("🚀 Initializing with custom authority keypair...");
  
  await initializeConfig({
    cluster: "devnet",
    authorityKeypairPath: "./keys/custom-authority.json", // Custom authority
    fees: {
      createAgent: 15000000,  // 0.015 SOL - Custom activation fee
      updateConfig: 3750000,  // 0.00375 SOL - Custom configuration updates
      transferAgent: 1500000, // 0.0015 SOL - Custom ownership transfers
      pauseAgent: 0,          // 0 SOL - Always free
      closeAgent: 0,          // 0 SOL - Always free
      executeAction: 0,       // 0 SOL - Always free
    },
    treasuries: {
      // These are ignored - treasuries are auto-generated in keys folder
      protocol: new PublicKey("11111111111111111111111111111112"),
      validator: new PublicKey("11111111111111111111111111111112"),
      network: new PublicKey("11111111111111111111111111111112"),
    },
    treasuryDistribution: {
      protocolBps: 6000,    // 60%
      validatorBps: 2500,   // 25%
      networkBps: 1500,     // 15%
    }
  });
}

// Run examples
async function main() {
  const example = process.env.EXAMPLE || "devnet";
  
  switch (example) {
    case "devnet":
      await initializeDevnet();
      break;
    case "mainnet":
      await initializeMainnet();
      break;
    case "custom":
      await initializeWithCustomAuthority();
      break;
    default:
      console.log("Usage: EXAMPLE=devnet|mainnet|custom ts-node scripts/example.ts");
      break;
  }
}

if (require.main === module) {
  main().catch(console.error);
}

export { initializeDevnet, initializeMainnet, initializeWithCustomAuthority };
