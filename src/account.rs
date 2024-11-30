use crate::metaplex::das;
use serde_json::json;
use solana_client::{
    client_error::ClientError as RpcClientError,
    rpc_client::RpcClient,
    rpc_request::{self},
};
use solana_sdk::{
    account::Account, commitment_config::CommitmentConfig, program_pack::Pack, pubkey::Pubkey,
};
use std::{env, error::Error, fmt::Debug, process::exit, str::FromStr};

fn read_account_data(account: &Account) {
    if account.data.is_empty() {
        print_warning("data: empty");
        return;
    }

    match account.owner.to_string().as_str() {
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" => {
            print_struct(spl_token::state::Mint::unpack(&account.data).unwrap());
            // if account.data.starts_with(&[1, 0, 0, 0]) {
            //     // SPL Mint
            //     print_struct(spl_token::state::Mint::unpack(&account.data).unwrap());
            // } else {
            //     // SPL Token account
            //     print_struct(spl_token::state::Account::unpack(&account.data).unwrap());
            // }
        }
        _ => todo!(),
    }
}

pub fn read_account(address: &str) {
    let acc_pubkey = match Pubkey::from_str(address) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            print_warning(
                format!("address {:?} is not a valid Solana public key", address).as_str(),
            );
            return;
        }
    };

    match get_account(&acc_pubkey) {
        Ok(account) => {
            print_struct(&account);
            if !account.data.is_empty() && !account.executable {
                read_account_data(&account);
            }
        }
        Err(err) => {
            if err.kind.to_string() == format!("AccountNotFound: pubkey={}", acc_pubkey) {
                // it can be a Metaplex Digital asset
                let asset = get_das_asset(&acc_pubkey);
                if asset.is_ok() {
                    print_struct(&asset);
                    return;
                }
            }
            print_error(err);
            exit(1);
        }
    };
}

fn get_account(pubkey: &Pubkey) -> Result<Account, RpcClientError> {
    let rpc = init_connection();
    rpc.get_account(pubkey)
}

fn get_das_asset(pubkey: &Pubkey) -> Result<das::Asset, RpcClientError> {
    let rpc = init_connection();
    // TODO: handle RpcError RpcResponseError message: "Method not found"
    rpc.send::<das::Asset>(
        rpc_request::RpcRequest::Custom { method: "getAsset" },
        json!([pubkey.to_string()]),
    )
}

fn init_connection() -> RpcClient {
    let rpc_url =
        env::var("SE_RPC_URL").unwrap_or("http://api.mainnet-beta.solana.com".to_string());
    RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed())
}

fn print_struct<T: Debug>(data_struct: T) {
    let type_name = std::any::type_name::<T>().split("::");
    let size = type_name.clone().count();
    let mut type_prefix = type_name.take(size - 1).collect::<Vec<&str>>().join("::");
    if type_prefix.starts_with('&') {
        type_prefix = type_prefix.strip_prefix('&').unwrap().to_string();
    }
    println!("{}::{:#?}", type_prefix, data_struct);
}

fn print_warning(msg: &str) {
    println!("{}", msg);
}

fn print_error<T: Error>(err: T) {
    println!("{}", err);
}
