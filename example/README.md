# How to run
Follow the next few steps


## devnet

### 1. install  dependencies
```
cd example

yarn install
```

## 2. start local validator
```
cd local_validator

./start-test-validator.sh
```

## 3. build program and list new program id
```
anchor build
```
3.1 get the pubkey `<new_pub_key>` listed by the command bellow
```
solana address -k target/deploy/amm_proxy-keypair.json
```
3.2 fill in the `<new_pub_key>` to example/programs/src/lib.rs in `declare_id!("<new_pub_key>");`

3.3 fill in the `<new_pub_key>` under example/Anchor.toml 
```
[program.localnet]
amm_proxy = <new_pub_key>
```



## 3. run with anchor
```
cd example

anchor test --skip-local-validator
```