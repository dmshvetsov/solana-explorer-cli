use anchor_lang::AnchorDeserialize;
use solana_sdk::pubkey::Pubkey;
use anchor_lang::prelude::borsh;

#[derive(AnchorDeserialize, Debug, Default)]
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
