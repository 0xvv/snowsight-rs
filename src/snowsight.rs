use std::{convert::TryFrom, env, str::FromStr};

use bus::BusReader;
use cast::{Cast, TxBuilder};
use ethers::abi::AbiDecode;
use ethers::core::types::{Address, Chain, U256};
use ethers::prelude::{BlockNumber, ProviderError};
use ethers::providers::{Http, Middleware, Provider};
use ethers::signers::Signer;
use ethers::types::transaction::eip2718::TypedTransaction;
use serde_json::{Map, Value};
use tracing::{debug, info};

use crate::{LocalWallet, MSG};

/// Pay the snowsight fee for the minimal duration of the desired tier (0,1,2)
pub async fn pay_fee(wallet: &LocalWallet, tier: &str) -> Result<(), ProviderError> {
    let rpc_url = env::var("RPC_URL").expect("RPC_URL env var needed");
    let provider = Provider::<Http>::try_from(rpc_url).unwrap();
    let cast = Cast::new(provider.clone());
    let nonce = provider.get_transaction_count(wallet.address(), Some(BlockNumber::Latest.into()));
    let snowsight_address = "0x727Dc3C412cCb942c6b5f220190ebAB3eFE0Eb93";
    let to = Address::from_str(snowsight_address).unwrap();
    let args = vec![tier.to_owned()];

    //Get Price
    let mut builder = TxBuilder::new(&provider, wallet.address(), to, Chain::Avalanche, false)
        .await
        .expect("could not build tx");
    builder
        .set_args("calculateMinPayment(uint256)()", args.clone())
        .await
        .expect("invalid args");
    let mut resp = cast
        .call(builder.build(), None)
        .await
        .expect("could not fetch price");
    resp.pop();
    let value = U256::decode_hex(resp).expect("could not decode price");

    //Pay
    let sig = "pay(uint256)()";
    let gas = U256::from(91000); // todo use cast estimate
    let mut builder = TxBuilder::new(&provider, wallet.address(), to, Chain::Avalanche, false)
        .await
        .expect("could not build tx");
    builder
        .set_args(sig, args)
        .await
        .expect("invalid args")
        .set_gas(gas)
        .set_value(value)
        .set_nonce(nonce.await?);

    let (mut tx, _function) = builder.build();
    tx.set_chain_id(43114);
    info!("{:?}", provider.fill_transaction(&mut tx, None).await);
    let signature = wallet.sign_transaction(&tx).await.expect("failed to sign");

    debug!("{:?}", tx);
    let signed_tx = tx.rlp_signed(&signature);
    let raw_tx = format!("{:x}", signed_tx);
    let data = cast.publish(raw_tx).await.expect("failed to publish tx");
    info!("Pay Tx hash: {:?}", *data);
    let after = data.await?.unwrap();
    info!("Pay Tx receipt: {:?}", after);
    Ok(())
}

pub async fn run_propagator(wallet: &LocalWallet, mut rx: BusReader<TypedTransaction>) -> Result<(), reqwest::Error> {
    let signed_key = wallet.sign_message(MSG).await.unwrap();
    for tx in rx.iter() {
        let signature = wallet.sign_transaction(&tx).await.expect("failed to sign");
        let signed_tx = tx.rlp_signed(&signature);
        let raw_tx = format!("{:x}", signed_tx);
        let mut args = Map::new();
        args.insert("signed_key".into(), Value::String(signed_key.to_string()));
        args.insert("include_finalized".into(), Value::String(raw_tx));
        let msg = Value::Object(args);
        info!("{}", serde_json::to_string(&msg).unwrap());
        let client = reqwest::Client::new();
        let res = client
            .post("http://tx-propagator.snowsight.chainsight.dev:8081")
            .body(serde_json::to_string(&msg).unwrap())
            .send()
            .await?;
    }
    Ok(())
}
