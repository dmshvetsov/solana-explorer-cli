## to do

- wallets
  - [x] SOL balance
  - [x] list of tokens and balance
  - [ ] display owned domains
  - [ ] SOL stake balance
- better view of token account
  - NFT
    - [x] on-chain metadata
    - [ ] off-chain metadata
    - [ ] token extension
    - [ ] marketplaces activity list/unlist/sell/previous owners
    - must work and show specific token standard info
      - [?] pNFT
      - [x] MPL Core
      - [x] MPL legacy
      - [x] cNFT
    - [x] read Metaplaex token metadata account directly
  - [ ] SFT
  -  FT
    - [x] on-chain metadata
- programs
  - [ ] IDL view
- transactions
  - [ ] better view of a mint
  - [ ] better view of a transfer
- UX
  - [ ] more concise output for different accounts with ability to print all the data
  - [ ] `--explain` flag that printout explanation what each account (struct) does
  - [ ] output in pages, a page consist of a list of data instances (account, mint, token metadata, off-chain metadata, balanance, transactions etc)
    - [ ] implement output format for all supported `account` reads
      - [ ] all structs that goes to output must be local to the crate
    - [ ] add pages struct
      - [ ] build page(s) from (owner program, pubkey) tuple
      - [ ] read data from sources iterating through the pages tuples
      - [ ] output result
      - [ ] remove duplication of read logic e.g. token_metadata duplication
  - [ ] find solution to the problem that SE local structs hiding what structs were used to manipulate data in the code; this info could be valuable to know what structs were used to fetch and upack the data this helps developers to get same Rust libs and interact with Solana as SE crate does; possible solutions:
    - mimic the path and names of original structs from libriries
      - cons need to follow changes in original libraries
      - pros i have full control over my local struct
      - cons need a lot of boilre code
    - add info to "explanation" section of Page output
      - seem like a good tradeoff
      - pros i have full control over my local struct
      - cons need a lot of boilre code
    - keep using original structs
      - cons some structs is hard to serialized to JSON, can be more blocks with other formats or manipulations

## relevant info

- most known programs https://github.com/helius-labs/xray/blob/900dd5a03c914cc435c958eccba94f2dc22f0372/src/lib/xray/config.ts

## inspiration

- https://github.com/cavemanloverboy/sol
- https://github.com/coral-xyz/anchor/tree/master/cli

## DAS API

- https://github.com/metaplex-foundation/digital-asset-standard-api/blob/main/clients/js/src/decorator.ts
