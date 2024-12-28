use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use std::env;

pub fn init_connection() -> RpcClient {
    // TODO: memoize created client
    let rpc_url =
        env::var("SE_RPC_URL").unwrap_or("http://api.mainnet-beta.solana.com".to_string());
    RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed())
}
