use std::env;
use clap::Parser;
use log::error;
use mobius::args_parser::{Commands, ParsedArgs};
use mobius::command::{do_autocomplete, do_chat, do_exec, do_init};

#[tokio::main]
async fn main() {
    let mut args = ParsedArgs::parse();
    if env::var("DEBUG").is_ok() {
        args.verbose = 3;
    }
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
        Commands::Exec { .. } => {
            if let Err(e) = do_exec(&args).await {
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
