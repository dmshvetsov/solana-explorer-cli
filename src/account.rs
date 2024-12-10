use crate::{
    magiceden::cm,
    metaplex::das,
    output::{print_error, print_struct, print_warning},
    rpc,
};
use anchor_lang::idl::IdlAccount;
use anchor_lang::AnchorDeserialize;
use flate2::read::ZlibDecoder;
use serde_json::json;
use solana_client::{
    client_error::ClientError as RpcClientError,
    rpc_request::{self, RpcError},
};
use solana_sdk::{
    account::{Account, ReadableAccount},
    program_pack::Pack,
    pubkey::Pubkey,
};
use std::io::prelude::*;
use std::{process::exit, str::FromStr};

fn read_account_data(account: &Account) {
    if account.data.is_empty() {
        print_warning("data: empty");
        return;
    }

    match account.owner.to_string().as_str() {
        // Token Program
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" => {
            print_struct(spl_token::state::Mint::unpack(&account.data).unwrap());
        }
        // Magic Eden Candy Machine
        "CMZYPASGWeTz7RNGHaRJfCq2XQ5pYK6nDvVQxzkH51zb" => {
            print_struct(cm::CandyMachine::deserialize(&mut &account.data[8..]).unwrap());
        }
        _ => todo!(),
    }
}

fn get_token_metadata(pubkey: &Pubkey) -> mpl_token_metadata::accounts::Metadata {
    let (metadata_pda, _) = mpl_token_metadata::accounts::Metadata::find_pda(pubkey);
    let metadata_account = get_account(&metadata_pda).unwrap();
    mpl_token_metadata::accounts::Metadata::safe_deserialize(metadata_account.data()).unwrap()
}

fn read_program_idl(pubkey: &Pubkey) {
    let idl_addr = IdlAccount::address(pubkey);
    let idl_acc = get_account(&idl_addr).unwrap();
    let discrimintaor_size = 8;
    let idl: IdlAccount =
        AnchorDeserialize::deserialize(&mut &idl_acc.data()[discrimintaor_size..]).unwrap();
    let compressed_len: usize = idl.data_len.try_into().unwrap();
    let mut decoder = ZlibDecoder::new(&idl_acc.data[44..44 + compressed_len]);
    let mut s = Vec::new();
    decoder.read_to_end(&mut s).unwrap();
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::from_slice::<serde_json::Value>(&s[..]).unwrap())
            .unwrap(),
    );
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
            if account.executable() {
                read_program_idl(&acc_pubkey);
            } else {
                read_account_data(&account);
                print_struct(get_token_metadata(&acc_pubkey));
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
    let rpc_con = rpc::init_connection();
    rpc_con.get_account(pubkey)
}

fn get_das_asset(pubkey: &Pubkey) -> Result<das::Asset, RpcClientError> {
    let rpc_con = rpc::init_connection();
    let res = rpc_con.send::<das::Asset>(
        rpc_request::RpcRequest::Custom { method: "getAsset" },
        json!([pubkey.to_string()]),
    );
    match res {
        Err(err) => {
            let err_kind = err.kind();
            match err_kind {
                solana_client::client_error::ClientErrorKind::RpcError(
                    RpcError::RpcResponseError { code: -32601, .. },
                ) => {
                    println!("RPC does not support DAS API");
                    exit(1);
                }
                _ => Err(err),
            }
        }
        _ => res,
    }
}
