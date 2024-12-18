set -e 

cargo build

SE=./target/debug/se 

echo "\n\npNFT"
$SE account 5soeS5y4jrsyaXSYpnxjrUT8qsJ3VVNGRtT2jK399gx7

echo "\n\nBERN token account"
$SE account 4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R

echo "\n\nCandy Machine"
$SE account Aq9PkNTgWdmhA24xAcKYYZwLDo1CfEUk6EqrZ87RsuEJ

echo "\n\ntx WSOL -> GOAT on Raydium swap"
$SE transaction 44cfUnsmWHfS1Kz4PcF9Rj4XoLSivfTYPunTQRLnvqAfhBqrYUV2mWoJUXr5goKAUqVp4g4XWBgLJ9rSYva7cxqq

echo "\n\ncNFT"
$SE account 9HNN54hfD3GVy4WkUtXjJdxaTo9tjFzYmEXYN9eHnLZp

echo "\n\nwallet"
$SE account HdxkiXqeN6qpK2YbG51W23QSWj3Yygc1eEk2zwmKJExp
