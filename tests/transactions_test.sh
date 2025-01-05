set -e

SE=./target/debug/se

echo "\n\ntransaction WSOL -> GOAT on Raydium swap"
$SE transaction 44cfUnsmWHfS1Kz4PcF9Rj4XoLSivfTYPunTQRLnvqAfhBqrYUV2mWoJUXr5goKAUqVp4g4XWBgLJ9rSYva7cxqq

echo "\n\ntransaction WSOL -> GOAT on Raydium swap with short tx alias"
$SE tx 44cfUnsmWHfS1Kz4PcF9Rj4XoLSivfTYPunTQRLnvqAfhBqrYUV2mWoJUXr5goKAUqVp4g4XWBgLJ9rSYva7cxqq

echo "\n\nlist of account transactions"
$SE account-transactions HEcFC9JDPR2CrMmUZ6nJpDKfbuFsg26DGYEJWw9B988E

echo "\n\nlist of account transactions with short ac-txs alias"
$SE ac-txs HEcFC9JDPR2CrMmUZ6nJpDKfbuFsg26DGYEJWw9B988E
