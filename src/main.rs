use clap::Parser;
use log::error;
use mobius::args_parser::{Commands, ParsedArgs};
use mobius::command::{do_autocomplete, do_chat, do_init};
use std::env;

#[tokio::main]
async fn main() {
    let args = ParsedArgs::parse();
    let mut log_lv = log::Level::Info;
    if env::var("DEBUG").is_ok() || args.verbose > 0 {
        log_lv = log::Level::Debug;
    }
    simple_logger::init_with_level(log_lv).unwrap();
    match args.command {
        Commands::Init { .. } => {
            if let Err(e) = do_init(&args).await {
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
