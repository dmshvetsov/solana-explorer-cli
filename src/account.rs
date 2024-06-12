use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    account::Account, commitment_config::CommitmentConfig, program_pack::Pack, pubkey::Pubkey,
};
use spl_token;
use std::{fmt::Debug, str::FromStr};

fn get_account(address: &str) -> Account {
    let rpc = init_connection();
    rpc.get_account(&Pubkey::from_str(address).unwrap())
        .unwrap()
}

fn read_account_data(account: &Account) {
    if account.data.is_empty() {
        panic!("account has empty data");
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
        _ => panic!("not implemented"),
    }
}

pub fn read_account(address: &str) {
    let account = get_account(address);
    print_struct(&account);
    if !account.data.is_empty() && !account.executable {
        read_account_data(&account);
    }
}

fn init_connection() -> RpcClient {
    RpcClient::new_with_commitment(
        String::from("http://api.mainnet-beta.solana.com"),
        CommitmentConfig::confirmed(),
    )
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
