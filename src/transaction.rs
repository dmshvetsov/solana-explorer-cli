use std::{process, str::FromStr};

use crate::{
    output::{print_error, print_struct, print_warning},
    rpc,
};
use solana_client::rpc_config::RpcTransactionConfig;
use solana_sdk::{commitment_config::CommitmentConfig, signature::Signature};
use solana_transaction_status::{EncodedConfirmedTransactionWithStatusMeta, UiTransactionEncoding};

pub fn read_tx(sig_hash: &str) {
    let sig = match Signature::from_str(sig_hash) {
        Ok(sig) => sig,
        Err(_) => {
            print_warning(format!("signature {:?} is not a valid", sig_hash).as_str());
            process::exit(1);
        }
    };

    match get_tx(&sig) {
        Ok(tx) => print_struct(tx),
        Err(err) => {
            print_error(err);
            process::exit(1);
        }
    }
}

fn get_tx(
    sig: &Signature,
) -> Result<EncodedConfirmedTransactionWithStatusMeta, solana_client::client_error::ClientError> {
    let rpc_con = rpc::init_connection();
    let conf = RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::JsonParsed),
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    };
    rpc_con.get_transaction_with_config(sig, conf)
}
