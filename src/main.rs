use clap::Parser;
use mobius::args_parser::{Commands, ParedArgs};
use mobius::command::do_init;

fn main() {
    let args = ParedArgs::parse();
    stderrlog::new()
        .verbosity(args.verbose as usize)
        .init()
        .unwrap();
    match args.command {
        Commands::Init { .. } => {
            let _ = do_init(&args);
        }
        Commands::Pipe { .. } => {}
        Commands::AutoComplete { .. } => {}
    }
}
