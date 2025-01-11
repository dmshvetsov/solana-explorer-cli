mod account;
mod balance;
mod magiceden;
mod metaplex;
mod output;
mod rpc;
mod token;
mod transaction;

use account::reader::read_account;
use clap::{Args, Parser, Subcommand};
use output::OutputFormat;
use transaction::{list_account_txs, read_tx};

/// Solana explorer CLI utility
/// with a goal to explore all account and tx on Solana
#[derive(Parser)]
#[command(name = "solana explorer", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Resource,
}

#[derive(Subcommand)]
enum Resource {
    Account(AccountCommand),
    Ac(AccountCommand),
    Transaction(TransactionCommand),
    Tx(TransactionCommand),
    AccountTransactions(ListAccountTransactionsCommand),
    AcTxs(ListAccountTransactionsCommand),
}

#[derive(Args, Debug)]
struct AccountCommand {
    /// public account address
    address: String,
    #[arg(short, long)]
    format: Option<OutputFormat>,
}

#[derive(Args, Debug)]
struct TransactionCommand {
    /// hash of transaction signature
    signature: String,
}

#[derive(Args, Debug)]
struct ListAccountTransactionsCommand {
    /// public account address
    address: String,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Resource::Account(args) | Resource::Ac(args) => {
            read_account(
                &args.address,
                args.format.clone().unwrap_or(OutputFormat::AsStruct),
            );
        }
        Resource::Transaction(args) | Resource::Tx(args) => {
            read_tx(&args.signature);
        }
        Resource::AccountTransactions(args) | Resource::AcTxs(args) => {
            list_account_txs(&args.address);
        }
    }
}
