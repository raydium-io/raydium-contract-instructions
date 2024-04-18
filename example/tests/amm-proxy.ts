import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  ComputeBudgetProgram,
  Keypair,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Transaction,
} from "@solana/web3.js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

import {
  createAssociatedTokenAccountIfNotExist,
  createMintPair,
  createMarket,
  getAssociatedPoolKeys,
  getMarket,
  sleep,
} from "./util";

import { AmmProxy } from "../target/types/amm_proxy";
import { getAssociatedTokenAddress } from "@solana/spl-token";

const globalInfo = {
  marketProgram: new PublicKey("EoTcMgcDRTJVZDMZWBoU6rhYHZfkNTVEAfz3uUJRcYGj"),
  ammProgram: new PublicKey("HWy1jotHpo6UqeQxx49dpYYdQB8wj9Qk9MdxwjLvDHB8"),
  ammCreateFeeDestination: new PublicKey(
    "3XMrhbv989VxAMi3DErLV9eJht1pHppW5LbKxe9fkEFR"
  ),
  market: new Keypair(),
};

const confirmOptions = {
  skipPreflight: true,
};

describe("amm-proxy", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const owner = anchor.Wallet.local().payer;
  const program = anchor.workspace.AmmProxy as Program<AmmProxy>;
  const marketId = globalInfo.market.publicKey.toString();
  console.log("market:", marketId.toString());
  it("amm anchor test!", async () => {
    let conn = anchor.getProvider().connection;
    const { tokenA, tokenB } = await createMintPair(
      owner,
      anchor.getProvider()
    );
    // create serum market
    const marketInfo = await createMarket({
      connection: conn,
      wallet: anchor.Wallet.local(),
      baseMint: tokenA,
      quoteMint: tokenB,
      baseLotSize: 1,
      quoteLotSize: 1,
      dexProgram: globalInfo.marketProgram,
      market: globalInfo.market,
    });
    // wait for transaction success
    sleep(60000);

    // get serum market info
    const market = await getMarket(
      conn,
      marketId,
      globalInfo.marketProgram.toString()
    );
    // console.log("market info:", JSON.stringify(market));

    const poolKeys = await getAssociatedPoolKeys({
      programId: globalInfo.ammProgram,
      serumProgramId: globalInfo.marketProgram,
      marketId: market.address,
      baseMint: market.baseMint,
      quoteMint: market.quoteMint,
    });
    // console.log("amm poolKeys: ", JSON.stringify(poolKeys));

    const ammAuthority = poolKeys.authority;
    const nonce = poolKeys.nonce;
    const ammId: PublicKey = poolKeys.id;
    const ammCoinVault: PublicKey = poolKeys.baseVault;
    const ammPcVault: PublicKey = poolKeys.quoteVault;
    const lpMintAddress: PublicKey = poolKeys.lpMint;
    const ammTargetOrders: PublicKey = poolKeys.targetOrders;
    const ammOpenOrders: PublicKey = poolKeys.openOrders;

    const [amm_config, _] = await getAmmConfigAddress(globalInfo.ammProgram);
    console.log("amm config:", amm_config.toString());
    /************************************ initialize test ***********************************************************************/

    const transaction = new Transaction();
    const userCoinTokenAccount = await createAssociatedTokenAccountIfNotExist(
      owner.publicKey,
      market.baseMint,
      transaction,
      anchor.getProvider().connection
    );

    const userPcTokenAccount = await createAssociatedTokenAccountIfNotExist(
      owner.publicKey,
      market.quoteMint,
      transaction,
      anchor.getProvider().connection
    );
    if (transaction.instructions.length > 0) {
      const txid = anchor.getProvider().send(transaction, null, {
        skipPreflight: true,
        preflightCommitment: "confirmed",
      });
      console.log("create user lp token account txid:", txid);
    }

    const userLPTokenAccount: PublicKey = await getAssociatedTokenAddress(
      poolKeys.lpMint,
      owner.publicKey
    );

    let tx = await program.methods
      .proxyInitialize(
        nonce,
        new anchor.BN(0),
        new anchor.BN(1000000000), // set as you want
        new anchor.BN(2000000000) // set as you want
      )
      .accounts({
        ammProgram: globalInfo.ammProgram,
        amm: ammId,
        ammAuthority: ammAuthority,
        ammOpenOrders: ammOpenOrders,
        ammLpMint: lpMintAddress,
        ammCoinMint: market.baseMintAddress,
        ammPcMint: market.quoteMintAddress,
        ammCoinVault: ammCoinVault,
        ammPcVault: ammPcVault,
        ammTargetOrders: ammTargetOrders,
        ammConfig: amm_config,
        createFeeDestination: globalInfo.ammCreateFeeDestination,
        marketProgram: globalInfo.marketProgram,
        market: marketId,
        userWallet: owner.publicKey,
        userTokenCoin: userCoinTokenAccount,
        userTokenPc: userPcTokenAccount,
        userTokenLp: userLPTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        sysvarRent: SYSVAR_RENT_PUBKEY,
      })
      .preInstructions([
        ComputeBudgetProgram.setComputeUnitLimit({ units: 1400000 }),
      ])
      .rpc(confirmOptions);
    console.log("initialize tx: ", tx);

    /************************************ deposit test ***********************************************************************/

    tx = await program.methods
      .proxyDeposit(
        new anchor.BN(1000000000), // maxCoinAmount
        new anchor.BN(3000000000), // maxPcAmount
        new anchor.BN(0) // baseSide?
      )
      .accounts({
        ammProgram: globalInfo.ammProgram,
        amm: poolKeys.id,
        ammAuthority: poolKeys.authority,
        ammOpenOrders: poolKeys.openOrders,
        ammTargetOrders: poolKeys.targetOrders,
        ammLpMint: poolKeys.lpMint,
        ammCoinVault: poolKeys.baseVault,
        ammPcVault: poolKeys.quoteVault,
        market: marketId,
        marketEventQueue: market.eventQueue,
        userTokenCoin: userCoinTokenAccount,
        userTokenPc: userPcTokenAccount,
        userTokenLp: userLPTokenAccount,
        userOwner: owner.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc(confirmOptions);
    console.log("deposit tx: ", tx);

    /************************************ withdraw test ***********************************************************************/

    tx = await program.methods
      .proxyWithdraw(
        new anchor.BN(10) // lpAmount
      )
      .accounts({
        ammProgram: globalInfo.ammProgram,
        amm: poolKeys.id,
        ammAuthority: poolKeys.authority,
        ammOpenOrders: poolKeys.openOrders,
        ammTargetOrders: poolKeys.targetOrders,
        ammLpMint: poolKeys.lpMint,
        ammCoinVault: poolKeys.baseVault,
        ammPcVault: poolKeys.quoteVault,
        marketProgram: globalInfo.marketProgram,
        market: marketId,
        marketCoinVault: market.baseVault,
        marketPcVault: market.quoteVault,
        marketVaultSigner: marketInfo.vaultOwner,
        userTokenLp: userLPTokenAccount,
        userTokenCoin: userCoinTokenAccount,
        userTokenPc: userPcTokenAccount,
        userOwner: owner.publicKey,
        marketEventQ: market.eventQueue,
        marketBids: market.bids,
        marketAsks: market.asks,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc(confirmOptions);

    console.log("withdraw tx: ", tx);

    /************************************ swapBaseIn test ********************************************************************** */

    tx = await program.methods
      .proxySwapBaseIn(
        new anchor.BN(10000), // amountIn
        new anchor.BN(1) // amountOut
      )
      .accounts({
        ammProgram: globalInfo.ammProgram,
        amm: poolKeys.id,
        ammAuthority: poolKeys.authority,
        ammOpenOrders: poolKeys.openOrders,
        ammCoinVault: poolKeys.baseVault,
        ammPcVault: poolKeys.quoteVault,
        marketProgram: globalInfo.marketProgram,
        market: marketId,
        marketBids: market.bids,
        marketAsks: market.asks,
        marketEventQueue: market.eventQueue,
        marketCoinVault: market.baseVault,
        marketPcVault: market.quoteVault,
        marketVaultSigner: marketInfo.vaultOwner,
        userTokenSource: userCoinTokenAccount,
        userTokenDestination: userPcTokenAccount,
        userSourceOwner: owner.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc(confirmOptions);
    console.log("swap_base_in tx: ", tx);

    /************************************ swapBaseOut test ***********************************************************************/

    tx = await program.methods
      .proxySwapBaseOut(
        new anchor.BN(10000), // max_amount_in
        new anchor.BN(1) //amount_out
      )
      .accounts({
        ammProgram: globalInfo.ammProgram,
        amm: poolKeys.id,
        ammAuthority: poolKeys.authority,
        ammOpenOrders: poolKeys.openOrders,
        ammCoinVault: poolKeys.baseVault,
        ammPcVault: poolKeys.quoteVault,
        marketProgram: globalInfo.marketProgram,
        market: marketId,
        marketBids: market.bids,
        marketAsks: market.asks,
        marketEventQueue: market.eventQueue,
        marketCoinVault: market.baseVault,
        marketPcVault: market.quoteVault,
        marketVaultSigner: marketInfo.vaultOwner,
        userTokenSource: userCoinTokenAccount,
        userTokenDestination: userPcTokenAccount,
        userSourceOwner: owner.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc(confirmOptions);
    console.log("swap_base_out tx: ", tx);
  });
});

export async function getAmmConfigAddress(
  programId: PublicKey
): Promise<[PublicKey, number]> {
  const [address, bump] = await PublicKey.findProgramAddress(
    [Buffer.from(anchor.utils.bytes.utf8.encode("amm_config_account_seed"))],
    programId
  );
  return [address, bump];
}
