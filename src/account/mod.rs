pub mod reader;

use solana_sdk::account::Account as SolanaAccount;
use solana_sdk::pubkey::Pubkey;
use uom::si;
use uom::si::information::byte;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Account {
    pub lamports: u64,
    pub public_key: Pubkey,
    pub owner: Pubkey,
    pub executable: bool,
    pub rent_epoch: u64,
    // pub data_size: si::information::Information<_, f64>,
    pub data_size: si::usize::Information,
}

impl Account {
    pub fn new(acc_pubkey: &Pubkey, account: &SolanaAccount) -> Self {
        Account {
            lamports: account.lamports,
            public_key: *acc_pubkey,
            owner: account.owner,
            executable: account.executable,
            rent_epoch: account.rent_epoch,
            data_size: si::information::Information::new::<byte>(account.data.len()),
        }
    }
}

impl Output for Account {}

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
