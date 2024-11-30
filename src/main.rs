mod account;
mod metaplex {
    pub mod das;
}
mod magiceden;

use account::read_account;
use clap::{Args, Parser, Subcommand};

/// Solana explorer with a built in serializer for most common accounts data
/// currently supported accounts are SPL Token accounts
#[derive(Parser)]
#[command(name = "solana explorer", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Resource,
}

#[derive(Subcommand)]
enum Resource {
    Account(AccountCommand),
}

#[derive(Args, Debug)]
struct AccountCommand {
    /// account address
    address: String,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Resource::Account(args) => {
            read_account(&args.address);
            // let account = get_account(&args.address);
            // print_output(&account);
            // if !account.data.is_empty() {
            //     let account_data = read_account_data(&account);
            //     print_output(&account_data);
            // }
        }
    }
}
