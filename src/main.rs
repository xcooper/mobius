use clap::Parser;
use log::error;
use mobius::args_parser::{Commands, ParedArgs};
use mobius::command::{do_init, do_pipe};

#[tokio::main]
async fn main() {
    let args = ParedArgs::parse();
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
        Commands::Pipe { .. } => {
            if let Err(e) = do_pipe(&args).await {
                error!("{}", e);
            }
        }
        Commands::AutoComplete { .. } => {}
    }
}
