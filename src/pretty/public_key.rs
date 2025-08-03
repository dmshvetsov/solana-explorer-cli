use borsh::BorshDeserialize;
use serde;
use solana_sdk::pubkey::Pubkey;
use std::fmt;

#[derive(BorshDeserialize, Default)]
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
        // see Solana Program list here https://github.com/solana-foundation/explorer/blob/master/app/utils/programs.ts
        match self.value.to_string().as_str() {
            "11111111111111111111111111111111" => {
                write!(f, "[Solana System Program] {}", self.value)
            }
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5mW" => {
                write!(f, "[Solana Token Program] {}", self.value)
            }
            "KeccakSecp256k11111111111111111111111111111" => {
                write!(f, "[Solana Secp256k1 Program] {}", self.value)
            }
            "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL" => {
                write!(f, "[Solana Associated Token Account Program] {}", self.value)
            }
            "CMZYPASGWeTz7RNGHaRJfCq2XQ5pYK6nDvVQxzkH51zb" => {
                write!(f, "[Magic Eden Launchpad Candy Machine] {}", self.value)
            }
            "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d" => {
                write!(f, "[Metaplex Core Program] {}", self.value)
            }
            _ => fmt::Debug::fmt(&self.value, f),
        }
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
