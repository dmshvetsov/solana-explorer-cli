use crate::{output::Output, pretty::string::PrettyString};
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

impl Output for TokenMint {
    fn struct_name(&self) -> String {
        String::from("TokenMint")
    }

    fn to_raw_struct(&self) -> String {
        format!("{:#?}", self)
    }

    fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

/// On-chain token metadata
#[derive(Debug, Serialize)]
pub struct TokenMetadata {
    pub key: Key,
    pub update_authority: Pubkey,
    pub mint: Pubkey,
    pub name: PrettyString,
    pub symbol: PrettyString,
    pub uri: PrettyString,
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
            name: mpl_metadata.name.into(),
            symbol: mpl_metadata.symbol.into(),
            uri: mpl_metadata.uri.into(),
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

    fn to_raw_struct(&self) -> String {
        format!("{:#?}", self)
    }

    fn to_json(&self) -> String {
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

impl Output for TokenAccount {
    fn struct_name(&self) -> String {
        String::from("TokenAccount")
    }

    fn to_raw_struct(&self) -> String {
        format!("{:#?}", self)
    }

    fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
