set -e

SE=./target/debug/se

# NFTs

echo "\n\npNFT"
$SE account 5soeS5y4jrsyaXSYpnxjrUT8qsJ3VVNGRtT2jK399gx7
sleep 0.5

echo "\n\n Metaplex metadata account"
$SE account FeYYbECgnky3rWrmxBB7cybWgyUNEVKwQQWb8oU8DnMj
sleep 0.5

echo "\n\np MPL Core NFT"
$SE account HGFDAByQskKo9Az9yAtgg6qP8TnwMayZGKbyi3vkanzT
sleep 0.5

echo "\n\np MPL Core NFT with json format"
$SE account HGFDAByQskKo9Az9yAtgg6qP8TnwMayZGKbyi3vkanzT -f json
sleep 0.5

echo "\n\ncNFT"
$SE account 9HNN54hfD3GVy4WkUtXjJdxaTo9tjFzYmEXYN9eHnLZp
sleep 0.5

# FTs

echo "\n\nRAYDIUM token mint"
$SE account 4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R
sleep 0.5

echo "\n\nRAYDIUM token mint with short ac alias"
$SE ac 4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R
sleep 0.5

echo "\n\ntoken account for RAYDIUM"
$SE account 2UYALq5MMJbDsxyftrjWjP1jDfQ3iQHXTEXh6zGxoo9H

# Magic Eden Laucnhpads

# FIXME: failing
# echo "\n\nCandy Machine Aq9PkNTgWdmhA24xAcKYYZwLDo1CfEUk6EqrZ87RsuEJ"
# $SE account Aq9PkNTgWdmhA24xAcKYYZwLDo1CfEUk6EqrZ87RsuEJ
# sleep 0.5

echo "\n\nMagic Eden Core Open Edition"
$SE account DQebYkR5JxtpCpvNJu1u3FEJDhat3kkVgk3bbBr6iAAW
sleep 0.5

echo "\n\nMagic Eden Launchpad #1"
$SE account D2Fbhy8pPi47w7gaDEYmSzskAAQFUqiHG53hUE71HpC2
sleep 0.5

echo "\n\nMagic Eden Launchpad #2"
$SE account F1obR8rmTi7a22TE3qRTYsJmeN5YCNgUcuPSwLK8Cooc
sleep 0.5

echo "\n\nMagic Eden Launchpad #3"
$SE account HG524wasxJYwHEoxQHdmUn7fryk4YDHFJAHXBHZ2LKCr
sleep 0.5

# Wallets

echo "\n\nsmall wallet"
$SE account J1MdztLMncohKGrfkTASnftzQC3ssKacY9FYQ43Lcowf

# TODO: fix large acc rpc requests
# echo "\n\nlarge wallet"
# $SE account HdxkiXqeN6qpK2YbG51W23QSWj3Yygc1eEk2zwmKJExp
