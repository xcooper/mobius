use clap::Parser;
use mobius::args_parser::Cli;

fn main() {
    let args = Cli::parse();
    println!("Hello {:?}!", args.command);
}
