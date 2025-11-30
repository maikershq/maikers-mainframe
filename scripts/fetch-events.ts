/**
 * Fetch and decode Mainframe program events
 *
 * Usage:
 *   ts-node scripts/fetch-events.ts [--network mainnet|devnet] [--limit 100] [--event AgentCreated]
 */

import { Connection, PublicKey } from "@solana/web3.js";
import { AnchorProvider, Program, BN } from "@coral-xyz/anchor";
import { Mainframe } from "../target/types/mainframe";
import * as fs from "fs";

function convertBNFields(obj: any): any {
  if (obj instanceof PublicKey) {
    return obj.toString();
  }
  if (obj instanceof BN) {
    return obj.toString();
  }
  if (Array.isArray(obj)) {
    return obj.map(convertBNFields);
  }
  if (obj && typeof obj === "object") {
    const converted: any = {};
    for (const [key, value] of Object.entries(obj)) {
      converted[key] = convertBNFields(value);
    }
    return converted;
  }
  return obj;
}

const MAINFRAME_PROGRAM_ID = new PublicKey(
  "mnfm211AwTDA8fGvPezYs3jjxAXgoucHGuTMUbjFssE"
);

interface FetchOptions {
  network: "mainnet" | "devnet";
  limit: number;
  eventType?: string;
  beforeSignature?: string;
}

async function parseArgs(): Promise<FetchOptions> {
  const args = process.argv.slice(2);
  const options: FetchOptions = {
    network: "mainnet",
    limit: 100,
  };

  for (let i = 0; i < args.length; i++) {
    if (args[i] === "--network" && args[i + 1]) {
      options.network = args[i + 1] as "mainnet" | "devnet";
      i++;
    } else if (args[i] === "--limit" && args[i + 1]) {
      options.limit = parseInt(args[i + 1]);
      i++;
    } else if (args[i] === "--event" && args[i + 1]) {
      options.eventType = args[i + 1];
      i++;
    } else if (args[i] === "--before" && args[i + 1]) {
      options.beforeSignature = args[i + 1];
      i++;
    }
  }

  return options;
}

async function fetchProgramEvents(options: FetchOptions) {
  const rpcUrl =
    options.network === "mainnet"
      ? process.env.SOLANA_RPC_URL || "https://api.mainnet-beta.solana.com"
      : "https://api.devnet.solana.com";

  console.log(`🔍 Fetching events from ${options.network}...`);
  console.log(`📡 RPC: ${rpcUrl}`);
  console.log(`📊 Limit: ${options.limit}`);
  if (options.eventType) console.log(`🎯 Event filter: ${options.eventType}`);
  console.log("");

  const connection = new Connection(rpcUrl, "confirmed");

  // Load IDL from file
  const idlPath = __dirname + "/../target/idl/mainframe.json";
  const idl = JSON.parse(fs.readFileSync(idlPath, "utf8"));

  // Create a dummy wallet for the provider
  const dummyWallet = {
    publicKey: PublicKey.default,
    signTransaction: async () => {
      throw new Error("Not needed");
    },
    signAllTransactions: async () => {
      throw new Error("Not needed");
    },
  };

  // @ts-ignore
  const provider = new AnchorProvider(connection, dummyWallet, {
    commitment: "confirmed",
  });

  // @ts-ignore
  const program = new Program(idl, provider);

  // Fetch transaction signatures
  console.log("📜 Fetching transaction signatures...");
  const signatures = await connection.getSignaturesForAddress(
    MAINFRAME_PROGRAM_ID,
    {
      limit: options.limit,
      before: options.beforeSignature,
    }
  );

  console.log(`✅ Found ${signatures.length} transactions\n`);

  const events: Array<{
    signature: string;
    slot: number;
    blockTime: number | null;
    eventName: string;
    data: any;
  }> = [];

  // Process each transaction
  let processedCount = 0;
  for (const sig of signatures) {
    try {
      const tx = await connection.getTransaction(sig.signature, {
        commitment: "confirmed",
        maxSupportedTransactionVersion: 0,
      });

      if (!tx || !tx.meta?.logMessages) {
        processedCount++;
        continue;
      }

      // Look for events in logs
      const logs = tx.meta.logMessages;
      const hasEvent = logs.some((log) => log.includes("Program data:"));

      if (hasEvent) {
        console.log(
          `\n🔍 TX ${processedCount + 1}/${
            signatures.length
          }: ${sig.signature.slice(0, 8)}...`
        );
        console.log(`   Logs with events:`);

        for (const log of logs) {
          if (log.includes("Program data:")) {
            console.log(`   ${log}`);

            // Extract and decode event
            const dataMatch = log.match(/Program data: ([A-Za-z0-9+/=]+)/);
            if (dataMatch && dataMatch[1]) {
              const eventData = Buffer.from(dataMatch[1], "base64");
              const discriminator = eventData.slice(0, 8);

              console.log(
                `   Discriminator: [${Array.from(discriminator).join(", ")}]`
              );

              // Try to decode
              try {
                // @ts-ignore
                const decoded = program.coder.events.decode(
                  eventData.toString("base64")
                );
                if (decoded) {
                  console.log(`   ✅ Decoded as: ${decoded.name}`);

                  // Case-insensitive event filter
                  const matchesFilter =
                    !options.eventType ||
                    decoded.name.toLowerCase() ===
                      options.eventType.toLowerCase();

                  if (matchesFilter) {
                    events.push({
                      signature: sig.signature,
                      slot: sig.slot,
                      blockTime: sig.blockTime,
                      eventName: decoded.name,
                      data: convertBNFields(decoded.data),
                    });
                    console.log(`   ✅ Added to results`);
                  } else {
                    console.log(
                      `   ⏭️  Skipped (filter: ${options.eventType})`
                    );
                  }
                }
              } catch (e) {
                console.log(`   ⚠️  Decode failed: ${e.message || e}`);
              }
            }
          }
        }
      }

      processedCount++;
    } catch (error) {
      processedCount++;
      continue;
    }
  }

  return events;
}

// Simple event parser (Anchor's EventParser equivalent)
class EventParser {
  constructor(private programId: PublicKey, private coder: any) {}

  parseLogs(logs: string[]): Array<{ name: string; data: any }> {
    const events: Array<{ name: string; data: any }> = [];

    for (const log of logs) {
      // Anchor events are logged as "Program data: <base64>"
      if (log.includes("Program data:")) {
        try {
          const dataMatch = log.match(/Program data: ([A-Za-z0-9+/=]+)/);
          if (dataMatch && dataMatch[1]) {
            const eventData = Buffer.from(dataMatch[1], "base64");

            // Anchor event format: 8-byte discriminator + event data
            // Try to decode using the coder
            try {
              const decoded = this.coder.events.decode(
                eventData.toString("base64")
              );
              if (decoded) {
                events.push({
                  name: decoded.name,
                  data: decoded.data,
                });
              }
            } catch (e) {
              // If coder fails, try manual decoding
              // Event discriminator is first 8 bytes
              const discriminator = eventData.slice(0, 8);

              // Match against known event discriminators from IDL
              const eventName = this.matchDiscriminator(discriminator);
              if (eventName) {
                events.push({
                  name: eventName,
                  data: { raw: eventData.toString("base64") },
                });
              }
            }
          }
        } catch (error) {
          // Skip unparseable logs
        }
      }
    }

    return events;
  }

  private matchDiscriminator(discriminator: Buffer): string | null {
    // AgentCreated discriminator from IDL
    const knownEvents: Record<string, number[]> = {
      AgentCreated: [237, 44, 61, 111, 90, 251, 241, 34],
      AgentUpdated: [210, 179, 162, 250, 123, 250, 210, 166],
      AgentTransferred: [5, 213, 4, 28, 147, 238, 64, 57],
      AgentPaused: [228, 35, 167, 28, 96, 5, 210, 82],
      AgentResumed: [138, 191, 50, 131, 92, 153, 211, 143],
      AgentClosed: [72, 133, 207, 59, 220, 188, 201, 165],
      AgentAccountClosed: [16, 48, 148, 81, 34, 222, 87, 149],
      AffiliateRegistered: [105, 5, 31, 167, 189, 121, 113, 42],
      AffiliatePaid: [23, 62, 238, 53, 254, 32, 36, 32],
      AffiliateBonusSet: [23, 255, 195, 110, 108, 101, 24, 233],
      TierUpgraded: [141, 12, 195, 74, 212, 102, 162, 123],
      TreasuryAddressesUpdated: [73, 89, 76, 157, 121, 252, 43, 169],
    };

    for (const [name, disc] of Object.entries(knownEvents)) {
      if (discriminator.length >= 8) {
        let match = true;
        for (let i = 0; i < 8; i++) {
          if (discriminator[i] !== disc[i]) {
            match = false;
            break;
          }
        }
        if (match) return name;
      }
    }

    return null;
  }
}

function formatEvent(event: any, index: number) {
  console.log(`\n${index + 1}. ${event.eventName}`);
  console.log(`   Signature: ${event.signature}`);
  console.log(`   Slot: ${event.slot}`);
  console.log(
    `   Time: ${
      event.blockTime ? new Date(event.blockTime * 1000).toISOString() : "N/A"
    }`
  );
  console.log(`   Data:`);

  switch (event.eventName) {
    case "agentCreated":
      console.log(`     Agent: ${event.data.agentAccount}`);
      console.log(`     NFT Mint: ${event.data.nftMint}`);
      console.log(`     Owner: ${event.data.owner}`);
      console.log(`     Collection: ${event.data.collectionMint || "None"}`);
      console.log(`     Metadata URI: ${event.data.metadataUri}`);
      console.log(`     Seller: ${event.data.seller || "None"}`);
      console.log(
        `     Timestamp: ${event.data.timestamp} (${new Date(
          parseInt(event.data.timestamp) * 1000
        ).toISOString()})`
      );
      console.log(`     Version: ${event.data.version}`);
      break;

    case "agentUpdated":
      console.log(`     Agent: ${event.data.agentAccount}`);
      console.log(`     Owner: ${event.data.owner}`);
      console.log(`     Old Version: ${event.data.oldVersion}`);
      console.log(`     New Version: ${event.data.newVersion}`);
      console.log(
        `     Timestamp: ${event.data.timestamp} (${new Date(
          parseInt(event.data.timestamp) * 1000
        ).toISOString()})`
      );
      break;

    case "affiliatePaid":
      console.log(`     Agent: ${event.data.agentAccount}`);
      console.log(`     Seller: ${event.data.seller}`);
      console.log(
        `     Amount: ${parseInt(event.data.affiliateAmount) / 1e9} SOL`
      );
      console.log(`     Rate: ${event.data.affiliateBps} bps`);
      console.log(
        `     Timestamp: ${event.data.timestamp} (${new Date(
          parseInt(event.data.timestamp) * 1000
        ).toISOString()})`
      );
      break;

    case "affiliateRegistered":
      console.log(`     Affiliate: ${event.data.affiliate}`);
      console.log(`     Referrer: ${event.data.referrer || "None"}`);
      console.log(
        `     Timestamp: ${event.data.timestamp} (${new Date(
          parseInt(event.data.timestamp) * 1000
        ).toISOString()})`
      );
      break;

    case "tierUpgraded":
      console.log(`     Affiliate: ${event.data.affiliate}`);
      console.log(`     Old Tier: ${event.data.oldTier}`);
      console.log(`     New Tier: ${event.data.newTier}`);
      console.log(`     Total Sales: ${event.data.totalSales}`);
      console.log(
        `     Timestamp: ${event.data.timestamp} (${new Date(
          parseInt(event.data.timestamp) * 1000
        ).toISOString()})`
      );
      break;

    default:
      console.log(`     ${JSON.stringify(event.data, null, 6)}`);
  }
}

async function main() {
  try {
    const options = await parseArgs();
    const events = await fetchProgramEvents(options);

    console.log(`\n📊 Found ${events.length} events\n`);
    console.log("═".repeat(60));

    events.forEach((event, index) => formatEvent(event, index));

    console.log("\n" + "═".repeat(60));
    console.log(`\n✅ Total: ${events.length} events`);

    // Summary by event type
    const summary = events.reduce((acc, event) => {
      acc[event.eventName] = (acc[event.eventName] || 0) + 1;
      return acc;
    }, {} as Record<string, number>);

    console.log("\n📈 Summary:");
    Object.entries(summary).forEach(([name, count]) => {
      console.log(`   ${name}: ${count}`);
    });
  } catch (error) {
    console.error("❌ Error:", error);
    process.exit(1);
  }
}

main();
