# Solana Explorer CLI

:> [!WARNING]
> solana explorer CLI is in development. Your contribution is very welcomed! You can request features in a free form in [github issues](https://github.com/dmshvetsov/solana-explorer-cli/issues/new) and check existing [feature requests](https://github.com/dmshvetsov/solana-explorer-cli/issues?q=is%3Aissue+is%3Aopen+label%3A%22feature+request%22)

Read Solana Blockchain state from your terminal.

## Requirements

* Rust `>=1.79`

## Installation

    $ cargo install solana-explorer-cli

## Usage

By default this CLI uses Solana mainnet-beta RPC `http://api.mainnet-beta.solana.com`. For better experience provide your own RPC URL, preferably with [DAS API](https://developers.metaplex.com/rpc-providers#rp-cs-available) support

    $ export SE_RPC_URL=<your Solana RPC provider URL>

Start exploring

    $ se account <ADDRESS>

    $ se transaction <SIGNATURE HASH>
