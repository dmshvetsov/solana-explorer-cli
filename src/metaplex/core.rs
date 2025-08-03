use mpl_core::{
    accounts::{BaseAssetV1, BaseCollectionV1},
    types::{Key, UpdateAuthority},
};
use serde::Serialize;
use solana_sdk::pubkey::Pubkey;

use crate::{output::Output, pretty::{public_key::PrettyPublicKey, string::PrettyString}};

#[derive(Debug, Serialize)]
pub struct CoreCollectionV1 {
    pub key: Key,
    pub update_authority: Pubkey,
    pub name: PrettyString,
    pub uri: PrettyString,
    pub num_minted: u32,
    pub current_size: u32,
}

impl From<BaseCollectionV1> for CoreCollectionV1 {
    fn from(collection: BaseCollectionV1) -> Self {
        CoreCollectionV1 {
            key: collection.key,
            update_authority: collection.update_authority,
            name: collection.name.into(),
            uri: collection.uri.into(),
            num_minted: collection.num_minted,
            current_size: collection.current_size,
        }
    }
}

impl Output for CoreCollectionV1 {
    fn struct_name(&self) -> String {
        String::from("CoreCollectionV1")
    }

    fn to_raw_struct(&self) -> String {
        format!("{:#?}", self)
    }

    fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

#[derive(Debug, Serialize)]
pub struct CoreAssetV1 {
    pub key: Key,
    pub owner: PrettyPublicKey,
    pub update_authority: UpdateAuthority,
    pub name: PrettyString,
    pub uri: PrettyString,
    pub seq: Option<u64>,
}

impl From<BaseAssetV1> for CoreAssetV1 {
    fn from(asset: BaseAssetV1) -> Self {
        CoreAssetV1 {
            key: asset.key,
            owner: asset.owner.into(),
            update_authority: asset.update_authority,
            name: asset.name.into(),
            uri: asset.uri.into(),
            seq: asset.seq,
        }
    }
}

impl Output for CoreAssetV1 {
    fn struct_name(&self) -> String {
        String::from("CoreAssetV1")
    }

    fn to_raw_struct(&self) -> String {
        format!("{:#?}", self)
    }

    fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
