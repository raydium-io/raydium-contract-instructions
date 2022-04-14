
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::Transaction;
use solana_sdk::commitment_config::CommitmentConfig;

use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSendTransactionConfig;

use spl_associated_token_account::{get_associated_token_address};

use anyhow::{format_err, Result};

use std::str::FromStr;


use raydium_contract_instructions::{
    amm_instruction::{ID as ammProgramID, swap_base_in as amm_swap},
    stable_instruction::{ID as stableProgramID, swap_base_in as stable_swap},
};


fn read_keypair_file(s: &str) -> Result<Keypair> {
    solana_sdk::signature::read_keypair_file(s)
        .map_err(|_| format_err!("failed to read keypair from {}", s))
}


fn main() -> Result<()> {
    println!("start ...");
    
    let rpc = RpcClient::new("https://api.mainnet-beta.solana.com".to_string() );

    let payer = read_keypair_file("id.json")?;

    // should create the spl ata accounts first if not exist
    let user_ray_account = get_associated_token_address(&payer.pubkey(), 
        &Pubkey::from_str("4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R")?);
        
    let user_usdc_account = get_associated_token_address(&payer.pubkey(), 
        &Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")?);
    
    let user_usdt_account = get_associated_token_address(&payer.pubkey(), 
        &Pubkey::from_str("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB")?);


    
    // https://api.raydium.io/v2/sdk/liquidity/mainnet.json
    // RAY-USDC
    // {
    //     "id":"6UmmUiYoBjSrhakAobJw8BvkmJtDVxaeBtbt7rxWo1mg",
    //     "baseMint":"4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R",
    //     "quoteMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    //     "lpMint":"FbC6K13MzHvN42bXrtGaWsvZY9fxrackRSZcBGfjPc7m",
    //     "baseDecimals":6,"quoteDecimals":6,"lpDecimals":6,"version":4,
    //     "programId":"675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8",
    //     "authority":"5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1",
    //     "openOrders":"J8u8nTHYtvudyqwLrXZboziN95LpaHFHpd97Jm5vtbkW",
    //     "targetOrders":"3cji8XW5uhtsA757vELVFAeJpskyHwbnTSceMFY5GjVT",
    //     "baseVault":"FdmKUE4UMiJYFK5ogCngHzShuVKrFXBamPWcewDr31th",
    //     "quoteVault":"Eqrhxd7bDUCH3MepKmdVkgwazXRzY6iHhEoBpY7yAohk",
    //     "withdrawQueue":"ERiPLHrxvjsoMuaWDWSTLdCMzRkQSo8SkLBLYEmSokyr",
    //     "lpVault":"D1V5GMf3N26owUFcbz2qR5N4G81qPKQvS2Vc4SM73XGB",
    //     "marketVersion":3,
    //     "marketProgramId":"9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin",
    //     "marketId":"2xiv8A5xrJ7RnGdxXB42uFEkYHJjszEhaJyKKt4WaLep",
    //     "marketAuthority":"FmhXe9uG6zun49p222xt3nG1rBAkWvzVz7dxERQ6ouGw",
    //     "marketBaseVault":"GGcdamvNDYFhAXr93DWyJ8QmwawUHLCyRqWL3KngtLRa",
    //     "marketQuoteVault":"22jHt5WmosAykp3LPGSAKgY45p7VGh4DFWSwp21SWBVe",
    //     "marketBids":"Hf84mYadE1VqSvVWAvCWc9wqLXak4RwXiPb4A91EAUn5",
    //     "marketAsks":"DC1HsWWRCXVg3wk2NndS5LTbce3axwUwUZH1RgnV4oDN",
    //     "marketEventQueue":"H9dZt8kvz1Fe5FyRisb77KcYTaN8LEbuVAfJSnAaEABz"
    // },

    let instr = amm_swap(
        &ammProgramID,
        &Pubkey::from_str("6UmmUiYoBjSrhakAobJw8BvkmJtDVxaeBtbt7rxWo1mg")?,
        &Pubkey::from_str("5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1")?,
        &Pubkey::from_str("J8u8nTHYtvudyqwLrXZboziN95LpaHFHpd97Jm5vtbkW")?,
        &Pubkey::from_str("3cji8XW5uhtsA757vELVFAeJpskyHwbnTSceMFY5GjVT")?,
        &Pubkey::from_str("FdmKUE4UMiJYFK5ogCngHzShuVKrFXBamPWcewDr31th")?,
        &Pubkey::from_str("Eqrhxd7bDUCH3MepKmdVkgwazXRzY6iHhEoBpY7yAohk")?,

        &Pubkey::from_str("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin")?,
        &Pubkey::from_str("2xiv8A5xrJ7RnGdxXB42uFEkYHJjszEhaJyKKt4WaLep")?,
        &Pubkey::from_str("Hf84mYadE1VqSvVWAvCWc9wqLXak4RwXiPb4A91EAUn5")?,
        &Pubkey::from_str("DC1HsWWRCXVg3wk2NndS5LTbce3axwUwUZH1RgnV4oDN")?,
        &Pubkey::from_str("H9dZt8kvz1Fe5FyRisb77KcYTaN8LEbuVAfJSnAaEABz")?,

        &Pubkey::from_str("GGcdamvNDYFhAXr93DWyJ8QmwawUHLCyRqWL3KngtLRa")?,
        &Pubkey::from_str("22jHt5WmosAykp3LPGSAKgY45p7VGh4DFWSwp21SWBVe")?,
        &Pubkey::from_str("FmhXe9uG6zun49p222xt3nG1rBAkWvzVz7dxERQ6ouGw")?,

        &user_ray_account,
        &user_usdc_account,
        &payer.pubkey(),

        1000000,
        2000000
    )?;
    

    let instructions = vec![instr];
    let signers = vec![&payer];
    let recent_hash = rpc.get_latest_blockhash()?;
    let txn = Transaction::new_signed_with_payer(
        &instructions,
        Some(&payer.pubkey()),
        &signers,
        recent_hash,
    );

    let signature = rpc.send_and_confirm_transaction_with_spinner_and_config(
        &txn,
        CommitmentConfig::confirmed(),
        RpcSendTransactionConfig {
            skip_preflight: true,
            ..RpcSendTransactionConfig::default()
        },
    )?;

    println!("amm swap send txn: {}.", signature);


    // https://api.raydium.io/v2/sdk/liquidity/mainnet.json
    // USDT-USDC stable
    // {
    //     "id":"2EXiumdi14E9b8Fy62QcA5Uh6WdHS2b38wtSxp72Mibj",
    //     "baseMint":"Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB",
    //     "quoteMint":"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    //     "lpMint":"As3EGgLtUVpdNpE6WCKauyNRrCCwcQ57trWQ3wyRXDa6",
    //     "baseDecimals":6,
    //     "quoteDecimals":6,
    //     "lpDecimals":6,
    //     "version":5,
    //     "programId":"5quBtoiQqxF9Jv6KYKctB59NT3gtJD2Y65kdnB1Uev3h",
    //     "authority":"3uaZBfHPfmpAHW7dsimC1SnyR61X4bJqQZKWmRSCXJxv",
    //     "openOrders":"4zbGjjRx8bmZjynJg2KnkJ54VAk1crcrYsGMy79EXK1P",
    //     "targetOrders":"AYf5abBGrwjz2n2gGP4YG91hJer22zakrizrRhddTehS",
    //     "baseVault":"5XkWQL9FJL4qEvL8c3zCzzWnMGzerM3jbGuuyRprsEgG",
    //     "quoteVault":"jfrmNrBtxnX1FH36ATeiaXnpA4ppQcKtv7EfrgMsgLJ",
    //     "withdrawQueue":"11111111111111111111111111111111",
    //     "lpVault":"11111111111111111111111111111111",
    //     "marketVersion":3,
    //     "marketProgramId":"9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin",
    //     "marketId":"77quYg4MGneUdjgXCunt9GgM1usmrxKY31twEy3WHwcS",
    //     "marketAuthority":"FGBvMAu88q9d1Csz7ZECB5a2gbWwp6qicNxN2Mo7QhWG",
    //     "marketBaseVault":"H61Y7xVnbWVXrQQx3EojTEqf3ogKVY5GfGjEn5ewyX7B",
    //     "marketQuoteVault":"9FLih4qwFMjdqRAGmHeCxa64CgjP1GtcgKJgHHgz44ar",
    //     "marketBids":"37m9QdvxmKRdjm3KKV2AjTiGcXMfWHQpVFnmhtb289yo",
    //     "marketAsks":"AQKXXC29ybqL8DLeAVNt3ebpwMv8Sb4csberrP6Hz6o5",
    //     "marketEventQueue":"9MgPMkdEHFX7DZaitSh6Crya3kCCr1As6JC75bm3mjuC",
    //     "modelDataAccount":"CDSr3ssLcRB6XYPJwAfFt18MZvEZp4LjHcvzBVZ45duo"
    //     }
  
    let instr = stable_swap(
        &stableProgramID,
        &Pubkey::from_str("2EXiumdi14E9b8Fy62QcA5Uh6WdHS2b38wtSxp72Mibj")?,
        &Pubkey::from_str("3uaZBfHPfmpAHW7dsimC1SnyR61X4bJqQZKWmRSCXJxv")?,
        &Pubkey::from_str("4zbGjjRx8bmZjynJg2KnkJ54VAk1crcrYsGMy79EXK1P")?,
        &Pubkey::from_str("5XkWQL9FJL4qEvL8c3zCzzWnMGzerM3jbGuuyRprsEgG")?,
        &Pubkey::from_str("jfrmNrBtxnX1FH36ATeiaXnpA4ppQcKtv7EfrgMsgLJ")?,
        &Pubkey::from_str("CDSr3ssLcRB6XYPJwAfFt18MZvEZp4LjHcvzBVZ45duo")?,

        &Pubkey::from_str("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin")?,
        &Pubkey::from_str("77quYg4MGneUdjgXCunt9GgM1usmrxKY31twEy3WHwcS")?,
        &Pubkey::from_str("37m9QdvxmKRdjm3KKV2AjTiGcXMfWHQpVFnmhtb289yo")?,
        &Pubkey::from_str("AQKXXC29ybqL8DLeAVNt3ebpwMv8Sb4csberrP6Hz6o5")?,
        &Pubkey::from_str("9MgPMkdEHFX7DZaitSh6Crya3kCCr1As6JC75bm3mjuC")?,

        &Pubkey::from_str("H61Y7xVnbWVXrQQx3EojTEqf3ogKVY5GfGjEn5ewyX7B")?,
        &Pubkey::from_str("9FLih4qwFMjdqRAGmHeCxa64CgjP1GtcgKJgHHgz44ar")?,
        &Pubkey::from_str("FGBvMAu88q9d1Csz7ZECB5a2gbWwp6qicNxN2Mo7QhWG")?,

        &user_usdt_account,
        &user_usdc_account,
        &payer.pubkey(),

        1000000,
        970000
    )?;
    

    let instructions = vec![instr];
    let signers = vec![&payer];
    let recent_hash = rpc.get_latest_blockhash()?;
    let txn = Transaction::new_signed_with_payer(
        &instructions,
        Some(&payer.pubkey()),
        &signers,
        recent_hash,
    );

    let signature = rpc.send_and_confirm_transaction_with_spinner_and_config(
        &txn,
        CommitmentConfig::confirmed(),
        RpcSendTransactionConfig {
            skip_preflight: true,
            ..RpcSendTransactionConfig::default()
        },
    )?;

    println!("stable swap send txn: {}.", signature);

    Ok(())
}
