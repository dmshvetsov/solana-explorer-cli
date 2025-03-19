use crate::output::Output;
use mpl_token_metadata::accounts::Metadata;
use serde::Serialize;
use solana_sdk::account::Account as SolanaAccount;
use solana_sdk::{account::ReadableAccount, native_token::lamports_to_sol};

#[derive(Debug, Serialize)]
pub struct Balance {
    pub sol: f64,
    pub spl: Vec<SplBalance>,
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

impl From<SolanaAccount> for Balance {
    fn from(acc: SolanaAccount) -> Balance {
        Balance {
            sol: lamports_to_sol(acc.lamports()),
            spl: vec![],
        }
    }
}

impl Output for Balance {
    fn struct_name(&self) -> String {
        String::from("Balance")
    }

    fn to_raw_struct(self: &Self) -> String {
        format!("{:#?}", self)
    }

    fn to_json(self: &Self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
