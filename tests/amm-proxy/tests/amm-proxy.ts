import BigNumber from 'bignumber.js'
import * as anchor from "@project-serum/anchor";
import {Program} from "@project-serum/anchor";
import {Keypair, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, Transaction} from '@solana/web3.js'
import {TOKEN_PROGRAM_ID} from '@solana/spl-token';
import {closeAccount, initializeAccount} from '@project-serum/serum/lib/token-instructions'
import {getMultipleAccountsInfo, Spl, SPL_ACCOUNT_LAYOUT, SPL_MINT_LAYOUT, WSOL,} from "@raydium-io/raydium-sdk";
import {
    createAssociatedTokenAccountIfNotExist,
    createMintPair,
    createSerumMarket,
    getAssociatedPoolKeys,
    getBigNumber,
    getFilteredTokenAccountsByOwner,
    getMarket,
    getMintDecimals,
    getVaultOwnerAndNonce,
    sleep
} from "./util";

import {AmmProxy} from "../target/types/amm_proxy";

const marketInfo = {
    serumDexProgram: new PublicKey("z678mfRyG19BTfAJm5fcRh6hWJfNBbN1Gv9kvPCqvLA"),
    ammProgram: new PublicKey("C21qJRTdoAh6Jk9AvpB51Rt2CMi8xkFcGUX6LeG3QTVT"),
    serumMarket: new Keypair(),
}



describe("amm-proxy", () => {
        const provider = anchor.Provider.env();
        anchor.setProvider(provider);
        const program = anchor.workspace.AmmProxy as Program<AmmProxy>;
        const serumMarketId = marketInfo.serumMarket.publicKey.toString()

        it("amm anchor test!", async () => {
            let conn = provider.connection
            const owner = provider.wallet.publicKey

            let alreadCreatedMarket = false
            let multipleInfo = await getMultipleAccountsInfo(conn, [new PublicKey(serumMarketId)])
            if (multipleInfo.length > 0 && multipleInfo[0] !== null) {
                if (multipleInfo[0]?.data.length !== 0) {
                    alreadCreatedMarket = true
                }
            }
            console.log("alreadCreatedMarket:", alreadCreatedMarket)
            if (!alreadCreatedMarket) {
                const {tokenAMintAddress, tokenBMintAddress} = await createMintPair(provider)
                // create serum market
                const createMarketInfo = await createSerumMarket({
                    connection: provider.connection,
                    wallet: provider.wallet,
                    baseMint: tokenAMintAddress,
                    quoteMint: tokenBMintAddress,
                    baseLotSize: 1,
                    quoteLotSize: 1,
                    dexProgram: marketInfo.serumDexProgram,
                    market: marketInfo.serumMarket,
                })
                console.log(JSON.stringify(createMarketInfo))
                // wait for transaction success
                sleep(3000)
            }

            // get serum market info
            const market = await getMarket(conn, serumMarketId, marketInfo.serumDexProgram.toString())
            console.log("serum market info:", JSON.stringify(market))
            const poolKeys = await getAssociatedPoolKeys({
                programId: marketInfo.ammProgram,
                serumProgramId: marketInfo.serumDexProgram,
                marketId: market.address,
                baseMint: market.baseMint,
                quoteMint: market.quoteMint
            })
            console.log("amm poolKeys: ", JSON.stringify(poolKeys))

            const ammAuthority = poolKeys.authority
            const nonce = new anchor.BN(poolKeys.nonce)
            const ammId: PublicKey = poolKeys.id
            const poolCoinTokenAccount: PublicKey = poolKeys.baseVault
            const poolPcTokenAccount: PublicKey = poolKeys.quoteVault
            const lpMintAddress: PublicKey = poolKeys.lpMint
            const poolTempLpTokenAccount: PublicKey = poolKeys.lpVault
            const ammTargetOrders: PublicKey = poolKeys.targetOrders
            const poolWithdrawQueue: PublicKey = poolKeys.withdrawQueue
            const ammOpenOrders: PublicKey = poolKeys.openOrders

            let alreadPreInitialized = false
            multipleInfo = await getMultipleAccountsInfo(conn, [lpMintAddress])
            if (multipleInfo.length > 0 && multipleInfo[0] !== null) {
                const tempLpMint = SPL_MINT_LAYOUT.decode(multipleInfo[0]?.data)
                if (getBigNumber(tempLpMint.supply) !== 0) {
                    alreadPreInitialized = true
                }
            }
            console.log("alreadPreInitialized: ", alreadPreInitialized)
            if (!alreadPreInitialized) {
                /************************************ preInitialize test ***********************************************************************/
                const tx = await program.rpc.proxyPreInitialize(nonce.toNumber(), {
                    accounts: {
                        ammProgram: marketInfo.ammProgram,
                        ammTargetOrders: ammTargetOrders,
                        poolWithdrawQueue: poolWithdrawQueue,
                        ammAuthority: ammAuthority,
                        lpMint: lpMintAddress,
                        coinMint: market.baseMintAddress,
                        pcMint: market.quoteMintAddress,
                        poolCoinTokenAccount: poolCoinTokenAccount,
                        poolPcTokenAccount: poolPcTokenAccount,
                        poolTempLpTokenAccount: poolTempLpTokenAccount,
                        serumMarket: market.address,
                        userWallet: owner,
                        splTokenProgram: TOKEN_PROGRAM_ID,
                        systemProgram: SystemProgram.programId,
                        rent: SYSVAR_RENT_PUBKEY,
                    }
                });
                console.log("preinitialize tx: ", tx);
                sleep(60000)
            }


            let alreadInitialized = false
            multipleInfo = await getMultipleAccountsInfo(conn, [ammId])
            if (multipleInfo.length > 0 && multipleInfo[0] !== null) {
                if (multipleInfo[0]?.data.length !== 0) {
                    alreadInitialized = true
                }
            }
            console.log("alreadInitialized:", alreadInitialized)
            if (!alreadInitialized) {
                /************************************ initialize test ***********************************************************************/

                    // set as you want
                const userInputBaseValue = 1
                const userInputQuoteValue = 2

                await initAmm(
                    conn,
                    provider,
                    market,
                    userInputBaseValue,
                    userInputQuoteValue,
                    poolCoinTokenAccount,
                    poolPcTokenAccount,
                    lpMintAddress,
                )

                // belongs to owner who create the pool
                const userLpTokenAccountPubKey = await Spl.getAssociatedTokenAccount({mint: lpMintAddress, owner: owner})
                let tx = await program.rpc.proxyInitialize(nonce, new anchor.BN(0), {
                    accounts: {
                        ammProgram: marketInfo.ammProgram,
                        amm: ammId,
                        ammAuthority: ammAuthority,
                        ammOpenOrders: ammOpenOrders,
                        lpMint: lpMintAddress,
                        coinMint: market.baseMintAddress,
                        pcMint: market.quoteMintAddress,
                        poolCoinTokenAccount: poolCoinTokenAccount,
                        poolPcTokenAccount: poolPcTokenAccount,
                        poolWithdrawQueue: poolWithdrawQueue,
                        poolTargetOrdersAccount: ammTargetOrders,
                        poolLpTokenAccount: userLpTokenAccountPubKey,
                        poolTempLpTokenAccount: poolTempLpTokenAccount,
                        serumProgram: marketInfo.serumDexProgram,
                        serumMarket: serumMarketId,
                        userWallet: owner,
                        splTokenProgram: TOKEN_PROGRAM_ID,
                        systemProgram: SystemProgram.programId,
                        rent: SYSVAR_RENT_PUBKEY,
                    }
                });
                console.log("initialize tx: ", tx);
                sleep(30000)
            }

            /************************************ deposit test ***********************************************************************/

            const transaction = new Transaction();
            const userCoinTokenAccount = await createAssociatedTokenAccountIfNotExist(
                provider.wallet.publicKey,
                market.baseMint,
                transaction,
                provider.connection
            )

            const userPcTokenAccount = await createAssociatedTokenAccountIfNotExist(
                provider.wallet.publicKey,
                market.quoteMint,
                transaction,
                provider.connection
            )

            const userLPTokenAccount: PublicKey = await createAssociatedTokenAccountIfNotExist(
                provider.wallet.publicKey,
                poolKeys.lpMint,
                transaction,
                provider.connection
            )

            if (transaction.instructions.length > 0) {
                const txid = provider.send(transaction, null, {
                    skipPreflight: true,
                    preflightCommitment: "confirmed"
                })
                console.log("create user lp token account txid:", txid)
                sleep(3000)
                // checkTxid(provider.connection, txid)
            }

            let tx = await program.rpc.proxyDeposit(
                new anchor.BN(1000000000), // maxCoinAmount
                new anchor.BN(2000000000), // maxPcAmount
                new anchor.BN(1), // baseSide?
                {
                    accounts: {
                        ammProgram: marketInfo.ammProgram,
                        amm: poolKeys.id,
                        ammAuthority: poolKeys.authority,
                        ammOpenOrders: poolKeys.openOrders,
                        ammTargetOrders: poolKeys.targetOrders,
                        lpMint: poolKeys.lpMint,
                        poolCoinTokenAccount: poolKeys.baseVault,
                        poolPcTokenAccount: poolKeys.quoteVault,
                        serumMarket: serumMarketId,
                        userCoinTokenAccount: userCoinTokenAccount,
                        userPcTokenAccount: userPcTokenAccount,
                        userLpTokenAccount: userLPTokenAccount,
                        userOwner: provider.wallet.publicKey,
                        splTokenProgram: TOKEN_PROGRAM_ID,
                    },
                })
            console.log("deposit tx: ", tx)
            sleep(3000)


            /************************************ withdraw test ***********************************************************************/

            const {
                vaultOwner,
                vaultNonce
            } = await getVaultOwnerAndNonce(new PublicKey(serumMarketId), marketInfo.serumDexProgram)
            if (vaultNonce.toNumber() != market.vaultSignerNonce) {
                console.log("withdraw vaultOwner:", vaultOwner.toString(), "vaultNonce: ", vaultNonce.toNumber(), "market.vaultSignerNonce:", market.vaultSignerNonce.toString())
                throw("vaultSignerNonce incorrect!")
            }
            tx = await program.rpc.proxyWithdraw(
                new anchor.BN(1000), // lpAmount
                {
                    accounts: {
                        ammProgram: marketInfo.ammProgram,
                        amm: poolKeys.id,
                        ammAuthority: poolKeys.authority,
                        ammOpenOrders: poolKeys.openOrders,
                        ammTargetOrders: poolKeys.targetOrders,
                        lpMint: poolKeys.lpMint,
                        poolCoinTokenAccount: poolKeys.baseVault,
                        poolPcTokenAccount: poolKeys.quoteVault,
                        poolWithdrawQueue: poolKeys.withdrawQueue,
                        poolTempLpTokenAccount: poolKeys.lpVault,
                        serumProgram: marketInfo.serumDexProgram,
                        serumMarket: serumMarketId,
                        serumCoinVaultAccount: market.baseVault,
                        serumPcVaultAccount: market.quoteVault,
                        serumVaultSigner: vaultOwner,
                        userCoinTokenAccount: userCoinTokenAccount,
                        userPcTokenAccount: userPcTokenAccount,
                        userLpTokenAccount: userLPTokenAccount,
                        userOwner: provider.wallet.publicKey,
                        serumEventQ: market.eventQueue,
                        serumBids: market.bids,
                        serumAsks: market.asks,
                        splTokenProgram: TOKEN_PROGRAM_ID,
                    },
                })

            console.log("withdraw tx: ", tx)
            sleep(3000)

            /************************************ swapBaseIn test ********************************************************************** */

            tx = await program.rpc.proxySwapBaseIn(
                new anchor.BN(100), // amountIn
                new anchor.BN(100), // amountOut
                {
                    accounts: {
                        ammProgram: marketInfo.ammProgram,
                        amm: poolKeys.id,
                        ammAuthority: poolKeys.authority,
                        ammOpenOrders: poolKeys.openOrders,
                        ammTargetOrders: poolKeys.targetOrders,
                        poolCoinTokenAccount: poolKeys.baseVault,
                        poolPcTokenAccount: poolKeys.quoteVault,
                        serumProgram: marketInfo.serumDexProgram,
                        serumMarket: serumMarketId,
                        serumBids: market.bids,
                        serumAsks: market.asks,
                        serumEventQueue: market.eventQueue,
                        serumCoinVaultAccount: market.baseVault,
                        serumPcVaultAccount: market.quoteVault,
                        serumVaultSigner: vaultOwner,
                        userSourceTokenAccount: userCoinTokenAccount,
                        userDestinationTokenAccount: userPcTokenAccount,
                        userSourceOwner: provider.wallet.publicKey,
                        splTokenProgram: TOKEN_PROGRAM_ID,
                    },
                })
            console.log("swap_base_in tx: ", tx)
            sleep(3000)

            /************************************ swapBaseOut test ***********************************************************************/

            tx = await program.rpc.proxySwapBaseOut(
                new anchor.BN(100), // max_amount_in
                new anchor.BN(100), //amount_out
                {
                    accounts: {
                        ammProgram: marketInfo.ammProgram,
                        amm: poolKeys.id,
                        ammAuthority: poolKeys.authority,
                        ammOpenOrders: poolKeys.openOrders,
                        ammTargetOrders: poolKeys.targetOrders,
                        poolCoinTokenAccount: poolKeys.baseVault,
                        poolPcTokenAccount: poolKeys.quoteVault,
                        serumProgram: marketInfo.serumDexProgram,
                        serumMarket: serumMarketId,
                        serumBids: market.bids,
                        serumAsks: market.asks,
                        serumEventQueue: market.eventQueue,
                        serumCoinVaultAccount: market.baseVault,
                        serumPcVaultAccount: market.quoteVault,
                        serumVaultSigner: vaultOwner,
                        userSourceTokenAccount: userCoinTokenAccount,
                        userDestinationTokenAccount: userPcTokenAccount,
                        userSourceOwner: provider.wallet.publicKey,
                        splTokenProgram: TOKEN_PROGRAM_ID,
                    },
                })
            console.log("swap_base_out tx: ", tx)
        });

    }
);

async function initAmm(
    conn: any,
    provider: anchor.Provider,
    market: any,
    userInputBaseValue: number,
    userInputQuoteValue: number,
    poolCoinTokenAccount: PublicKey,
    poolPcTokenAccount: PublicKey,
    lpMintAddress: PublicKey,
) {
    const baseMintDecimals = new BigNumber(await getMintDecimals(conn, market.baseMintAddress as PublicKey))
    const quoteMintDecimals = new BigNumber(await getMintDecimals(conn, market.quoteMintAddress as PublicKey))
    const coinVol = new BigNumber(10).exponentiatedBy(baseMintDecimals).multipliedBy(userInputBaseValue)
    const pcVol = new BigNumber(10).exponentiatedBy(quoteMintDecimals).multipliedBy(userInputQuoteValue)
    const transaction = new Transaction()
    const signers: any = []
    const owner = provider.wallet.publicKey
    const baseTokenAccount = await getFilteredTokenAccountsByOwner(conn, owner, market.baseMintAddress)
    const quoteTokenAccount = await getFilteredTokenAccountsByOwner(conn, owner, market.quoteMintAddress)
    const baseTokenList: any = baseTokenAccount.value.map((item: any) => {
        if (item.account.data.parsed.info.tokenAmount.amount >= getBigNumber(coinVol)) {
            return item.pubkey
        }
        return null
    })
    const quoteTokenList: any = quoteTokenAccount.value.map((item: any) => {
        if (item.account.data.parsed.info.tokenAmount.amount >= getBigNumber(pcVol)) {
            return item.pubkey
        }
        return null
    })
    let baseToken: string | null = null
    for (const item of baseTokenList) {
        if (item !== null) {
            baseToken = item
        }
    }
    let quoteToken: string | null = null
    for (const item of quoteTokenList) {
        if (item !== null) {
            quoteToken = item
        }
    }
    if (
        (baseToken === null && market.baseMintAddress.toString() !== WSOL.mint) ||
        (quoteToken === null && market.quoteMintAddress.toString() !== WSOL.mint)
    ) {
        throw new Error('no money')
    }

    const destLpToken: PublicKey = await createAssociatedTokenAccountIfNotExist(
        owner,
        lpMintAddress,
        transaction,
        conn
    )

    if (market.baseMintAddress.toString() === WSOL.mint) {
        const newAccount = new Keypair()
        transaction.add(
            SystemProgram.createAccount({
                fromPubkey: owner,
                newAccountPubkey: newAccount.publicKey,
                lamports: parseInt(coinVol.toFixed()) + 1e7,
                space: SPL_ACCOUNT_LAYOUT.span,
                programId: TOKEN_PROGRAM_ID
            })
        )
        transaction.add(
            initializeAccount({
                account: newAccount.publicKey,
                mint: new PublicKey(WSOL.mint),
                owner
            })
        )

        transaction.add(Spl.makeTransferInstruction({
            source: newAccount.publicKey,
            destination: poolCoinTokenAccount,
            owner: owner,
            amount: parseInt(coinVol.toFixed())
        }))

        transaction.add(
            closeAccount({
                source: newAccount.publicKey,
                destination: owner,
                owner
            })
        )

        signers.push(newAccount)
    } else {
        transaction.add(
            Spl.makeTransferInstruction({
                source: new PublicKey(baseToken),
                destination: poolCoinTokenAccount,
                owner: owner,
                amount: parseInt(coinVol.toFixed())
            })
        )
    }
    if (market.quoteMintAddress.toString() === WSOL.mint) {
        const newAccount = new Keypair()
        transaction.add(
            SystemProgram.createAccount({
                fromPubkey: owner,
                newAccountPubkey: newAccount.publicKey,
                lamports: parseInt(pcVol.toFixed()) + 1e7,
                space: SPL_ACCOUNT_LAYOUT.span,
                programId: TOKEN_PROGRAM_ID
            })
        )
        transaction.add(
            initializeAccount({
                account: newAccount.publicKey,
                mint: new PublicKey(WSOL.mint),
                owner
            })
        )
        transaction.add(Spl.makeTransferInstruction({
            source: newAccount.publicKey,
            destination: poolPcTokenAccount,
            owner: owner,
            amount: parseInt(pcVol.toFixed())
        }));

        transaction.add(
            closeAccount({
                source: newAccount.publicKey,
                destination: owner,
                owner
            })
        )
        signers.push(newAccount)
    } else {
        transaction.add(
            Spl.makeTransferInstruction({
                source: new PublicKey(quoteToken),
                destination: poolPcTokenAccount,
                owner: owner,
                amount: parseInt(pcVol.toFixed())
            })
        )
    }

    const txid = await provider.send(transaction, signers, {
        skipPreflight: true,
        preflightCommitment: "confirmed"
    })
    console.log("initAMM txid:", txid)
    sleep(3000)
    // checkTxid(conn, txid)
}

