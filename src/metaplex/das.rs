use serde::Deserialize;
use std::fmt::Debug;

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub content: Content,
    pub authorities: Vec<Authority>,
    pub compression: Compression,
}

#[derive(Deserialize, Debug)]
pub struct Content {
    pub metadata: Metadata,
    pub edition_num: Option<u64>,
    pub files: Vec<AFile>,
    pub links: Links,
}

#[derive(Deserialize, Debug)]
pub struct Authority {
    pub address: String,
    pub scopes: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Links {
    pub image: String,
    pub external_url: String,
}

#[derive(Deserialize, Debug)]
pub struct AFile {
    pub uri: String,
    pub mime: String,
}

#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub attributes: Vec<Attribute>,
    pub description: String,
    pub name: String,
    pub symbol: String,
    pub token_standard: String,
}

#[derive(Deserialize, Debug)]
pub struct Attribute {
    pub value: String,
    pub trait_type: String,
}

#[derive(Deserialize, Debug)]
pub struct Compression {
    pub eligible: bool,
    pub compressed: bool,
    pub data_hash: String,
    pub creator_hash: String,
    pub asset_hash: String,
    pub tree: String,
    pub seq: u128,
    pub leaf_id: u128,
}
