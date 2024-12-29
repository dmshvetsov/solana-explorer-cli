use solana_sdk::{account::ReadableAccount, native_token::lamports_to_sol};
use crate::output::Output;
use serde::Serialize;
use solana_sdk::account::Account;
use mpl_token_metadata::accounts::Metadata;

#[derive(Debug, Serialize)]
pub struct Balance {
    pub sol: f64,
    pub spl: Vec<SplBalance>
}

#[derive(Debug, Serialize)]
pub struct SplBalance {
    // TODO: need to know decimals value and value in f64 for readability
    pub amount: u64,
    pub metadata: Metadata,
}

impl Balance {
    pub fn set_spl(&mut self, spl: Vec<SplBalance>) {
        self.spl = spl;
    }
}


impl From<Account> for Balance {
    fn from(acc: Account) -> Balance {
        Balance {
            sol: lamports_to_sol(acc.lamports()),
            spl: vec![]
        }
    }
}

impl Output for Balance {}
