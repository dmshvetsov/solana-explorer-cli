use crate::output::Output;

#[derive(Debug)]
pub struct Token {
    account: solana_sdk::account::Account,
    data: spl_token::state::Mint,
    metadata: mpl_token_metadata::accounts::Metadata,
}

impl Token {
    pub fn new(
        account: solana_sdk::account::Account,
        data: spl_token::state::Mint,
        metadata: mpl_token_metadata::accounts::Metadata,
    ) -> Self {
        Token {
            account,
            data,
            metadata,
        }
    }
}

impl Output for Token {}
