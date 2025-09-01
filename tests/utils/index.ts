import { AnchorProvider } from "@coral-xyz/anchor";
import {  TOKEN_PROGRAM_ID, createInitializeMintInstruction,  MINT_SIZE, createAssociatedTokenAccountInstruction, createMintToInstruction, getAssociatedTokenAddressSync, createMint } from "@solana/spl-token";
import { PublicKey, Keypair, SystemProgram, Transaction, sendAndConfirmTransaction } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * as mpl from "@metaplex-foundation/mpl-token-metadata";
import { randomBytes } from "crypto";
import bs58 from "bs58";
function generateSolanaAddressWithLength(targetLength: number): [PublicKey, Keypair] {
  let tokenMint: PublicKey;
  let tokenKeypair: Keypair;
  while (true) {
    const keypair = Keypair.generate();
    const encoded = bs58.encode(keypair.publicKey.toBytes());

    if (encoded.length === targetLength) {
      tokenMint = new PublicKey(encoded);
      tokenKeypair = keypair
      break;
    }
  }
  return [tokenMint, tokenKeypair];
}


export async function createTokenWithMetadata(
  provider: AnchorProvider,
  payer: Keypair,
  name: string,
  symbol: string,
  uri: string,
  amountToMint: number
): Promise<{ tokenMint: PublicKey, tokenAccount: PublicKey }> {
  
  const connection = provider.connection;
  
const METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
  // const mintKeypair = Keypair.generate();
  const [tokenMint, mintKeypair] = generateSolanaAddressWithLength(43);
  
  const tokenATA = getAssociatedTokenAddressSync(tokenMint, payer.publicKey);
  
  const tokenMetadata = {
    decimals: 9,
    name: name,
    symbol: symbol,
    uri: uri,
    sellerFeeBasisPoints: 0,
    creators: [
      {
        address: payer.publicKey, 
        verified: true,
        share: 100, 
      },
    ],
    collection: null,
    uses: null
  };
  let metadataAddress = PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      METADATA_PROGRAM_ID.toBuffer(),
        tokenMint.toBuffer(),
      ],
      METADATA_PROGRAM_ID
    )[0];
  


  const createMintTransaction = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: tokenMint,
      space: MINT_SIZE,
      lamports: await connection.getMinimumBalanceForRentExemption(MINT_SIZE),
      programId: TOKEN_PROGRAM_ID,
    }),
    createInitializeMintInstruction(
      tokenMint,
      tokenMetadata.decimals,
      payer.publicKey,
      null,
      TOKEN_PROGRAM_ID
    ),
    createAssociatedTokenAccountInstruction(
      payer.publicKey,
      tokenATA,
      payer.publicKey,
      tokenMint
    ),
    createMintToInstruction(
      tokenMint,
      tokenATA,
      payer.publicKey,
      amountToMint
    ),
    mpl.createCreateMetadataAccountV3Instruction(
      {
        metadata: metadataAddress,
        mint: tokenMint,
        mintAuthority: payer.publicKey,
        payer: payer.publicKey,
        updateAuthority: payer.publicKey,
      },
      {
        createMetadataAccountArgsV3: {
          data: tokenMetadata,
          isMutable: false,
          collectionDetails: null,
        },
      }
    )
  );

 const signature= await sendAndConfirmTransaction(connection, createMintTransaction, [payer,mintKeypair]);
  
  console.log("Token mint and metadata successfully created!",signature);

  return {
      tokenMint: tokenMint,
      tokenAccount: tokenATA,
  };
}
  export const POOL_SEED = Buffer.from(anchor.utils.bytes.utf8.encode("pool"));
  export const POOL_VAULT_SEED = Buffer.from(
    anchor.utils.bytes.utf8.encode("pool_vault")
  );
  export const POOL_AUTH_SEED = Buffer.from(
    anchor.utils.bytes.utf8.encode("vault_and_lp_mint_auth_seed")
  );
  export const POOL_LPMINT_SEED = Buffer.from(
    anchor.utils.bytes.utf8.encode("pool_lp_mint")
  );
  export const ORACLE_SEED = Buffer.from(
    anchor.utils.bytes.utf8.encode("observation")
  );
  export async function getAuthAddress(
    programId: PublicKey
  ): Promise<PublicKey> {
    const [address, bump] = await PublicKey.findProgramAddress(
      [POOL_AUTH_SEED],
      programId
    );
    return address;
  }
  
  export async function getPoolAddress(
    ammConfig: PublicKey,
    tokenMint0: PublicKey,
    tokenMint1: PublicKey,
    programId: PublicKey
  ): Promise<PublicKey> {
    const [address, bump] = await PublicKey.findProgramAddress(
      [
        POOL_SEED,
        ammConfig.toBuffer(),
        tokenMint0.toBuffer(),
        tokenMint1.toBuffer(),
      ],
      programId
    );
    return address;
  }
  export async function getPoolLpMintAddress(
    pool: PublicKey,
    programId: PublicKey
  ): Promise<PublicKey> {
    const [address, bump] = await PublicKey.findProgramAddress(
      [POOL_LPMINT_SEED, pool.toBuffer()],
      programId
    );
    return address;
  }
  export async function getPoolVaultAddress(
    pool: PublicKey,
    vaultTokenMint: PublicKey,
    programId: PublicKey
  ): Promise<PublicKey> {
    const [address, bump] = await PublicKey.findProgramAddress(
      [POOL_VAULT_SEED, pool.toBuffer(), vaultTokenMint.toBuffer()],
      programId
    );
    return address;
  }
  export async function getOracleAccountAddress(
    pool: PublicKey,
    programId: PublicKey
  ): Promise<PublicKey> {
    const [address, bump] = await PublicKey.findProgramAddress(
      [ORACLE_SEED, pool.toBuffer()],
      programId
    );
    return address;
  }