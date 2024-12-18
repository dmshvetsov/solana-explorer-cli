use solana_sdk::{account::ReadableAccount, native_token::lamports_to_sol};
use crate::output::Output;
use serde::Serialize;
use solana_sdk::account::Account;

#[derive(Debug, Serialize)]
pub struct Balance {
    pub sol: f64,
}

impl From<Account> for Balance {
    fn from(acc: Account) -> Balance {
        Balance {
            sol: lamports_to_sol(acc.lamports())
        }
    }
}

impl Output for Balance {}
