import {
  Account,
  Connection,
  Keypair,
  PublicKey,
  Signer,
  SystemProgram,
  Transaction,
  TransactionSignature,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  createMint,
} from "@solana/spl-token";
import * as anchor from "@coral-xyz/anchor";
import {
  DexInstructions,
  Market as MarketSerum,
  TokenInstructions,
} from "@project-serum/serum";

import {
  Liquidity,
  Market as raydiumSerum,
  Spl,
  SPL_MINT_LAYOUT,
} from "@raydium-io/raydium-sdk";
import { LiquidityAssociatedPoolKeys } from "@raydium-io/raydium-sdk/src/liquidity";

export async function getAssociatedPoolKeys({
  programId,
  serumProgramId,
  marketId,
  baseMint,
  quoteMint,
}: {
  programId: PublicKey;
  serumProgramId: PublicKey;
  marketId: PublicKey;
  baseMint: PublicKey;
  quoteMint: PublicKey;
}): Promise<LiquidityAssociatedPoolKeys> {
  const id = await Liquidity.getAssociatedId({ programId, marketId });
  const lpMint = await Liquidity.getAssociatedLpMint({ programId, marketId });
  const { publicKey: authority, nonce } =
    await Liquidity.getAssociatedAuthority({ programId });
  const baseVault = await Liquidity.getAssociatedBaseVault({
    programId,
    marketId,
  });
  const quoteVault = await Liquidity.getAssociatedQuoteVault({
    programId,
    marketId,
  });
  const lpVault = await Liquidity.getAssociatedLpVault({ programId, marketId });
  const openOrders = await Liquidity.getAssociatedOpenOrders({
    programId,
    marketId,
  });
  const targetOrders = await Liquidity.getAssociatedTargetOrders({
    programId,
    marketId,
  });
  const withdrawQueue = await Liquidity.getAssociatedWithdrawQueue({
    programId,
    marketId,
  });

  const { publicKey: marketAuthority } =
    await raydiumSerum.getAssociatedAuthority({
      programId: serumProgramId,
      marketId,
    });

  return {
    // base
    id,
    baseMint,
    quoteMint,
    lpMint,
    // version
    version: 4,
    programId,
    // keys
    authority,
    nonce,
    baseVault,
    quoteVault,
    lpVault,
    openOrders,
    targetOrders,
    withdrawQueue,
    // market version
    marketVersion: 4,
    marketProgramId: serumProgramId,
    // market keys
    marketId,
    marketAuthority,
  };
}

export async function createAssociatedTokenAccountIfNotExist(
  owner: PublicKey,
  mint: PublicKey,
  transaction: Transaction,
  conn: any
) {
  const associatedAccount = await Spl.getAssociatedTokenAccount({
    mint,
    owner,
  });
  const payer = owner;
  const associatedAccountInfo = await conn.getAccountInfo(associatedAccount);
  if (!associatedAccountInfo) {
    transaction.add(
      Spl.makeCreateAssociatedTokenAccountInstruction({
        mint,
        associatedAccount,
        owner,
        payer,
      })
    );
  }
  return associatedAccount;
}

export async function sendTransaction(
  connection: Connection,
  wallet: any,
  transaction: Transaction,
  signers: Array<Account> = []
) {
  const txid: TransactionSignature = await wallet.sendTransaction(
    transaction,
    connection,
    {
      signers,
      skipPreflight: true,
      preflightCommitment: "confirmed",
    }
  );

  return txid;
}

export function getBigNumber(num: any) {
  return num === undefined || num === null ? 0 : parseFloat(num.toString());
}

export async function getMarket(
  conn: any,
  marketAddress: string,
  serumProgramId: string
): Promise<Market> {
  try {
    const marketAddressPubKey = new PublicKey(marketAddress);
    const market = await Market.load(
      conn,
      marketAddressPubKey,
      undefined,
      new PublicKey(serumProgramId)
    );
    return market;
  } catch (error: any) {
    console.log("get market err: ", error);
    throw error;
  }
}

export class Market extends MarketSerum {
  public baseVault: PublicKey | null = null;
  public quoteVault: PublicKey | null = null;
  public requestQueue: PublicKey | null = null;
  public eventQueue: PublicKey | null = null;
  public bids: PublicKey | null = null;
  public asks: PublicKey | null = null;
  public baseLotSize: number = 0;
  public quoteLotSize: number = 0;
  // private _decoded: any
  public quoteMint: PublicKey | null = null;
  public baseMint: PublicKey | null = null;
  public vaultSignerNonce: Number | null = null;

  static async load(
    connection: Connection,
    address: PublicKey,
    options: any = {},
    programId: PublicKey
  ) {
    const { owner, data } = throwIfNull(
      await connection.getAccountInfo(address),
      "Market not found"
    );
    if (!owner.equals(programId)) {
      throw new Error("Address not owned by program: " + owner.toBase58());
    }
    const decoded = this.getLayout(programId).decode(data);
    if (
      !decoded.accountFlags.initialized ||
      !decoded.accountFlags.market ||
      !decoded.ownAddress.equals(address)
    ) {
      throw new Error("Invalid market");
    }
    const [baseMintDecimals, quoteMintDecimals] = await Promise.all([
      getMintDecimals(connection, decoded.baseMint),
      getMintDecimals(connection, decoded.quoteMint),
    ]);

    const market = new Market(
      decoded,
      baseMintDecimals,
      quoteMintDecimals,
      options,
      programId
    );
    // market._decoded = decoded
    market.baseLotSize = decoded.baseLotSize;
    market.quoteLotSize = decoded.quoteLotSize;
    market.baseVault = decoded.baseVault;
    market.quoteVault = decoded.quoteVault;
    market.requestQueue = decoded.requestQueue;
    market.eventQueue = decoded.eventQueue;
    market.bids = decoded.bids;
    market.asks = decoded.asks;
    market.quoteMint = decoded.quoteMint;
    market.baseMint = decoded.baseMint;
    market.vaultSignerNonce = decoded.vaultSignerNonce;
    return market;
  }
}

export async function getMintDecimals(
  connection: Connection,
  mint: PublicKey
): Promise<number> {
  const { data } = throwIfNull(
    await connection.getAccountInfo(mint),
    "mint not found"
  );
  const { decimals } = SPL_MINT_LAYOUT.decode(data);
  return decimals;
}

function throwIfNull<T>(value: T | null, message = "account not found"): T {
  if (value === null) {
    throw new Error(message);
  }
  return value;
}

export async function getFilteredTokenAccountsByOwner(
  connection: Connection,
  programId: PublicKey,
  mint: PublicKey
): Promise<{ context: {}; value: [] }> {
  // @ts-ignore
  const resp = await connection._rpcRequest("getTokenAccountsByOwner", [
    programId.toBase58(),
    {
      mint: mint.toBase58(),
    },
    {
      encoding: "jsonParsed",
    },
  ]);
  if (resp.error) {
    throw new Error(resp.error.message);
  }
  return resp.result;
}

export async function checkTxid(conn: Connection, txid: any) {
  let txidSuccessFlag = 0;
  await conn.onSignature(txid, function (_signatureResult: any, _context: any) {
    if (_signatureResult.err) {
      txidSuccessFlag = -1;
    } else {
      txidSuccessFlag = 1;
    }
  });

  const timeAwait = new Date().getTime();
  let outOfWhile = false;
  while (!outOfWhile) {
    console.log(
      "wait txid:",
      txid,
      outOfWhile,
      txidSuccessFlag,
      (new Date().getTime() - timeAwait) / 1000
    );
    if (txidSuccessFlag !== 0) {
      outOfWhile = true;
    }
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }
  if (txidSuccessFlag !== 1) {
    throw new Error("Transaction failed");
  }
}

export async function createMintPair(
  wallet: Signer,
  provider: anchor.Provider
) {
  const connection = provider.connection;
  const mintAuthority = Keypair.generate();
  const tokenA = await createMint(
    connection,
    wallet,
    mintAuthority.publicKey,
    null,
    9
  );
  const tokenB = await createMint(
    connection,
    wallet,
    mintAuthority.publicKey,
    null,
    9
  );

  const ownerTokenAAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    wallet,
    tokenA,
    wallet.publicKey,
    false,
    "processed",
    undefined,
    TOKEN_PROGRAM_ID
  );

  await mintTo(
    connection,
    wallet,
    tokenA,
    ownerTokenAAccount.address,
    mintAuthority,
    100_000_000_000_000,
    [],
    { skipPreflight: true },
    TOKEN_PROGRAM_ID
  );

  const ownerTokenBAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    wallet,
    tokenB,
    wallet.publicKey,
    false,
    "processed",
    undefined,
    TOKEN_PROGRAM_ID
  );
  await mintTo(
    connection,
    wallet,
    tokenB,
    ownerTokenBAccount.address,
    mintAuthority,
    100_000_000_000_000,
    [],
    { skipPreflight: true },
    TOKEN_PROGRAM_ID
  );

  console.log(
    "create tokenA: ",
    tokenA.toString(),
    " tokenB: ",
    tokenB.toString()
  );
  return { tokenA, tokenB };
}

export async function createMarket({
  connection,
  wallet,
  baseMint,
  quoteMint,
  baseLotSize,
  quoteLotSize,
  dexProgram,
  market,
}: {
  connection: Connection;
  wallet: anchor.Wallet;
  baseMint: PublicKey;
  quoteMint: PublicKey;
  baseLotSize: number;
  quoteLotSize: number;
  dexProgram: PublicKey;
  market: Keypair;
}): Promise<SerumMarketInfo> {
  const requestQueue = new Keypair();
  const eventQueue = new Keypair();
  const bids = new Keypair();
  const asks = new Keypair();
  const baseVault = new Keypair();
  const quoteVault = new Keypair();
  const feeRateBps = 0;
  const quoteDustThreshold = new anchor.BN(10);
  const { vaultOwner, vaultNonce } = await getVaultOwnerAndNonce(
    market.publicKey,
    dexProgram
  );

  const tx1 = new Transaction();
  tx1.add(
    SystemProgram.createAccount({
      fromPubkey: wallet.publicKey,
      newAccountPubkey: baseVault.publicKey,
      lamports: await connection.getMinimumBalanceForRentExemption(165),
      space: 165,
      programId: TokenInstructions.TOKEN_PROGRAM_ID,
    }),
    SystemProgram.createAccount({
      fromPubkey: wallet.publicKey,
      newAccountPubkey: quoteVault.publicKey,
      lamports: await connection.getMinimumBalanceForRentExemption(165),
      space: 165,
      programId: TokenInstructions.TOKEN_PROGRAM_ID,
    }),
    TokenInstructions.initializeAccount({
      account: baseVault.publicKey,
      mint: baseMint,
      owner: vaultOwner,
    }),
    TokenInstructions.initializeAccount({
      account: quoteVault.publicKey,
      mint: quoteMint,
      owner: vaultOwner,
    })
  );

  const tx2 = new Transaction();
  tx2.add(
    SystemProgram.createAccount({
      fromPubkey: wallet.publicKey,
      newAccountPubkey: market.publicKey,
      lamports: await connection.getMinimumBalanceForRentExemption(
        Market.getLayout(dexProgram).span
      ),
      space: Market.getLayout(dexProgram).span,
      programId: dexProgram,
    }),
    SystemProgram.createAccount({
      fromPubkey: wallet.publicKey,
      newAccountPubkey: requestQueue.publicKey,
      lamports: await connection.getMinimumBalanceForRentExemption(5120 + 12),
      space: 5120 + 12,
      programId: dexProgram,
    }),
    SystemProgram.createAccount({
      fromPubkey: wallet.publicKey,
      newAccountPubkey: eventQueue.publicKey,
      lamports: await connection.getMinimumBalanceForRentExemption(262144 + 12),
      space: 262144 + 12,
      programId: dexProgram,
    }),
    SystemProgram.createAccount({
      fromPubkey: wallet.publicKey,
      newAccountPubkey: bids.publicKey,
      lamports: await connection.getMinimumBalanceForRentExemption(65536 + 12),
      space: 65536 + 12,
      programId: dexProgram,
    }),
    SystemProgram.createAccount({
      fromPubkey: wallet.publicKey,
      newAccountPubkey: asks.publicKey,
      lamports: await connection.getMinimumBalanceForRentExemption(65536 + 12),
      space: 65536 + 12,
      programId: dexProgram,
    }),
    DexInstructions.initializeMarket({
      market: market.publicKey,
      requestQueue: requestQueue.publicKey,
      eventQueue: eventQueue.publicKey,
      bids: bids.publicKey,
      asks: asks.publicKey,
      baseVault: baseVault.publicKey,
      quoteVault: quoteVault.publicKey,
      baseMint,
      quoteMint,
      baseLotSize: new anchor.BN(baseLotSize),
      quoteLotSize: new anchor.BN(quoteLotSize),
      feeRateBps,
      vaultSignerNonce: vaultNonce,
      quoteDustThreshold,
      programId: dexProgram,
      authority: undefined,
    })
  );

  const signedTransactions = await signTransactions({
    transactionsAndSigners: [
      { transaction: tx1, signers: [baseVault, quoteVault] },
      {
        transaction: tx2,
        signers: [market, requestQueue, eventQueue, bids, asks],
      },
    ],
    wallet: wallet,
    connection: connection,
  });
  for (let signedTransaction of signedTransactions) {
    await sendSignedTransaction({
      signedTransaction,
      connection: connection,
    });
  }

  return {
    market: market.publicKey,
    requestQueue: requestQueue.publicKey,
    eventQueue: eventQueue.publicKey,
    bids: bids.publicKey,
    asks: asks.publicKey,
    baseVault: baseVault.publicKey,
    quoteVault: quoteVault.publicKey,
    baseMint,
    quoteMint,
    baseLotSize: new anchor.BN(baseLotSize),
    quoteLotSize: new anchor.BN(quoteLotSize),
    feeRateBps,
    vaultOwner,
    vaultSignerNonce: vaultNonce,
    quoteDustThreshold,
    programId: dexProgram,
    // authority: undefined,
  };
}

export interface SerumMarketInfo {
  market: PublicKey;
  requestQueue: PublicKey;
  eventQueue: PublicKey;
  bids: PublicKey;
  asks: PublicKey;
  baseVault: PublicKey;
  quoteVault: PublicKey;
  baseMint: PublicKey;
  quoteMint: PublicKey;
  baseLotSize: anchor.BN;
  quoteLotSize: anchor.BN;
  feeRateBps: number;
  vaultOwner: PublicKey;
  vaultSignerNonce: anchor.BN;
  quoteDustThreshold: any;
  programId: any;
}

export async function getVaultOwnerAndNonce(
  marketId: PublicKey,
  dexProgramId: PublicKey
) {
  const vaultNonce = new anchor.BN(0);
  while (true) {
    try {
      const vaultOwner = await PublicKey.createProgramAddress(
        [marketId.toBuffer(), vaultNonce.toArrayLike(Buffer, "le", 8)],
        dexProgramId
      );
      return { vaultOwner, vaultNonce };
    } catch (e) {
      vaultNonce.iaddn(1);
    }
  }
}

export async function signTransactions({
  transactionsAndSigners,
  wallet,
  connection,
}: {
  transactionsAndSigners: {
    transaction: Transaction;
    signers?: Array<Keypair>;
  }[];
  wallet: anchor.Wallet;
  connection: Connection;
}) {
  const blockhash = (await connection.getRecentBlockhash("max")).blockhash;
  transactionsAndSigners.forEach(({ transaction, signers = [] }) => {
    transaction.recentBlockhash = blockhash;
    transaction.setSigners(
      wallet.publicKey,
      ...signers.map((s) => s.publicKey)
    );
    if (signers?.length > 0) {
      transaction.partialSign(...signers);
    }
  });
  return await wallet.signAllTransactions(
    transactionsAndSigners.map(({ transaction }) => transaction)
  );
}

export async function sendSignedTransaction({
  signedTransaction,
  connection,
  timeout = 10000,
}: {
  signedTransaction: Transaction;
  connection: Connection;
  timeout?: number;
}): Promise<string> {
  const rawTransaction = signedTransaction.serialize();
  const startTime = getUnixTs();

  const txid: TransactionSignature = await connection.sendRawTransaction(
    rawTransaction,
    {
      skipPreflight: true,
    }
  );

  console.log("txid:", txid);
  await sleep(timeout);
  console.log("Latency", txid, getUnixTs() - startTime);
  return txid;
}

export const getUnixTs = () => {
  return new Date().getTime() / 1000;
};

export async function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
