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
            // Native programs
            "11111111111111111111111111111111" => write!(f, "[System Program] {}", self.value),
            "AddressLookupTab1e1111111111111111111111111" => {
                write!(f, "[Address Lookup Table Program] {}", self.value)
            }
            "ComputeBudget111111111111111111111111111111" => {
                write!(f, "[Compute Budget Program] {}", self.value)
            }
            "Config1111111111111111111111111111111111111" => {
                write!(f, "[Config Program] {}", self.value)
            }
            "Stake11111111111111111111111111111111111111" => {
                write!(f, "[Stake Program] {}", self.value)
            }
            "Vote111111111111111111111111111111111111111" => {
                write!(f, "[Vote Program] {}", self.value)
            }
            "Ed25519SigVerify111111111111111111111111111" => {
                write!(f, "[Ed25519 SigVerify Precompile] {}", self.value)
            }
            "KeccakSecp256k11111111111111111111111111111" => {
                write!(f, "[Secp256k1 SigVerify Precompile] {}", self.value)
            }

            // SPL programs
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5mW" => {
                write!(f, "[Token Program] {}", self.value)
            }
            "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb" => {
                write!(f, "[Token-2022 Program] {}", self.value)
            }
            "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL" => {
                write!(f, "[Associated Token Program] {}", self.value)
            }
            "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s" => {
                write!(f, "[Token Metadata Program] {}", self.value)
            }
            "vau1zxA2LbssAUEF7Gpw91zMM1LvXrvpzJtmZ58rPsn" => {
                write!(f, "[Token Vault Program] {}", self.value)
            }
            "SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy" => {
                write!(f, "[Stake Pool Program] {}", self.value)
            }
            "Feat1YXHhH6t1juaWF74WLcfv4XoNocjXA6sPWHNgAse" => {
                write!(f, "[Feature Proposal Program] {}", self.value)
            }
            "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr" => {
                write!(f, "[Memo Program] {}", self.value)
            }
            "Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo" => {
                write!(f, "[Memo Program v1] {}", self.value)
            }
            "namesLPneVptA9f5Uo4BtvHu1Twqp9mgTqtnimhDsw" => {
                write!(f, "[Name Service Program] {}", self.value)
            }
            "LendZqTs7gn5CTSJU1jWKhKuVpjJGom45nnwPb2AMTi" => {
                write!(f, "[Lending Program] {}", self.value)
            }
            "SwaPpA9LAaLfeLi3a68M4DjnLqgtticKg6CnyNwgAC8" => {
                write!(f, "[Swap Program] {}", self.value)
            }
            "BPFLoaderUpgradeab1e11111111111111111111111" => {
                write!(f, "[BPF Upgradeable Loader] {}", self.value)
            }
            "BPFLoader2111111111111111111111111111111111" => {
                write!(f, "[BPF Loader 2] {}", self.value)
            }
            "BPFLoader1111111111111111111111111111111111" => {
                write!(f, "[BPF Loader] {}", self.value)
            }
            "NativeLoader1111111111111111111111111111111" => {
                write!(f, "[Native Loader] {}", self.value)
            }
            "ProgM6JCCvbYkfKqJYHePx4xxSUSqJp7rh8Lyv7nk7S" => {
                write!(f, "[Program Metadata Program] {}", self.value)
            }

            // Popular DeFi and NFT programs
            "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin" => {
                write!(f, "[Serum Dex Program v3] {}", self.value)
            }
            "EUqojwWA2rd19FZrzeBncJsm38Jm1hEhE3zsmX3bRc2o" => {
                write!(f, "[Serum Dex Program v2] {}", self.value)
            }
            "BJ3jrUzddfuSrZHXSCxMUUQsjKEyLmuuyZebkcaFp2fg" => {
                write!(f, "[Serum Dex Program v1] {}", self.value)
            }
            "22Y43yTVxuUkoRKdm9thyRhQ3SdgQS7c7kB6UNCiaczD" => {
                write!(f, "[Serum Swap Program] {}", self.value)
            }
            "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8" => {
                write!(f, "[Raydium AMM Program] {}", self.value)
            }
            "9HzJyW1qZsEiSfMUf6L2jo3CcTKAyBmSyKdwQeYisHrC" => {
                write!(f, "[Raydium IDO Program] {}", self.value)
            }
            "RVKd61ztZW9GUwhRbbLoYVRE5Xf1B2tVscKqwZqXgEr" => {
                write!(f, "[Raydium Liquidity Pool v1] {}", self.value)
            }
            "27haf8L6oxUeXrHrgEgsexjSY5hbVUWEmvv9Nyxg8vQv" => {
                write!(f, "[Raydium Liquidity Pool v2] {}", self.value)
            }
            "EhhTKczWMGQt46ynNeRX1WfeagwwJd7ufHvCDjRxjo5Q" => {
                write!(f, "[Raydium Staking Program] {}", self.value)
            }
            "DjVE6JNiYqPL2QXyCUUh8rNjHrbz9hXHNYt99MQ59qw1" => {
                write!(f, "[Orca Swap Program v1] {}", self.value)
            }
            "9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP" => {
                write!(f, "[Orca Swap Program v2] {}", self.value)
            }
            "82yxjeMsvaURa4MbZZ7WZZHfobirZYkH1zF8fmeGtyaQ" => {
                write!(f, "[Orca Aquafarm Program] {}", self.value)
            }
            "SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ" => {
                write!(f, "[Saber Stable Swap Program] {}", self.value)
            }
            "Crt7UoUR6QgrFrN7j8rmSQpUTNWNSitSwWvsWGf1qZ5t" => {
                write!(f, "[Saber Router Program] {}", self.value)
            }
            "SSwpMgqNDsyV7mAgN9ady4bDVu5ySjmmXejXvy2vLt1" => {
                write!(f, "[Step Finance Swap Program] {}", self.value)
            }
            "SWiMDJovFc7V7mCC9AyAUEx5N49d3Uhoi5hCVzDrD1U" => {
                write!(f, "[Swim Swap Program] {}", self.value)
            }
            "DtmE9D2CSB4L5D6A15mraeEjrGMm6auWVzgaD8hK2tZM" => {
                write!(f, "[Switchboard Oracle Program] {}", self.value)
            }
            "FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH" => {
                write!(f, "[Pyth Oracle Program] {}", self.value)
            }
            "gSbePebfvPy7tRqimPoVecS2uzBYPJ1zLaYV4MGmQ1i" => {
                write!(f, "[Mango Program v3] {}", self.value)
            }
            "5fNfvyp5czQVX77yoACa3JJVEhdRaWjPuazuWgjhTqEH" => {
                write!(f, "[Mango Program v2] {}", self.value)
            }
            "JD3bq9hGdy38PuWQ4h2YJpELmHVGPPfFSuFkpzAd9zfu" => {
                write!(f, "[Mango Program v1] {}", self.value)
            }
            "7sPptkymzvayoSbLXzBsXEF8TSf3typNnAWkrKrDizNb" => {
                write!(f, "[Mango ICO Program] {}", self.value)
            }
            "GqTPL6qRf5aUuqscLh8Rg2HTxPUXfhhAXDptTLhp1t2J" => {
                write!(f, "[Mango Governance Program] {}", self.value)
            }
            "MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD" => {
                write!(f, "[Marinade Staking Program] {}", self.value)
            }
            "CrX7kMhLC3cSsXJdT7JDgqrRVWGnUpX3gfEfxxU2NVLi" => {
                write!(f, "[Lido for Solana Program] {}", self.value)
            }
            "Port7uDYB3wk6GJAw4KT1WpTeMtSu9bTcChBHkX2LfR" => {
                write!(f, "[Port Finance Program] {}", self.value)
            }
            "So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo" => {
                write!(f, "[Solend Program] {}", self.value)
            }
            "MERLuDFBMmsHnsBPZw2sDQZHvXFMwp8EdjudcU2HKky" => {
                write!(f, "[Mercurial Stable Swap Program] {}", self.value)
            }
            "CJsLwbP1iu5DuUikHEJnLfANgKy6stB2uFgvBBHoyxwz" => {
                write!(f, "[Solanart] {}", self.value)
            }
            "5ZfZAwP2m93waazg8DkrrVmsupeiPEvaEHowiUP7UAbJ" => {
                write!(f, "[Solanart Global Offers] {}", self.value)
            }
            "DoqmnA7F7k5D1xdGQ4ejzZopfecgJYSWXe1WfJrveW1R" => {
                write!(f, "[Metaplex Candy Machine Legacy] {}", self.value)
            }
            "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d" => {
                write!(f, "[Metaplex Core Program] {}", self.value)
            },
            "cndy3Z4yapfJBmL3ShUp5exZKqR3z33thTzeNMm2gRZ" => {
                write!(f, "[Metaplex Candy Machine Core] {}", self.value)
            }
            "p1exdMJcjVao65QdewkaZRUnU6VPSXhus9n2GzWfh98" => {
                write!(f, "[Metaplex Fixed Price Sale] {}", self.value)
            }
            "auctxRXPeJoc4817jDhf4HbjnhXcVVui6q8F6DKkHEyv" => {
                write!(f, "[Metaplex Auction] {}", self.value)
            }
            "CMZYPASGWeTz7RNGHaRJfCq2XQ5pYK6nDvVQxzkH51zb" => {
                write!(f, "[Magic Eden Launchpad Candy Machine] {}", self.value)
            }
            "M2mx93ekt1fmXSVkTrUL9xVFHkmME8HTUi5Cyc5aF7K" => {
                write!(f, "[Magic Eden V2] {}", self.value)
            }
            "MEisE1HzehtrDpAAT8PnLHjpSSkRYakotTuJRPjTpo8" => {
                write!(f, "[Magic Eden V1] {}", self.value)
            }
            "hadeK9DLv9eA7ya5KCTqSvSvRZeJC3JgD5a9Y3CNbvu" => write!(f, "[Hadeswap] {}", self.value),
            "cndyAnrLdpjq1Ssp1z8xxFBBZLvtxVFZLQ674zmG3Bq" => {
                write!(f, "[Candy Machine Core] {}", self.value)
            }
            "CndyV3LdqHUfDLmE5naZjVN8rBZz4tqhdefbAnjHG3JR" => {
                write!(f, "[Candy Machine V2] {}", self.value)
            }
            "cndyJmQeYhErLfM7jvMmhRgWYtY88ahptQoZ5HkFvzVp" => {
                write!(f, "[Candy Machine V3] {}", self.value)
            }
            "DdtqB6JoAQzj5VKEtrXQpESamAgJvLw3k3SS9PD39PaY" => {
                write!(f, "[Candy Guard] {}", self.value)
            }
            "Guard1JwRhJkVH6XZhzoYxeBVQe872VH5QqU83RuijgJB" => {
                write!(f, "[Candy Guard Program] {}", self.value)
            }

            // Wormhole
            "3u8hJUVTA4jH1wYAyUur7FFZVQ8H635K3tSHHF4ssjQ5" => {
                write!(f, "[Wormhole Core Bridge] {}", self.value)
            }
            "DZnkkTmCiFWfYTfT41X3Rd1kDgozqzxWaHqsw6W4x2oe" => {
                write!(f, "[Wormhole Token Bridge] {}", self.value)
            }
            "2rHhojZ7hpu1zA91nvZmT8TqWWvMcKmmNBCr2mKTtMq4" => {
                write!(f, "[Wormhole NFT Bridge] {}", self.value)
            }
            "WormT3McKhFJ2RkiGpdw9s9BZCGb2SD1hq3sJ4jBnV8a" => {
                write!(f, "[Wormhole] {}", self.value)
            }

            // Other notable programs
            "L2TExMFKdjpN9kozasaurPirfHy9P8sbXoAN1qA3S95" => {
                write!(f, "[Lighthouse Program] {}", self.value)
            }
            "GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR" => {
                write!(f, "[OpenBook Dex] {}", self.value)
            }
            "srmqPvymJeFKQ4zGQed1GFppgkRBP9LHTpvgWrgsQ2r" => {
                write!(f, "[OpenBook Dex V2] {}", self.value)
            }
            "Dooar9JkhdZ7J3LHN3A7YCuoGRUggXhQaG4kijfLGU2j" => {
                write!(f, "[STEPN Dex] {}", self.value)
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
