use crate::output::Output;
use anchor_lang::prelude::Pubkey;
use mpl_token_metadata::accounts::Metadata;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use solana_sdk::account::Account;
use spl_token::state::Mint;

#[derive(Debug, Serialize)]
pub struct Token {
    pub account: Account,
    #[serde(serialize_with = "serialize_mint")]
    pub data: Mint,
    pub metadata: Metadata,
}

fn serialize_mint<S>(mint: &Mint, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut s = serializer.serialize_struct("Mint", 5)?;
    if mint.mint_authority.is_some() {
        s.serialize_field("mint_authority", &format!("{:?}", &mint.mint_authority))?;
    } else {
        s.serialize_field("mint_authority", &None::<Pubkey>)?;
    }
    if mint.freeze_authority.is_some() {
        s.serialize_field("freeze_authority", &format!("{:?}", &mint.freeze_authority))?;
    } else {
        s.serialize_field("freeze_authority", &None::<Pubkey>)?;
    }
    s.serialize_field("supply", &mint.supply)?;
    s.serialize_field("decimals", &mint.decimals)?;
    s.serialize_field("is_initialized", &mint.is_initialized)?;
    s.end()
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
