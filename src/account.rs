use crate::{
    magiceden::{self, cm},
    metaplex::das,
    output::{
        output_json, output_raw_struct, print_error, print_struct, print_warning, OutputFormat,
    },
    rpc,
    token::Token,
};
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
use std::{process::exit, str::FromStr};


fn get_token_metadata(pubkey: &Pubkey) -> mpl_token_metadata::accounts::Metadata {
    let (metadata_pda, _) = mpl_token_metadata::accounts::Metadata::find_pda(pubkey);
    let metadata_account = get_account(&metadata_pda).unwrap();
    mpl_token_metadata::accounts::Metadata::safe_deserialize(metadata_account.data()).unwrap()
}

// fn read_program_idl(pubkey: &Pubkey) {
//     // TODO: handle inconsistency here we print JSON every time despite command format param/flag
//     let idl_addr = IdlAccount::address(pubkey);
//     let idl_acc = match get_account(&idl_addr) {
//         Ok(acc) => acc,
//         Err(_) => {
//             print_warning("read of non Anchor programs is not supported yet");
//             exit(0);
//         }
//     };
//     let discrimintaor_size = 8;
//     let idl: IdlAccount =
//         AnchorDeserialize::deserialize(&mut &idl_acc.data()[discrimintaor_size..]).unwrap();
//     let compressed_len: usize = idl.data_len.try_into().unwrap();
//     let mut decoder = ZlibDecoder::new(&idl_acc.data[44..44 + compressed_len]);
//     let mut s = Vec::new();
//     decoder.read_to_end(&mut s).unwrap();
//     println!(
//         "{}",
//         serde_json::to_string_pretty(&serde_json::from_slice::<serde_json::Value>(&s[..]).unwrap())
//             .unwrap(),
//     );
// }

/// Main entry point to account command/module
pub fn read_account(address: &str, output_format: OutputFormat) {
    let acc_pubkey = match Pubkey::from_str(address) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            print_warning(
                format!("address {:?} is not a valid Solana public key", address).as_str(),
            );
            return;
        }
    };

    let account = match get_account(&acc_pubkey) {
        Ok(account) => account,
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

    // program accounts
    if account.executable() {
        todo!();
        // read_program_idl(&acc_pubkey);
        // exit(0);
    }

    // non-program accounts
    match account.owner.to_string().as_str() {
        // Token Program
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" => {
            let unpacked_data = spl_token::state::Mint::unpack(&account.data).unwrap();
            let metadata = get_token_metadata(&acc_pubkey);
            let token = Token::new(account, unpacked_data, metadata);
            match output_format {
                OutputFormat::AsStruct => output_raw_struct(token),
                OutputFormat::AsJson => output_json(token),
            }
        }
        // Magic Eden Candy Machine
        "CMZYPASGWeTz7RNGHaRJfCq2XQ5pYK6nDvVQxzkH51zb" => {
            print!("account data: ");
            print_struct(cm::CandyMachine::deserialize(&mut &account.data[8..]).unwrap());
        }
        _ => {
            todo!();
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
