# How to run
Follow the next few steps

## envirment 
### localnet

####  1. deploy serum
Make sure you have run `solana-test-validator`
```
git clone https://github.com/project-serum/serum-dex.git && cd  serum-dex 

// build
anchor build

// deploy when build done
solana program deploy ./dex/target/deploy/serum_dex.so ----program-id z678mfRyG19BTfAJm5fcRh6hWJfNBbN1Gv9kvPCqvLA
```

#### 2. deploy amm program

> The code has't been open sourced!    
> solana program deploy amm.so --program-id C21qJRTdoAh6Jk9AvpB51Rt2CMi8xkFcGUX6LeG3QTVT


### devnet

#### 1. change config in anchor.toml

```
[provider]                     [provider]
cluster = "localnet" --------> cluster = "devnet"
```

## install TS depentence
```
yarn install
```

## 2. run with anchor
```
anchor test
```