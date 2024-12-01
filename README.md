# Solana Explorer CLI

Read Solana Blockchain state from your terminal.

## Requirements

* Rust `>=1.79`

## Installation

    $ cargo install solana-explorer-cli

## Usage

By default this CLI uses Solana mainnet-beta RPC `http://api.mainnet-beta.solana.com`. For better experience provide your own RPC URL, prefferably with [DAS API](https://developers.metaplex.com/rpc-providers#rp-cs-available) support

    $ export SE_RPC_URL=<your Solana RPC provider URL>

Start exploring

    $ se account <ADDRESS>
