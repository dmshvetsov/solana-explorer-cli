use crate::{
    balance::{Balance, SplBalance},
    magiceden::{self, cm},
    metaplex::das as mpl_das,
    output::{
        output_json, output_raw_struct, print_error, print_struct, print_warning, OutputFormat,
    },
    rpc,
    token::{TokenAccount, TokenMint},
};
use serde_json::json;
use solana_account_decoder_client_types::UiAccountEncoding;
use solana_client::{
    client_error::ClientError as RpcClientError,
    rpc_config::{self},
    rpc_request::{self, RpcError},
    rpc_response::{self},
};
use solana_sdk::{
    account::{Account as SolanaAccount, ReadableAccount},
    commitment_config,
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
    match account {
        // Token Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
        SolanaAccount {
            owner: spl_token::ID,
            ..
        } => {
            match &account.data[0..4] {
                &[1, 0, 0, 0] | &[0, 0, 0, 0] => {
                    // mint account 
                    // 1000 NFT, 0000 FT
                    let unpacked_data = spl_token::state::Mint::unpack(&account.data).unwrap();
                    let metadata = get_token_metadata(&acc_pubkey);
                    let token_mint = TokenMint::new(account, unpacked_data, metadata);
                    match output_format {
                        OutputFormat::AsStruct => output_raw_struct(token_mint),
                        OutputFormat::AsJson => output_json(token_mint),
                    }
                }
                _ => {
                    // token account
                    let unpacked_data = spl_token::state::Account::unpack(&account.data).unwrap();
                    let token_account = TokenAccount::new(account, unpacked_data);
                    match output_format {
                        OutputFormat::AsStruct => output_raw_struct(token_account),
                        OutputFormat::AsJson => output_json(token_account),
                    }
                }
            }
        }
        // Metaplex Core
        SolanaAccount {
            owner: mpl_core::ID,
            ..
        } => {
            // check first byte that represents mpl_core Key enum to determint type of account
            match account.data[0] {
                5 => {
                    match mpl_core::accounts::BaseCollectionV1::from_bytes(&account.data) {
                        Ok(unpacked_data) => print_struct(unpacked_data),
                        Err(err) => {
                            print_error(err);
                            exit(1);
                        }
                    };
                }
                1 => {
                    match mpl_core::accounts::BaseAssetV1::from_bytes(&account.data) {
                        Ok(unpacked_data) => print_struct(unpacked_data),
                        Err(err) => {
                            print_error(err);
                            exit(1);
                        }
                    };
                }
                _ => todo!(),
            }
            let das_asset = get_das_asset(&acc_pubkey);
            // TODO: add formats
            if das_asset.is_ok() {
                print_struct(das_asset);
            }
        }
        // Magic Eden Candy Machine
        SolanaAccount {
            owner: magiceden::cm::CMZ_ID,
            ..
        } => match output_format {
            OutputFormat::AsStruct => {
                print_struct(cm::CandyMachine::unpack(&account.data).unwrap())
            }
            OutputFormat::AsJson => todo!(),
        },
        // System Program 11111111111111111111111111111111, on-curve, non-executable account
        // (a key-pair "wallet" with balance)
        SolanaAccount {
            owner: solana_sdk::system_program::ID,
            executable: false,
            ..
        } => {
            let mut balance = Balance::from(account);

            // TODO: add flag to hide/show empty balances
            let mint_balances: Vec<(Pubkey, u64)> = get_spl_tokens_by_owner(&acc_pubkey)
                .unwrap()
                .into_iter()
                .map(|spl_token_acc| {
                    let spl_token = spl_token::state::Account::unpack(
                        &spl_token_acc.account.data.decode().unwrap(),
                    )
                    .unwrap();
                    (spl_token.mint, spl_token.amount)
                })
                .filter(|(_, amount)| *amount != 0)
                .collect();

            let spl_metadata_acc_addresses: Vec<Pubkey> = mint_balances
                .iter()
                .map(|(mint_addr, _)| {
                    let (metadata_pda, _) =
                        mpl_token_metadata::accounts::Metadata::find_pda(mint_addr);
                    metadata_pda
                })
                .collect();

            let spl_token_balances: Vec<SplBalance> =
                get_multiple_accounts(&spl_metadata_acc_addresses)
                    .unwrap()
                    .into_iter()
                    .filter(|acc| acc.is_some())
                    .enumerate()
                    .map(|(idx, acc)| {
                        let metadata = mpl_token_metadata::accounts::Metadata::safe_deserialize(
                            acc.unwrap().data(),
                        )
                        .unwrap();
                        let (_, amount) = mint_balances[idx];
                        SplBalance { amount, metadata }
                    })
                    .collect();

            // TODO: sort by balance
            balance.set_spl(spl_token_balances);
            match output_format {
                OutputFormat::AsStruct => output_raw_struct(balance),
                OutputFormat::AsJson => output_json(balance),
            }
        }
        _ => {
            todo!("account address {} with data size {} owned by {} program, not supported yet in solana explorer CLI", acc_pubkey, account.data.len(), account.owner.to_string());
        }
    };
}

fn get_account(pubkey: &Pubkey) -> Result<SolanaAccount, RpcClientError> {
    let rpc_con = rpc::init_connection();
    rpc_con.get_account(pubkey)
}

fn get_multiple_accounts(pubkeys: &[Pubkey]) -> Result<Vec<Option<SolanaAccount>>, RpcClientError> {
    let rpc_con = rpc::init_connection();
    rpc_con.get_multiple_accounts(pubkeys)
}

fn get_das_asset(pubkey: &Pubkey) -> Result<mpl_das::Asset, RpcClientError> {
    let rpc_con = rpc::init_connection();
    let res = rpc_con.send::<mpl_das::Asset>(
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

fn get_spl_tokens_by_owner(
    owner: &Pubkey,
) -> Result<Vec<rpc_response::RpcKeyedAccount>, RpcClientError> {
    let rpc_con = rpc::init_connection();
    let filter = rpc_config::RpcTokenAccountsFilter::ProgramId(spl_token::ID.to_string());
    let config = rpc_config::RpcAccountInfoConfig {
        encoding: Some(UiAccountEncoding::Base64Zstd),
        commitment: Some(commitment_config::CommitmentConfig::confirmed()),
        data_slice: None,
        min_context_slot: None,
    };
    let res = rpc_con.send::<rpc_response::Response<Vec<rpc_response::RpcKeyedAccount>>>(
        rpc_request::RpcRequest::GetTokenAccountsByOwner,
        json!([owner.to_string(), filter, config]),
    )?;
    Ok(res.value)
}
