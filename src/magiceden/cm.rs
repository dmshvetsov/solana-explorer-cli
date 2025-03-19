use std::io;

use borsh::BorshDeserialize;
use serde::Serialize;
use solana_sdk::pubkey::Pubkey;

use crate::output::Output;

pub const CMZ_ID: Pubkey =
    solana_sdk::pubkey::Pubkey::from_str_const("CMZYPASGWeTz7RNGHaRJfCq2XQ5pYK6nDvVQxzkH51zb");

// source https://github.com/me-foundation/cmx/blob/main/programs/cmx/src/state.rs
#[derive(BorshDeserialize, Debug, Default, Serialize)]
#[allow(dead_code)]
pub struct CandyMachine {
    pub authority: Pubkey,
    pub wallet_authority: Pubkey,
    pub config: Pubkey,
    pub items_redeemed_normal: u64,
    pub items_redeemed_raffle: u64,
    pub raffle_tickets_purchased: u64,
    pub uuid: String,
    pub items_available: u64,
    pub raffle_seed: u64,
    pub bump: u8,
    pub notary: Option<Pubkey>,
    pub order_info: Pubkey,
    pub is_lite: bool,
    pub notary_required: Vec<bool>,
    pub mip1_ruleset: Option<Pubkey>,
    pub is_open_edition: Option<bool>,
}

impl CandyMachine {
    pub const MAX_LAUNCH_STAGES: usize = 10;
    pub const DESCIMINATOR_SIZE: usize = 8;
    pub const PADDING: usize = 318;
    pub const SIZE: usize = 32 // authority
        + 32 // wallet
        + 32 // config
        + 8 // items_redeemed_normal
        + 8 // items_redeemed_raffle
        + 1 + 6 // uuid
        + 8 // items_available
        + 8 // raffle_seed
        + 1 // bump
        + 1 + 32 // notary
        + 32 // order_info
        + 1 // is_lite
        + 4 + CandyMachine::MAX_LAUNCH_STAGES
        + 33 // Optional mip1_ruleset
        + 2; // optional is_open_edition

    pub fn unpack(data: &[u8]) -> Result<CandyMachine, std::io::Error> {
        let mut padding: usize = 0;
        while padding < Self::PADDING {
            let cm_data = &data[Self::DESCIMINATOR_SIZE..(Self::SIZE - padding)];
            let res = CandyMachine::try_from_slice(cm_data);
            if res.is_ok() {
                return res;
            }
            padding += 1;
        }
        Result::Err(std::io::Error::new(
            io::ErrorKind::InvalidData,
            "can't unpack Candy Machine data",
        ))
    }
}

impl Output for CandyMachine {
    fn struct_name(self: &Self) -> String {
        String::from("CandyMachine")
    }

    fn to_raw_struct(self: &Self) -> String {
        format!("{:#?}", self)
    }

    fn to_json(self: &Self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
