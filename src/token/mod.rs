use crate::output::Output;
use mpl_token_metadata::{
    accounts::Metadata,
    types::{Collection, CollectionDetails, Creator, Key, ProgrammableConfig, TokenStandard, Uses},
};
use serde::{Serialize, Serializer};
use solana_sdk::pubkey::Pubkey;
use spl_token::state::{Account as SolanaSplTokenAccount, AccountState, Mint};

///
/// Wallet/Owner Account -> Token Account -> Mint Account -> Metadata (PDA) Account
///
/// Wallet/Owner Account - owner, usually a someones wallet, System program owns and operates owner accounts
/// TokenAccount, see TokenAccount struct docs
/// TokenMint, see TokenMint struct docs

/// Mint Account - stores information about the token itself, its suplly, authorities etc
#[derive(Debug, Serialize)]
pub struct TokenMint {
    /// Optional authority used to mint new tokens. The mint authority may only
    /// be provided during mint creation. If no mint authority is present
    /// then the mint has a fixed supply and no further tokens may be
    /// minted.
    pub mint_authority: Option<Pubkey>,
    /// Total supply of tokens.
    pub supply: u64,
    /// Number of base 10 digits to the right of the decimal place.
    pub decimals: u8,
    /// Is `true` if this structure has been initialized
    pub is_initialized: bool,
    /// Optional authority to freeze token accounts.
    pub freeze_authority: Option<Pubkey>,
}

impl From<Mint> for TokenMint {
    fn from(spl_token_mint: Mint) -> Self {
        TokenMint {
            mint_authority: spl_token_mint.mint_authority.into(),
            supply: spl_token_mint.supply,
            decimals: spl_token_mint.decimals,
            is_initialized: spl_token_mint.is_initialized,
            freeze_authority: spl_token_mint.freeze_authority.into(),
        }
    }
}

// fn serialize_mint<S>(mint: &Mint, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     let mut s = serializer.serialize_struct("Mint", 5)?;
//     if mint.mint_authority.is_some() {
//         s.serialize_field("mint_authority", &format!("{:?}", &mint.mint_authority))?;
//     } else {
//         s.serialize_field("mint_authority", &None::<Pubkey>)?;
//     }
//     if mint.freeze_authority.is_some() {
//         s.serialize_field("freeze_authority", &format!("{:?}", &mint.freeze_authority))?;
//     } else {
//         s.serialize_field("freeze_authority", &None::<Pubkey>)?;
//     }
//     s.serialize_field("supply", &mint.supply)?;
//     s.serialize_field("decimals", &mint.decimals)?;
//     s.serialize_field("is_initialized", &mint.is_initialized)?;
//     s.end()
// }

// impl TokenMint {
//     pub fn new(
//         account: SolanaAccount,
//         data: spl_token::state::Mint,
//         metadata: mpl_token_metadata::accounts::Metadata,
//     ) -> Self {
//         TokenMint {
//             account,
//             data,
//             metadata,
//         }
//     }
// }

impl Output for TokenMint {
    fn struct_name(&self) -> String {
        String::from("TokenMint")
    }

    fn to_raw_struct(self: &Self) -> String {
        format!("{:#?}", self)
    }

    fn to_json(self: &Self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

#[derive(Debug, Serialize)]
pub struct TokenMetadata {
    pub key: Key,
    // #[cfg_attr(
    //     feature = "serde",
    //     serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    // )]
    pub update_authority: Pubkey,
    // #[cfg_attr(
    //     feature = "serde",
    //     serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    // )]
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub creators: Option<Vec<Creator>>,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
    pub edition_nonce: Option<u8>,
    pub token_standard: Option<TokenStandard>,
    pub collection: Option<Collection>,
    pub uses: Option<Uses>,
    pub collection_details: Option<CollectionDetails>,
    pub programmable_config: Option<ProgrammableConfig>,
}

impl From<Metadata> for TokenMetadata {
    fn from(mpl_metadata: Metadata) -> Self {
        TokenMetadata {
            key: mpl_metadata.key,
            update_authority: mpl_metadata.update_authority,
            mint: mpl_metadata.mint,
            name: mpl_metadata.name,
            symbol: mpl_metadata.symbol,
            uri: mpl_metadata.uri,
            seller_fee_basis_points: mpl_metadata.seller_fee_basis_points,
            creators: mpl_metadata.creators,
            primary_sale_happened: mpl_metadata.primary_sale_happened,
            is_mutable: mpl_metadata.is_mutable,
            edition_nonce: mpl_metadata.edition_nonce,
            token_standard: mpl_metadata.token_standard,
            collection: mpl_metadata.collection,
            uses: mpl_metadata.uses,
            collection_details: mpl_metadata.collection_details,
            programmable_config: mpl_metadata.programmable_config,
        }
    }
}

impl Output for TokenMetadata {
    fn struct_name(&self) -> String {
        String::from("TokenMetadata")
    }

    fn to_raw_struct(self: &Self) -> String {
        format!("{:#?}", self)
    }

    fn to_json(self: &Self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

fn serialize_account_state<S>(orig: &AccountState, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = match orig {
        AccountState::Uninitialized => "Uninitialized",
        AccountState::Initialized => "Initialized",
        AccountState::Frozen => "Frozen",
    };
    serializer.serialize_str(s)
}

/// Token Account - stores number of token owned another account (wallet e.g.), Token progra owns
/// and operates token accounts
#[derive(Debug, Serialize)]
pub struct TokenAccount {
    /// The mint associated with this account
    pub mint: Pubkey,
    /// The owner of this account.
    pub owner: Pubkey,
    /// The amount of tokens this account holds.
    pub amount: u64,
    /// If `delegate` is `Some` then `delegated_amount` represents
    /// the amount authorized by the delegate
    pub delegate: Option<Pubkey>,
    /// The account's state
    #[serde(serialize_with = "serialize_account_state")]
    pub state: AccountState,
    /// If is_native.is_some, this is a native token, and the value logs the
    /// rent-exempt reserve. An Account is required to be rent-exempt, so
    /// the value is used by the Processor to ensure that wrapped SOL
    /// accounts do not drop below this threshold.
    pub is_native: Option<u64>,
    /// The amount delegated
    pub delegated_amount: u64,
    /// Optional authority to close the account.
    pub close_authority: Option<Pubkey>,
}

impl From<SolanaSplTokenAccount> for TokenAccount {
    fn from(spl_token_account: SolanaSplTokenAccount) -> Self {
        TokenAccount {
            mint: spl_token_account.mint,
            owner: spl_token_account.owner,
            amount: spl_token_account.amount,
            delegate: spl_token_account.delegate.into(),
            state: spl_token_account.state,
            is_native: spl_token_account.is_native.into(),
            delegated_amount: spl_token_account.delegated_amount,
            close_authority: spl_token_account.close_authority.into(),
        }
    }
}

// fn serialize_spl_account<S>(mint: &SolanaSplTokenAccount, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     let mut s = serializer.serialize_struct("Account", 7)?;
//     s.serialize_field("mint", &format!("{:?}", &mint.mint))?;
//     s.serialize_field("owner", &format!("{:?}", &mint.owner))?;
//     if mint.delegate.is_some() {
//         s.serialize_field("delegate", &format!("{:?}", &mint.delegate))?;
//     } else {
//         s.serialize_field("delegate", &None::<Pubkey>)?;
//     }
//     s.serialize_field("state", &format!("{:?}", &mint.state))?;
//     if mint.delegate.is_some() {
//         s.serialize_field("is_native", &format!("{:?}", &mint.is_native))?;
//     } else {
//         s.serialize_field("is_native", &None::<u64>)?;
//     }
//     s.serialize_field("delegated_amount", &mint.delegated_amount)?;
//     if mint.delegate.is_some() {
//         s.serialize_field("close_authority", &format!("{:?}", &mint.close_authority))?;
//     } else {
//         s.serialize_field("close_authority", &None::<Pubkey>)?;
//     }
//     s.end()
// }

// impl TokenAccount {
//     pub fn new(account: SolanaAccount, data: SolanaSplTokenAccount) -> Self {
//         TokenAccount { account, data }
//     }
// }

impl Output for TokenAccount {
    fn struct_name(&self) -> String {
        String::from("TokenAccount")
    }

    fn to_raw_struct(self: &Self) -> String {
        format!("{:#?}", self)
    }

    fn to_json(self: &Self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
