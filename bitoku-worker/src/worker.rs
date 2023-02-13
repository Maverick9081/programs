use super::*;
use bitoku_sdk_agent_native::instruction::{unpack_request, Request};
use serde_json;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Signature;
use solana_transaction_status::{UiMessage, UiTransaction, UiTransactionEncoding};
use std::fs::{self, remove_file, File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use std::str;

// #[tokio::main]
pub fn get_transaction(signature: &str) {
    let url = String::from(RPC_URL);
    let connection = RpcClient::new_with_commitment(url, CommitmentConfig::confirmed());

    let req = get_request_account(signature);

    let data = connection
        .get_account_data(&Pubkey::from_str(req.as_str()).unwrap())
        .unwrap();

    let dat = data.as_slice();

    decode_request(dat.try_into().unwrap());
}

pub fn get_request_account(signature: &str) -> String {
    let url = String::from(RPC_URL);
    let connection = RpcClient::new_with_commitment(url, CommitmentConfig::confirmed());

    let sig = Signature::from_str(signature).unwrap();

    // std::thread::sleep(Duration::from_secs(10));
    let transaction_data = connection
        .get_transaction(&sig, UiTransactionEncoding::JsonParsed)
        .unwrap();

    let iter = serde_json::to_value(transaction_data.transaction.transaction).unwrap();

    let iter2: UiTransaction = serde_json::from_value(iter).unwrap();

    let hu: UiMessage = iter2.message;

    match hu {
        UiMessage::Parsed(parsed_message) => parsed_message.account_keys[1].pubkey.to_string(),

        _ => "null".to_string(),
    }
}

pub fn decode_request(data: [u8; 675]) {
    let req_bytes = &data[33..675];
    let req = unpack_request(req_bytes).unwrap();

    match req {
        Request::CreateBucket { name } => {
            println!("create bucket");
            let file_name = decode_name(name);
            let path = Path::new(&file_name);

            fs::create_dir(path).unwrap();
        }
        Request::CreateFile { name, data } => {
            println!("create file");
            let file_name = decode_name(name);
            let path = Path::new(&file_name);

            let non_zero_data = get_non_zeros(&data);
            let mut file = File::create(path).unwrap();

            file.write_all(&non_zero_data.as_slice()).unwrap();
        }
        Request::WriteFile {
            name,
            file_id,
            data,
        } => {
            println!("write file");
            let file_name = decode_name(name);
            let path = Path::new(&file_name);

            let non_zero_data = get_non_zeros(&data);

            let mut file = OpenOptions::new().append(true).open(&path).unwrap();

            file.write_all(&non_zero_data.as_slice()).unwrap();
        }
        Request::DeleteFile { name, file_id } => {
            println!("delete file");

            let file_name = decode_name(name);
            let path = Path::new(&file_name);

            remove_file(path).unwrap();
        }
        Request::SetPosition { name, file_id } => {
            println!("set position");
        }
        Request::CloseFile { name, file_id } => {
            println!("close file");
        }
        Request::OpenFile { name, file_id } => {
            println!("open file");
        }
        Request::ReadFile { name, file_id } => {
            println!("read file");
        }
    }
}

pub fn decode_name(name: [u8; 128]) -> String {
    let non_zero_bytes = get_non_zeros(&name);

    let output = str::from_utf8(&non_zero_bytes).unwrap();
    output.to_string()
}

pub fn get_non_zeros(data: &[u8]) -> Vec<u8> {
    data.iter().take_while(|&b| *b != 0).copied().collect()
}
