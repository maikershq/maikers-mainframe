import * as anchor from "@coral-xyz/anchor";
import { address, Address } from "@solana/addresses";
import {
  PublicKey,
  Keypair,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import {
  createMint,
  createAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { TOKEN_PROGRAM_ADDRESS } from "@solana-program/token";
import { publicKeyToAddress, addressToPublicKey } from "./solana-kit-helpers";

const MPL_TOKEN_METADATA_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

export interface CreateNFTParams {
  provider: anchor.AnchorProvider;
  payer: Keypair;
  owner: PublicKey;
  name: string;
  symbol: string;
  uri: string;
  collectionMint?: PublicKey;
  collectionMetadata?: PublicKey;
  collectionMasterEdition?: PublicKey;
  collectionAuthority?: Keypair;
}

export interface NFTData {
  mint: PublicKey;
  tokenAccount: PublicKey;
  metadata: PublicKey;
  masterEdition: PublicKey;
}

export function getMetadataPDA(mint: PublicKey): PublicKey {
  const [metadata] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mint.toBuffer(),
    ],
    MPL_TOKEN_METADATA_PROGRAM_ID
  );
  return metadata;
}

export function getMasterEditionPDA(mint: PublicKey): PublicKey {
  const [masterEdition] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mint.toBuffer(),
      Buffer.from("edition"),
    ],
    MPL_TOKEN_METADATA_PROGRAM_ID
  );
  return masterEdition;
}

export async function createNFTWithMetadata(
  params: CreateNFTParams
): Promise<NFTData> {
  const { provider, payer, owner, name, symbol, uri } = params;

  // Wait for payer to have funds
  let balance = await provider.connection.getBalance(payer.publicKey);
  let retries = 0;
  while (balance === 0 && retries < 10) {
    await new Promise((resolve) => setTimeout(resolve, 1000));
    balance = await provider.connection.getBalance(payer.publicKey);
    retries++;
  }

  // Create mint
  const mint = Keypair.generate();
  const mintPubkey = await createMint(
    provider.connection,
    payer,
    payer.publicKey,
    payer.publicKey,
    0,
    mint
  );

  // Create token account
  const tokenAccount = await createAccount(
    provider.connection,
    payer,
    mintPubkey,
    owner
  );

  // Mint one token
  await mintTo(provider.connection, payer, mintPubkey, tokenAccount, payer, 1);

  // Get metadata PDA
  const metadata = getMetadataPDA(mintPubkey);
  const masterEdition = getMasterEditionPDA(mintPubkey);

  // Create minimal metadata account using web3.js
  // Build metadata instruction data manually
  const metadataData = Buffer.alloc(679);
  let offset = 0;

  // Discriminator for CreateMetadataAccountV3 = 33
  metadataData.writeUInt8(33, offset);
  offset += 1;

  // DataV2 struct
  // name (String)
  const nameBytes = Buffer.from(name);
  metadataData.writeUInt32LE(nameBytes.length, offset);
  offset += 4;
  nameBytes.copy(metadataData, offset);
  offset += nameBytes.length;

  // symbol (String)
  const symbolBytes = Buffer.from(symbol);
  metadataData.writeUInt32LE(symbolBytes.length, offset);
  offset += 4;
  symbolBytes.copy(metadataData, offset);
  offset += symbolBytes.length;

  // uri (String)
  const uriBytes = Buffer.from(uri);
  metadataData.writeUInt32LE(uriBytes.length, offset);
  offset += 4;
  uriBytes.copy(metadataData, offset);
  offset += uriBytes.length;

  // sellerFeeBasisPoints (u16)
  metadataData.writeUInt16LE(0, offset);
  offset += 2;

  // creators (Option<Vec<Creator>>)
  metadataData.writeUInt8(0, offset); // None
  offset += 1;

  // collection (Option<Collection>)
  if (params.collectionMint) {
    metadataData.writeUInt8(1, offset); // Some
    offset += 1;
    metadataData.writeUInt8(0, offset); // verified = false (must verify separately)
    offset += 1;
    params.collectionMint.toBuffer().copy(metadataData, offset);
    offset += 32;
  } else {
    metadataData.writeUInt8(0, offset); // None
    offset += 1;
  }

  // uses (Option<Uses>)
  metadataData.writeUInt8(0, offset); // None
  offset += 1;

  // isMutable (bool)
  metadataData.writeUInt8(1, offset);
  offset += 1;

  // collectionDetails (Option<CollectionDetails>)
  metadataData.writeUInt8(0, offset); // None
  offset += 1;

  const createMetadataIx = new TransactionInstruction({
    keys: [
      { pubkey: metadata, isSigner: false, isWritable: true },
      { pubkey: mintPubkey, isSigner: false, isWritable: false },
      { pubkey: payer.publicKey, isSigner: true, isWritable: false },
      { pubkey: payer.publicKey, isSigner: true, isWritable: true },
      { pubkey: payer.publicKey, isSigner: false, isWritable: false },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
    ],
    programId: MPL_TOKEN_METADATA_PROGRAM_ID,
    data: metadataData.slice(0, offset),
  });

  const createMetadataTx = new Transaction().add(createMetadataIx);
  await provider.sendAndConfirm(createMetadataTx, [payer]);

  // If collection info is provided, verify the collection
  if (
    params.collectionMint &&
    params.collectionMetadata &&
    params.collectionMasterEdition &&
    params.collectionAuthority
  ) {
    const verifyIx = createVerifyCollectionInstruction({
      metadata,
      collectionAuthority: params.collectionAuthority.publicKey,
      collectionMint: params.collectionMint,
      collection: params.collectionMetadata,
      collectionMasterEditionAccount: params.collectionMasterEdition,
    });

    const verifyTx = new Transaction().add(verifyIx);
    await provider.sendAndConfirm(verifyTx, [params.collectionAuthority]);
  }

  return {
    mint: mintPubkey,
    tokenAccount,
    metadata,
    masterEdition,
  };
}

function createMetadataAccountV3Instruction(params: {
  metadata: PublicKey;
  mint: PublicKey;
  mintAuthority: PublicKey;
  payer: PublicKey;
  updateAuthority: PublicKey;
  data: {
    name: string;
    symbol: string;
    uri: string;
    sellerFeeBasisPoints: number;
    creators: any;
    collection: { verified: boolean; key: PublicKey } | null;
    uses: any;
  };
  isMutable: boolean;
  collectionDetails: any;
}): anchor.web3.TransactionInstruction {
  const keys = [
    { pubkey: params.metadata, isSigner: false, isWritable: true },
    { pubkey: params.mint, isSigner: false, isWritable: false },
    { pubkey: params.mintAuthority, isSigner: true, isWritable: false },
    { pubkey: params.payer, isSigner: true, isWritable: true },
    { pubkey: params.updateAuthority, isSigner: false, isWritable: false },
    { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
  ];

  const dataLayout = Buffer.alloc(500);
  let offset = 0;

  dataLayout.writeUInt8(33, offset);
  offset += 1;

  const nameBuffer = Buffer.from(params.data.name);
  dataLayout.writeUInt32LE(nameBuffer.length, offset);
  offset += 4;
  nameBuffer.copy(dataLayout, offset);
  offset += nameBuffer.length;

  const symbolBuffer = Buffer.from(params.data.symbol);
  dataLayout.writeUInt32LE(symbolBuffer.length, offset);
  offset += 4;
  symbolBuffer.copy(dataLayout, offset);
  offset += symbolBuffer.length;

  const uriBuffer = Buffer.from(params.data.uri);
  dataLayout.writeUInt32LE(uriBuffer.length, offset);
  offset += 4;
  uriBuffer.copy(dataLayout, offset);
  offset += uriBuffer.length;

  dataLayout.writeUInt16LE(params.data.sellerFeeBasisPoints, offset);
  offset += 2;

  dataLayout.writeUInt8(params.data.creators ? 1 : 0, offset);
  offset += 1;

  dataLayout.writeUInt8(params.data.collection ? 1 : 0, offset);
  offset += 1;
  if (params.data.collection) {
    dataLayout.writeUInt8(params.data.collection.verified ? 1 : 0, offset);
    offset += 1;
    params.data.collection.key.toBuffer().copy(dataLayout, offset);
    offset += 32;
  }

  dataLayout.writeUInt8(params.data.uses ? 1 : 0, offset);
  offset += 1;

  dataLayout.writeUInt8(params.isMutable ? 1 : 0, offset);
  offset += 1;

  dataLayout.writeUInt8(params.collectionDetails ? 1 : 0, offset);
  offset += 1;

  return new anchor.web3.TransactionInstruction({
    keys,
    programId: MPL_TOKEN_METADATA_PROGRAM_ID,
    data: dataLayout.slice(0, offset),
  });
}

function createMasterEditionV3Instruction(params: {
  edition: PublicKey;
  metadata: PublicKey;
  mint: PublicKey;
  mintAuthority: PublicKey;
  updateAuthority: PublicKey;
  payer: PublicKey;
  maxSupply: number;
}): anchor.web3.TransactionInstruction {
  const TOKEN_PROGRAM_ID = new PublicKey(TOKEN_PROGRAM_ADDRESS);

  const keys = [
    { pubkey: params.edition, isSigner: false, isWritable: true },
    { pubkey: params.mint, isSigner: false, isWritable: true },
    { pubkey: params.updateAuthority, isSigner: true, isWritable: false },
    { pubkey: params.mintAuthority, isSigner: true, isWritable: false },
    { pubkey: params.payer, isSigner: true, isWritable: true },
    { pubkey: params.metadata, isSigner: false, isWritable: true },
    { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
  ];

  const data = Buffer.alloc(10);
  data.writeUInt8(17, 0);
  if (params.maxSupply === 0) {
    data.writeUInt8(0, 1);
  } else {
    data.writeUInt8(1, 1);
    data.writeBigUInt64LE(BigInt(params.maxSupply), 2);
  }

  return new anchor.web3.TransactionInstruction({
    keys,
    programId: MPL_TOKEN_METADATA_PROGRAM_ID,
    data,
  });
}

function createVerifyCollectionInstruction(params: {
  metadata: PublicKey;
  collectionAuthority: PublicKey;
  collectionMint: PublicKey;
  collection: PublicKey;
  collectionMasterEditionAccount: PublicKey;
}): anchor.web3.TransactionInstruction {
  const keys = [
    { pubkey: params.metadata, isSigner: false, isWritable: true },
    { pubkey: params.collectionAuthority, isSigner: true, isWritable: true },
    { pubkey: params.collectionAuthority, isSigner: true, isWritable: false },
    { pubkey: params.collectionMint, isSigner: false, isWritable: false },
    { pubkey: params.collection, isSigner: false, isWritable: false },
    {
      pubkey: params.collectionMasterEditionAccount,
      isSigner: false,
      isWritable: false,
    },
  ];

  const data = Buffer.from([18]);

  return new anchor.web3.TransactionInstruction({
    keys,
    programId: MPL_TOKEN_METADATA_PROGRAM_ID,
    data,
  });
}

export async function createCollectionNFT(
  params: CreateNFTParams
): Promise<NFTData> {
  const { provider, payer, owner, name, symbol, uri } = params;

  let balance = await provider.connection.getBalance(payer.publicKey);
  let retries = 0;
  while (balance === 0 && retries < 10) {
    await new Promise((resolve) => setTimeout(resolve, 1000));
    balance = await provider.connection.getBalance(payer.publicKey);
    retries++;
  }

  const mint = Keypair.generate();
  const mintPubkey = await createMint(
    provider.connection,
    payer,
    payer.publicKey,
    payer.publicKey,
    0,
    mint
  );

  const tokenAccount = await createAccount(
    provider.connection,
    payer,
    mintPubkey,
    owner
  );

  await mintTo(provider.connection, payer, mintPubkey, tokenAccount, payer, 1);

  const metadata = getMetadataPDA(mintPubkey);
  const masterEdition = getMasterEditionPDA(mintPubkey);

  const metadataData = Buffer.alloc(679);
  let offset = 0;

  metadataData.writeUInt8(33, offset);
  offset += 1;

  const nameBytes = Buffer.from(name);
  metadataData.writeUInt32LE(nameBytes.length, offset);
  offset += 4;
  nameBytes.copy(metadataData, offset);
  offset += nameBytes.length;

  const symbolBytes = Buffer.from(symbol);
  metadataData.writeUInt32LE(symbolBytes.length, offset);
  offset += 4;
  symbolBytes.copy(metadataData, offset);
  offset += symbolBytes.length;

  const uriBytes = Buffer.from(uri);
  metadataData.writeUInt32LE(uriBytes.length, offset);
  offset += 4;
  uriBytes.copy(metadataData, offset);
  offset += uriBytes.length;

  metadataData.writeUInt16LE(0, offset);
  offset += 2;

  metadataData.writeUInt8(0, offset);
  offset += 1;

  metadataData.writeUInt8(0, offset);
  offset += 1;

  metadataData.writeUInt8(0, offset);
  offset += 1;

  metadataData.writeUInt8(1, offset);
  offset += 1;

  metadataData.writeUInt8(0, offset);
  offset += 1;

  const createMetadataIx = new TransactionInstruction({
    keys: [
      { pubkey: metadata, isSigner: false, isWritable: true },
      { pubkey: mintPubkey, isSigner: false, isWritable: false },
      { pubkey: payer.publicKey, isSigner: true, isWritable: false },
      { pubkey: payer.publicKey, isSigner: true, isWritable: true },
      { pubkey: payer.publicKey, isSigner: false, isWritable: false },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
    ],
    programId: MPL_TOKEN_METADATA_PROGRAM_ID,
    data: metadataData.slice(0, offset),
  });

  const createMetadataTx = new Transaction().add(createMetadataIx);
  await provider.sendAndConfirm(createMetadataTx, [payer]);

  const createMasterEditionIx = createMasterEditionV3Instruction({
    edition: masterEdition,
    metadata,
    mint: mintPubkey,
    mintAuthority: payer.publicKey,
    updateAuthority: payer.publicKey,
    payer: payer.publicKey,
    maxSupply: 0,
  });

  const createMasterEditionTx = new Transaction().add(createMasterEditionIx);
  await provider.sendAndConfirm(createMasterEditionTx, [payer]);

  return {
    mint: mintPubkey,
    tokenAccount,
    metadata,
    masterEdition,
  };
}
