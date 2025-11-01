/**
 * Solana Kit (v2) helpers for test utilities
 *
 * These helpers bridge between Anchor (which uses web3.js v1) and
 * modern Solana Kit v2 APIs for better type safety and performance.
 */

import { address, Address } from "@solana/addresses";
import { generateKeyPair, createKeyPairFromBytes } from "@solana/keys";
import { PublicKey, Keypair } from "@solana/web3.js";

/**
 * Convert web3.js v1 PublicKey to v2 Address
 */
export function publicKeyToAddress(pubkey: PublicKey): Address {
  return address(pubkey.toBase58());
}

/**
 * Convert v2 Address to web3.js v1 PublicKey
 */
export function addressToPublicKey(addr: Address): PublicKey {
  return new PublicKey(addr);
}

/**
 * Convert web3.js v1 Keypair to v2 CryptoKeyPair
 */
export async function keypairToV2(keypair: Keypair) {
  return await createKeyPairFromBytes(keypair.secretKey);
}

/**
 * Generate a new keypair using v2 API and return v1 compatible
 */
export async function generateKeypairV1Compat(): Promise<Keypair> {
  const v2Keypair = await generateKeyPair();
  // Note: This is a simplified conversion - full implementation would extract private key
  // For now, use v1 directly since Anchor requires it
  return Keypair.generate();
}

/**
 * Batch convert PublicKeys to Addresses
 */
export function publicKeysToAddresses(pubkeys: PublicKey[]): Address[] {
  return pubkeys.map(publicKeyToAddress);
}

/**
 * Batch convert Addresses to PublicKeys
 */
export function addressesToPublicKeys(addresses: Address[]): PublicKey[] {
  return addresses.map(addressToPublicKey);
}

/**
 * Type guard to check if a value is an Address
 */
export function isAddress(value: unknown): value is Address {
  return typeof value === "string" && value.length >= 32 && value.length <= 44;
}

/**
 * Type guard to check if a value is a PublicKey
 */
export function isPublicKey(value: unknown): value is PublicKey {
  return value instanceof PublicKey;
}

/**
 * Common program addresses as v2 Address types
 */
export const PROGRAM_ADDRESSES = {
  system: address("11111111111111111111111111111111"),
  token: address("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
  associatedToken: address("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
  metaplex: address("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
} as const;

/**
 * Helper to create Address from string with validation
 */
export function createAddressSafe(addressStr: string): Address {
  try {
    return address(addressStr);
  } catch (error) {
    throw new Error(`Invalid address: ${addressStr}`);
  }
}

/**
 * Helper to create multiple addresses safely
 */
export function createAddressesSafe(addressStrs: string[]): Address[] {
  return addressStrs.map(createAddressSafe);
}
