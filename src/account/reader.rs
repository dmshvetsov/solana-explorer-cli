use crate::{
    balance::{Balance, SplBalance},
    magiceden::{self, cm},
    metaplex::{
        core::{CoreAssetV1, CoreCollectionV1},
        das as mpl_das,
    },
    output::{print_error, print_warning, OutputFormat},
    page::Page,
    rpc,
    token::{TokenAccount, TokenMetadata, TokenMint},
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

use super::Account;

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

    let mut page = Page::new(output_format);

    let account = match get_account(&acc_pubkey) {
        Ok(account) => account,
        Err(err) => {
            // solana account not found
            if err.kind.to_string() == format!("AccountNotFound: pubkey={}", acc_pubkey) {
                // it can be a Metaplex Digital asset (DAS)
                match get_das_asset(&acc_pubkey) {
                    Ok(asset) => {
                        page.add(asset);
                        page.display();
                        exit(0);
                    }
                    _ => {
                        // not a DAS
                        // do nothing and throw "not found" error
                    }
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

    page.add(Account::new(&acc_pubkey, &account));

    // non-program accounts
    match account {
        // Token Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
        SolanaAccount {
            owner: spl_token::ID,
            ..
        } => {
            match &account.data[0..4] {
                &[1, 0, 0, 0] | &[0, 0, 0, 0] => {
                    // mint account: 1000 NFT, 0000 FT
                    let unpacked_data = spl_token::state::Mint::unpack(&account.data).unwrap();
                    page.add(TokenMint::from(unpacked_data));
                    let (metadata_pda, _) =
                        mpl_token_metadata::accounts::Metadata::find_pda(&acc_pubkey);
                    let metadata_account = get_account(&metadata_pda).unwrap();
                    page.add(Account::new(&metadata_pda, &metadata_account));
                    page.add(TokenMetadata::from(
                        mpl_token_metadata::accounts::Metadata::safe_deserialize(
                            metadata_account.data(),
                        )
                        .unwrap(),
                    ));
                }
                _ => {
                    // token account
                    let unpacked_data = spl_token::state::Account::unpack(&account.data).unwrap();
                    page.add(TokenAccount::from(unpacked_data));
                }
            }
        }
        // Metaplex Core
        SolanaAccount {
            owner: mpl_core::ID,
            ..
        } => {
            // check first byte that represents mpl_core Key enum to determine type of account
            match account.data[0] {
                5 => {
                    let unpacked_data =
                        mpl_core::accounts::BaseCollectionV1::from_bytes(&account.data).unwrap();
                    page.add(CoreCollectionV1::from(unpacked_data));
                }
                1 => {
                    let unpacked_data =
                        mpl_core::accounts::BaseAssetV1::from_bytes(&account.data).unwrap();
                    page.add(CoreAssetV1::from(unpacked_data));
                }
                _ => todo!(),
            }
            match get_das_asset(&acc_pubkey) {
                Ok(asset) => {
                    page.add(asset);
                }
                _ => {
                    // not a DAS
                    // do nothing
                }
            }
        }
        SolanaAccount {
            owner: mpl_token_metadata::ID,
            ..
        } => {
            // TODO: remove duplication of spl_token::ID case
            let metadata_acc = get_account(&acc_pubkey).unwrap();
            let unpacked_data =
                mpl_token_metadata::accounts::Metadata::safe_deserialize(&metadata_acc.data)
                    .unwrap();
            page.add(TokenMetadata::from(unpacked_data));
        }
        // Magic Eden Candy Machine
        SolanaAccount {
            owner: magiceden::cm::CMZ_ID,
            ..
        } => {
            let me_candy_machine = cm::CandyMachine::unpack(&account.data).unwrap();
            page.add(me_candy_machine);
        }
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
            page.add(balance);
        }
        _ => {
            todo!("account address {} with data size {} owned by {} program, not supported yet in solana explorer CLI", acc_pubkey, account.data.len(), account.owner.to_string());
        }
    };

    page.display();
}

fn get_account(pubkey: &Pubkey) -> Result<SolanaAccount, RpcClientError> {
    let rpc_con = rpc::init_connection();
    rpc_con.get_account(pubkey)
}

fn get_multiple_accounts(pubkeys: &[Pubkey]) -> Result<Vec<Option<SolanaAccount>>, RpcClientError> {
    let rpc_con = rpc::init_connection();
    // TODO: handle 413 content too large errors
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
