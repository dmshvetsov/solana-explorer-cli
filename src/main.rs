mod account;
mod balance;
mod output;
mod rpc;
mod token;
mod transaction;
mod metaplex;
mod magiceden;

use account::read_account;
use output::OutputFormat;
use transaction::read_tx;
use clap::{Args, Parser, Subcommand};

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
    Tx(TransactionCommand)
}

#[derive(Args, Debug)]
struct AccountCommand {
    /// account address
    address: String,
    #[arg(short, long)]
    format: Option<OutputFormat>,
}

#[derive(Args, Debug)]
struct TransactionCommand {
    signature: String,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Resource::Account(args) | Resource::Ac(args) => {
            read_account(&args.address, args.format.clone().unwrap_or(OutputFormat::AsStruct));
        }
        Resource::Transaction(args) | Resource::Tx(args) => {
            read_tx(&args.signature);
        }
    }
}
