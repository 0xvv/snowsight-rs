# snowsight-rs

Basic Rust example of a client for snowsight Avalanche mempool streaming service.
This uses `tokio-tungstenite` for the websocket, `ethers-rs` and `cast` to interact with the chain and `chacha20poly1305` for private key encryption.

This contains examples for:
- Connecting to the websocket and receiving transactions
- Paying the snowsight fee for a given tier
- Using the transaction propagator 
 
This could be used as a basis for a bot, although private key management should be improved.

### Running the code

You should change the NONCE constant at line 14 in `main.rs` before use.

You can choose to include the line to pay the fee or not (line 35 in `main.rs`).
Paying for the trial tier (0) costs a couple cents as of writing this.

You can now compile the code, during compilation keep reading.

Set the env var `BOT_PWD` with a 32 chars long password to encrypt and decrypt the private key.

Set the env var `RPC_URL` to an Avalanche mainnet RPC.

You can then run : 

```shell
snowsight-rs <private-key>
```
Your private key will be encrypted and stored in the home directory for later use.

Then run the code to see the transactions stream.
```shell
snowsight-rs 
```

If this example saved you some time and you'd like to show appreciation you can toss some coins at `0xbaa1F78cE6e71cE75aC0cac657aCdB644eFe4991`.

The code is under MIT license.
