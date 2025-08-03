pub mod reader;

use serde::Serialize;
use solana_sdk::account::Account as SolanaAccount;
use solana_sdk::pubkey::Pubkey;
use uom::si;
use uom::si::information::byte;

use crate::output::Output;
use crate::pretty::public_key::PrettyPublicKey;

#[derive(Debug, Serialize)]
pub struct Account {
    pub lamports: u64,
    pub public_key: PrettyPublicKey,
    pub owner: PrettyPublicKey,
    pub executable: bool,
    pub rent_epoch: u64,
    // pub data_size: si::information::Information<_, f64>,
    pub data_size: si::usize::Information,
}

impl Account {
    pub fn new(acc_pubkey: &Pubkey, account: &SolanaAccount) -> Self {
        Account {
            lamports: account.lamports,
            public_key: PrettyPublicKey::from(*acc_pubkey),
            owner: account.owner.into(),
            executable: account.executable,
            rent_epoch: account.rent_epoch,
            data_size: si::information::Information::new::<byte>(account.data.len()),
        }
    }
}

impl Output for Account {
    fn struct_name(&self) -> String {
        String::from("Account")
    }

    fn to_raw_struct(&self) -> String {
        format!("{:#?}", self)
    }

    fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

// impl From<SolanaAccount> for Account {
//     fn from(account: SolanaAccount) -> Self {
//         Account {
//             lamports: account.lamports,
//             public_key: Pubkey::default(),
//             owner: account.owner,
//             executable: account.executable,
//             rent_epoch: account.rent_epoch,
//             data_size: si::information::Information::new::<byte>(account.data.len()),
//         }
//     }
// }
