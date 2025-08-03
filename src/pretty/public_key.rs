use borsh::BorshDeserialize;
use serde;
use solana_sdk::pubkey::Pubkey;
use std::fmt;

#[derive(BorshDeserialize)]
pub struct PrettyPublicKey {
    value: Pubkey,
}

impl From<Pubkey> for PrettyPublicKey {
    fn from(value: Pubkey) -> Self {
        PrettyPublicKey { value }
    }
}

impl fmt::Debug for PrettyPublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.value, f)
    }
}

impl serde::Serialize for PrettyPublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.value.to_string().as_str())
    }
}

impl Default for PrettyPublicKey {
    fn default() -> Self {
        PrettyPublicKey {
            value: Default::default(),
        }
    }
}
