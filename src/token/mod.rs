use crate::output::Output;
use mpl_token_metadata::accounts::Metadata;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use solana_sdk::{account::Account as SolanaAccount, pubkey::Pubkey};
use spl_token::state::{Account as SplAccount, Mint};

///
/// Wallet/Owner Account -> Token Account -> Mint Account -> Metadata (PDA) Account
///
/// Wallet/Owner Account - owner, usually a someones wallet, System program owns and operates owner accounts
/// TokenAccount, see TokenAccount struct docs
/// TokenMint, see TokenMint struct docs

/// Mint Account - stores information about the token itself, its suplly, authorities etc
#[derive(Debug, Serialize)]
pub struct TokenMint {
    pub account: SolanaAccount,
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

impl TokenMint {
    pub fn new(
        account: SolanaAccount,
        data: spl_token::state::Mint,
        metadata: mpl_token_metadata::accounts::Metadata,
    ) -> Self {
        TokenMint {
            account,
            data,
            metadata,
        }
    }
}

impl Output for TokenMint {
    fn struct_name(&self) -> String {
        String::from("TokenMint")
    }

    fn to_raw_struct(self: &Self) -> String {
        format!("{:?}", self)
    }

    fn to_json(self: &Self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

/// Token Account - stores number of token owned another account (wallet e.g.), Token progra owns
/// and operates token accounts
#[derive(Debug, Serialize)]
pub struct TokenAccount {
    pub account: SolanaAccount,
    #[serde(serialize_with = "serialize_spl_account")]
    pub data: SplAccount,
}

fn serialize_spl_account<S>(mint: &SplAccount, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut s = serializer.serialize_struct("Account", 7)?;
    s.serialize_field("mint", &format!("{:?}", &mint.mint))?;
    s.serialize_field("owner", &format!("{:?}", &mint.owner))?;
    if mint.delegate.is_some() {
        s.serialize_field("delegate", &format!("{:?}", &mint.delegate))?;
    } else {
        s.serialize_field("delegate", &None::<Pubkey>)?;
    }
    s.serialize_field("state", &format!("{:?}", &mint.state))?;
    if mint.delegate.is_some() {
        s.serialize_field("is_native", &format!("{:?}", &mint.is_native))?;
    } else {
        s.serialize_field("is_native", &None::<u64>)?;
    }
    s.serialize_field("delegated_amount", &mint.delegated_amount)?;
    if mint.delegate.is_some() {
        s.serialize_field("close_authority", &format!("{:?}", &mint.close_authority))?;
    } else {
        s.serialize_field("close_authority", &None::<Pubkey>)?;
    }
    s.end()
}

impl TokenAccount {
    pub fn new(account: SolanaAccount, data: SplAccount) -> Self {
        TokenAccount { account, data }
    }
}

impl Output for TokenAccount {
    fn struct_name(&self) -> String {
        String::from("TokenAccount")
    }

    fn to_raw_struct(self: &Self) -> String {
        format!("{:?}", self)
    }

    fn to_json(self: &Self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
