use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;

pub const CMZ_ID: Pubkey =
    solana_sdk::pubkey::Pubkey::from_str_const("CMZYPASGWeTz7RNGHaRJfCq2XQ5pYK6nDvVQxzkH51zb");

#[derive(BorshDeserialize, Debug, Default)]
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
    pub const SIZE: usize = 8
        + 32 // authority
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
        // and 318 padding, not used in the deserialization
  
    pub fn unpack(data: &[u8]) -> Result<CandyMachine, std::io::Error> {
        let unknown_size_adjustment: usize = 3;
        CandyMachine::try_from_slice(&data[8..CandyMachine::SIZE + unknown_size_adjustment])
    }
}
