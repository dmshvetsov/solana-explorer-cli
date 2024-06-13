use serde::Deserialize;
use serde_json::json;
use solana_client::{
    client_error::ClientError as RpcClientError,
    rpc_client::RpcClient,
    rpc_request::{self},
};
use solana_sdk::{
    account::Account, commitment_config::CommitmentConfig, program_pack::Pack, pubkey::Pubkey,
};
use spl_token;
use std::{env, error::Error, fmt::Debug, str::FromStr};

fn read_account_data(account: &Account) {
    if account.data.is_empty() {
        print_warning("account has empty data");
        return;
    }

    match account.owner.to_string().as_str() {
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" => {
            if account.data.starts_with(&[1, 0, 0, 0]) {
                // SPL Mint
                print_struct(spl_token::state::Mint::unpack(&account.data).unwrap());
            } else {
                // SPL Token account
                print_struct(spl_token::state::Account::unpack(&account.data).unwrap());
            }
        }
        _ => todo!(),
    }
}

pub fn read_account(address: &str) {
    let acc_pubkey = match Pubkey::from_str(address) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            print_warning(
                format!("adress {:?} is not a valid Solana public key", address).as_str(),
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
                let asset = get_das_asset(&acc_pubkey);
                if asset.is_ok() {
                    print_struct(&asset);
                }
            }
            print_error(err);
        }
    };
}

/// Metaplex Digital Asset Standard item
#[derive(Deserialize, Debug)]
struct DasAsset {
    // TODO: add rest of Asset fields https://github.com/metaplex-foundation/digital-asset-standard-api/blob/main/clients/js/src/types.ts#L154
    id: String,
}

fn get_account(pubkey: &Pubkey) -> Result<Account, RpcClientError> {
    let rpc = init_connection();
    rpc.get_account(pubkey)
}

fn get_das_asset(pubkey: &Pubkey) -> Result<DasAsset, RpcClientError> {
    let rpc = init_connection();
    // TODO: handle RpcError RpcResponseError message: "Method not found"
    rpc.send::<DasAsset>(
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
