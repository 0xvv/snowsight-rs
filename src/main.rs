extern crate core;

use std::env;

use ethers::signers::{LocalWallet, Signer};
use serde_json::{Map, Value};
use std::string::String;
use tokio_tungstenite::tungstenite::Message;
use url::Url;
mod snowsight;
mod utils;

const MSG: &str = "Sign this message to authenticate your wallet with Snowsight.";
const NONCE: [u8; 24] = [0u8; 24]; // TODO CHANGE NONCE BEFORE USE

#[tokio::main]
async fn main() -> Result<(), ()> {
    let pwd = env::var("BOT_PWD").expect("BOT_PWD env var needed to decrypt pkey");
    let key = <[u8; 32]>::try_from(pwd.as_bytes()).unwrap();
    let args: Vec<String> = env::args().collect();
    // You can pass a private key as an argument and it will be encrypted and stored
    if args.len() > 1 {
        utils::encrypt(&*args[1], "pkey.encr", &key, &NONCE).expect("Could not encrypt input");
    } else {
        receive(key).await?;
    }
    Ok(())
}

async fn receive(key: [u8; 32]) -> Result<(), ()> {
    let pk_vec = utils::decrypt("pkey.encr", &key, &NONCE).expect("Could not decrypt");
    let pkey = String::from_utf8(pk_vec).unwrap();
    let wallet = pkey.parse::<LocalWallet>().unwrap();
    // Uncomment the line below to pay your fee before trying to connect
    //snowsight::pay_fee(&wallet, "0").await.expect("failed to pay fee:");

    let signature = wallet.sign_message(MSG).await.unwrap();

    let mut map = Map::new();
    map.insert("signed_key".into(), Value::String(signature.to_string()));
    map.insert("include_finalized".into(), Value::Bool(true));
    let obj = Value::Object(map);

    let (mut socket, _response) = tokio_tungstenite::tungstenite::connect(
        Url::parse("ws://mempool-stream.snowsight.chainsight.dev:8589").unwrap(),
    )
    .expect("Can't reach snowsight");

    socket
        .write_message(Message::Text(obj.to_string()))
        .unwrap();

    let message = socket.read_message().expect("Error reading message");
    println!("Authentication : {}", message);

    loop {
        let message = socket.read_message().expect("Error reading message");
        println!("tx: {}", message);
    }
}
