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
  - more concise output for different accounts with ability to print all the data
  - `--explain` flag that printout explanation what each account (struct) does

## relevant info

- most known programs https://github.com/helius-labs/xray/blob/900dd5a03c914cc435c958eccba94f2dc22f0372/src/lib/xray/config.ts

## inspiration

- https://github.com/cavemanloverboy/sol
- https://github.com/coral-xyz/anchor/tree/master/cli

## DAS API

- https://github.com/metaplex-foundation/digital-asset-standard-api/blob/main/clients/js/src/decorator.ts
