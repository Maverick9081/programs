use anyhow::Result;
use futures_util::StreamExt;
use std::{rc::Rc, str::FromStr};
pub mod helper;
pub mod request;
pub mod worker;
use anchor_client::{Client, Cluster};
use helper::*;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},
};

use std::thread;
use std::time::Duration;
use worker::*;

#[tokio::main]
async fn main() -> Result<()> {
    // let secret_key: [u8; 64] = [
    //     193, 23, 149, 24, 10, 50, 0, 148, 120, 28, 61, 150, 202, 179, 184, 169, 151, 2, 141, 253,
    //     156, 82, 130, 59, 91, 111, 138, 239, 39, 22, 251, 100, 231, 154, 76, 37, 201, 246, 233, 93,
    //     126, 103, 25, 172, 96, 113, 143, 245, 133, 35, 71, 232, 34, 54, 27, 176, 14, 250, 13, 5,
    //     89, 26, 200, 104,
    // ];

    // let wallet = Keypair::from_bytes(&secret_key).unwrap();

    // let payer = Rc::new(wallet) as Rc<dyn Signer>;

    // let client = Client::new(Cluster::Devnet, payer);

    // let program = Pubkey::from_str(PROGRAM).unwrap();

    // let prog = client.program(program);

    let filter = RpcTransactionLogsFilter::Mentions(vec![String::from(PROGRAM)]);

    let transaction_config = RpcTransactionLogsConfig {
        commitment: Some(CommitmentConfig::processed()),
    };

    let pub_sub_client = PubsubClient::new(WEB_SOCKET_URL).await?;

    // get_transaction();

    let (mut logs, logs2) = pub_sub_client
        .logs_subscribe(filter, transaction_config)
        .await?;

    while let Some(log) = logs.next().await {
        ////////////////////////////////////
        let sign = log.value.signature;

        thread::spawn(move || {
            println!("sleeping");
            thread::sleep(Duration::from_secs(30));
            println!("just woke up");
            let big = sign.clone();
            get_transaction(big.as_str());
        });

        println!("Transaction executed in slot {}:", log.context.slot);
        // println!("  Signature: {}:", log.value.signature);
        println!(
            "  Status: {}",
            log.value
                .err
                .map(|err| err.to_string())
                .unwrap_or_else(|| "Success".into())
        );
        println!("  Log Messages:");
        for msg in log.value.logs {
            println!("    {}", msg);
        }
    }
    logs2().await;

    Ok(())
}
