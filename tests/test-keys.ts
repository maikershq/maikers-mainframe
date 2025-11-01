import { Keypair } from "@solana/web3.js";
import * as fs from "fs";
import * as path from "path";

const KEYS_DIR = path.join(__dirname, "..", "keys", "test");

export function ensureKeysDir() {
  if (!fs.existsSync(KEYS_DIR)) {
    fs.mkdirSync(KEYS_DIR, { recursive: true });
  }
}

export function loadOrCreateKeypair(name: string): Keypair {
  ensureKeysDir();
  const keyPath = path.join(KEYS_DIR, `${name}.json`);
  
  if (fs.existsSync(keyPath)) {
    const secretKey = JSON.parse(fs.readFileSync(keyPath, "utf-8"));
    return Keypair.fromSecretKey(Uint8Array.from(secretKey));
  }
  
  const keypair = Keypair.generate();
  fs.writeFileSync(keyPath, JSON.stringify(Array.from(keypair.secretKey)));
  console.log(`Created new keypair: ${name} (${keypair.publicKey.toString()})`);
  return keypair;
}

export function loadOrCreateCollectionKeypair(name: string): Keypair {
  return loadOrCreateKeypair(`${name}-collection`);
}

export function cleanupTestKeys() {
  if (fs.existsSync(KEYS_DIR)) {
    fs.rmSync(KEYS_DIR, { recursive: true });
  }
}

