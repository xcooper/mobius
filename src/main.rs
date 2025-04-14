use clap::Parser;
use log::error;
use mobius::args_parser::{Commands, ParsedArgs};
use mobius::command::{do_autocomplete, do_init, do_chat};

#[tokio::main]
async fn main() {
    let args = ParsedArgs::parse();
    stderrlog::new()
        .verbosity(args.verbose as usize)
        .init()
        .unwrap();
    match args.command {
        Commands::Init { .. } => {
            if let Err(e) = do_init(&args) {
                error!("{}", e);
            }
        }
        Commands::Chat { .. } => {
            if let Err(e) = do_chat(&args).await {
                error!("{}", e);
            }
        }
        Commands::AutoComplete { .. } => {
            if let Err(e) = do_autocomplete(&args).await {
                error!("{}", e);
            }
        }
    }
}
