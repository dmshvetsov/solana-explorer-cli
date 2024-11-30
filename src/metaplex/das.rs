use serde::Deserialize;
use std::fmt::Debug;

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub content: Content,
    pub authorities: Vec<Authority>,
    pub compression: Compression,
}

#[derive(Deserialize, Debug)]
struct Content {
    pub metadata: Metadata,
    pub edition_num: Option<u64>,
    pub files: Vec<AFile>,
    pub links: Links,
}

#[derive(Deserialize, Debug)]
struct Authority {
    address: String,
    scopes: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Links {
    image: String,
    external_url: String,
}

#[derive(Deserialize, Debug)]
struct AFile {
    uri: String,
    mime: String,
}

#[derive(Deserialize, Debug)]
struct Metadata {
    attributes: Vec<Attribute>,
    description: String,
    name: String,
    symbol: String,
    token_standard: String,
}

#[derive(Deserialize, Debug)]
struct Attribute {
    value: String,
    trait_type: String,
}

#[derive(Deserialize, Debug)]
struct Compression {
    eligible: bool,
    compressed: bool,
    data_hash: String,
    creator_hash: String,
    asset_hash: String,
    tree: String,
    seq: u128,
    leaf_id: u128,
}
//     "compression": {
//       "eligible": false,
//       "compressed": true,
//       "data_hash": "HMDn6xqYdqhS44ZD31E1a5AmssAstPeMZ4kLMS3EhrGC",
//       "creator_hash": "Ab9mXRaQEA1QuiGUgBj68siPSqZNwQ2F9izNtFkghudE",
//       "asset_hash": "E8xGUBjJyKFqBKLue6Kjm4doxsmjm5KoUSJDr41T2bSK",
//       "tree": "5i1rrMFvFfwCkR15cf66bEXM2LmtfffDfYegQ3qdWgcF",
//       "seq": 25204,
//       "leaf_id": 597
//     },

// getAsset payload
// {
//   "jsonrpc": "2.0",
//   "result": {
//     "interface": "V1_NFT",
//     "id": "9HNN54hfD3GVy4WkUtXjJdxaTo9tjFzYmEXYN9eHnLZp",
//     "content": {
//       "$schema": "https://schema.metaplex.com/nft1.0.json",
//       "json_uri": "https://prod-tensor-creators-s3.s3.us-east-1.amazonaws.com/drop/e6ed698b-ae10-42f9-8d2e-16b8d844439e/712f6437-bb52-49d6-86a2-b2151579f547",
//       "files": [
//         {
//           "uri": "https://prod-tensor-creators-s3.s3.us-east-1.amazonaws.com/drop-metadata/14f27171-430f-42ca-b442-6a75a8daadad/images/624.png",
//           "mime": "image/png"
//         }
//       ],
//       "metadata": {
//         "attributes": [
//           {
//             "value": "Purple",
//             "trait_type": "Background"
//           },
//           {
//             "value": "None",
//             "trait_type": "Back Accessory"
//           },
//           {
//             "value": "Base",
//             "trait_type": "Base"
//           },
//           {
//             "value": "None",
//             "trait_type": "Earring"
//           },
//           {
//             "value": "Parka",
//             "trait_type": "Body"
//           },
//           {
//             "value": "White",
//             "trait_type": "Eyes"
//           },
//           {
//             "value": "Smoking",
//             "trait_type": "Front Accessory"
//           },
//           {
//             "value": "Forwards Cap",
//             "trait_type": "Headgear"
//           },
//           {
//             "value": "None",
//             "trait_type": "Faceplate"
//           },
//           {
//             "value": "Raider",
//             "trait_type": "Faction"
//           }
//         ],
//         "description": "A long time ago in a Solaxy far, far away...",
//         "name": "Tensorian #598",
//         "symbol": "TNSRNS",
//         "token_standard": "NonFungible"
//       },
//       "links": {
//         "image": "https://prod-tensor-creators-s3.s3.us-east-1.amazonaws.com/drop-metadata/14f27171-430f-42ca-b442-6a75a8daadad/images/624.png",
//         "external_url": "https://www.tensor.trade/"
//       }
//     },
//     "authorities": [
//       {
//         "address": "qzKzA7R24pWnuFcv6sHCTv2NKn1raG2fbL7bpgipJP5",
//         "scopes": [
//           "full"
//         ]
//       }
//     ],
//     "compression": {
//       "eligible": false,
//       "compressed": true,
//       "data_hash": "HMDn6xqYdqhS44ZD31E1a5AmssAstPeMZ4kLMS3EhrGC",
//       "creator_hash": "Ab9mXRaQEA1QuiGUgBj68siPSqZNwQ2F9izNtFkghudE",
//       "asset_hash": "E8xGUBjJyKFqBKLue6Kjm4doxsmjm5KoUSJDr41T2bSK",
//       "tree": "5i1rrMFvFfwCkR15cf66bEXM2LmtfffDfYegQ3qdWgcF",
//       "seq": 25204,
//       "leaf_id": 597
//     },
//     "grouping": [
//       {
//         "group_key": "collection",
//         "group_value": "5PA96eCFHJSFPY9SWFeRJUHrpoNF5XZL6RrE1JADXhxf"
//       }
//     ],
//     "royalty": {
//       "royalty_model": "creators",
//       "target": null,
//       "percent": 0.030000000000000002,
//       "basis_points": 300,
//       "primary_sale_happened": true,
//       "locked": false
//     },
//     "creators": [
//       {
//         "address": "6pZYD8qi7g8XT8pPg8L6NJs2znZkQ4CoPjTz6xqwnBWg",
//         "share": 100,
//         "verified": false
//       }
//     ],
//     "ownership": {
//       "frozen": false,
//       "delegated": false,
//       "delegate": null,
//       "ownership_model": "single",
//       "owner": "CYTUJfsguudUCB1nd2mSQHiUbXsi4jfc5v2NeZEwuvMG"
//     },
//     "supply": {
//       "print_max_supply": 0,
//       "print_current_supply": 0,
//       "edition_nonce": null
//     },
//     "mutable": true,
//     "burnt": false
//   },
//   "id": 0
// }

///// Metaplex Digital Asset Standard item
//#[derive(Deserialize, Debug)]
//pub struct Asset {
//    // TODO: add rest of Asset fields https://github.com/metaplex-foundation/digital-asset-standard-api/blob/main/clients/js/src/types.ts#L154
//    id: String,
//    content: AssetContent
//}
//
//#[derive(Deserialize, Debug)]
//pub struct AssetContent {
//    json_uri: String,
//    files: Option<Vec<File>>,
//    metadata: Metadata,
//    links: Option<Vec<HashMap<String, serde_json::Value>>>,
//}
//
//#[derive(Deserialize, Debug)]
//pub struct File {
//    uri: Option<String>,
//    mime: Option<String>,
//    #[serde(flatten)]
//    additional_properties: HashMap<String, serde_json::Value>,
//}
//
//#[derive(Deserialize, Debug)]
//pub struct Metadata {
//    name: String,
//    symbol: String,
//    description: Option<String>,
//    token_standard: Option<String>,
//    attributes: Option<Vec<Attribute>>,
//    #[serde(flatten)]
//    additional_properties: HashMap<String, serde_json::Value>,
//}
//
//#[derive(Deserialize, Debug)]
//pub struct Attribute {
//    trait_type: Option<String>,
//    value: Option<String>,
//    #[serde(flatten)]
//    additional_properties: HashMap<String, serde_json::Value>,
//}
//
