use std::str::FromStr;

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
            return;
        }
    };

    match get_tx(&sig) {
        Ok(tx) => print_struct(tx),
        Err(err) => print_error(err),
    }
}

fn get_tx(
    sig: &Signature,
) -> Result<EncodedConfirmedTransactionWithStatusMeta, solana_client::client_error::ClientError> {
    let rpc_con = rpc::init_connection();
    // This method uses the Finalized commitment level
    let conf = RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::Json),
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    };
    rpc_con.get_transaction_with_config(sig, conf)
}
