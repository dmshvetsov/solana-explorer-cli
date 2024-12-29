use serde::Deserialize;
use std::fmt::Debug;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Asset {
    pub content: Content,
    pub authorities: Vec<Authority>,
    pub compression: Compression,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Content {
    pub metadata: Metadata,
    pub edition_num: Option<u64>,
    pub files: Option<Vec<AssetFile>>,
    pub links: Option<Links>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Authority {
    pub address: String,
    pub scopes: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Links {
    pub image: Option<String>,
    pub external_url: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct AssetFile {
    pub uri: String,
    pub mime: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Metadata {
    pub name: String,
    pub symbol: String,
    pub attributes: Option<Vec<Attribute>>,
    pub description: Option<String>,
    pub token_standard: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Attribute {
    pub value: String,
    pub trait_type: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
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

// Example of DAS API response
//
// {
//   "jsonrpc": "2.0",
//   "result": {
//     "interface": "MplCoreAsset",
//     "id": "GVXmn44fkeUCMuZexkEqiwBBdv83yucs1LsQKuNC8Yb8",
//     "content": {
//       "$schema": "https://schema.metaplex.com/nft1.0.json",
//       "json_uri": "https://bafybeiad4kztrslpqaclwlnb2ftskpfeqhr3v23myoavpw3omye6xinsjm.ipfs.w3s.link/2479.json",
//       "files": [
//         {
//           "uri": "https://bafybeihzoyqffzzygfmsh3omn5qq6mxhfonhhrrqmxvmm72ifz2336lbve.ipfs.w3s.link/2479.png",
//           "mime": "image/png"
//         }
//       ],
//       "metadata": {
//         "attributes": [
//           {
//             "value": "Soft Lilac",
//             "trait_type": "Background"
//           },
//           {
//             "value": "None",
//             "trait_type": "Clothes"
//           },
//           {
//             "value": "Bucket",
//             "trait_type": "Head"
//           },
//           {
//             "value": "Cyclops",
//             "trait_type": "Eye"
//           },
//           {
//             "value": "Java",
//             "trait_type": "Skin"
//           },
//           {
//             "value": "None",
//             "trait_type": "Mouth"
//           }
//         ],
//         "description": "3333 Fels, of the AFEL ecosystem where degens, builders, and community unite to create and grow with constant innovation, fueled by the $waa token.",
//         "name": "FEL #2479",
//         "symbol": ""
//       },
//       "links": {
//         "image": "https://bafybeihzoyqffzzygfmsh3omn5qq6mxhfonhhrrqmxvmm72ifz2336lbve.ipfs.w3s.link/2479.png"
//       }
//     },
//     "authorities": [
//       {
//         "address": "Afbz6gcoYsbuio58RMVfpytmopJPosDLruE5ZAU1srAx",
//         "scopes": [
//           "full"
//         ]
//       }
//     ],
//     "compression": {
//       "eligible": false,
//       "compressed": false,
//       "data_hash": "",
//       "creator_hash": "",
//       "asset_hash": "",
//       "tree": "",
//       "seq": 0,
//       "leaf_id": 0
//     },
//     "grouping": [
//       {
//         "group_key": "collection",
//         "group_value": "C3JoGupodf1s7JhCKiLNv2hbPRqYkDjxebVmsJh2EtZC"
//       }
//     ],
//     "royalty": {
//       "royalty_model": "creators",
//       "target": null,
//       "percent": 0,
//       "basis_points": 0,
//       "primary_sale_happened": false,
//       "locked": false
//     },
//     "creators": [],
//     "ownership": {
//       "frozen": false,
//       "delegated": false,
//       "delegate": null,
//       "ownership_model": "single",
//       "owner": "4DRqQb3ihANJRqHNoenq9gTwp58rVn6jHMg5wiJYsQzf"
//     },
//     "supply": null,
//     "mutable": true,
//     "burnt": false,
//     "plugins": {},
//     "mpl_core_info": {
//       "plugins_json_version": 1
//     },
//     "external_plugins": []
//   },
//   "id": 1
// }
