use clap::command;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(about="A CLI for integrating with AI.", long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    Init {
        #[arg(short, long)]
        provider: String,
        #[arg(short, long)]
        model: String,
        #[arg(short, long)]
        api_key: String,
    },
    #[command(arg_required_else_help = true)]
    Pipe {
        #[arg(short, long)]
        prompt: String,
    },
    #[command(arg_required_else_help = true)]
    AutoComplete {
        #[arg(short, long)]
        prompt: String,
    },
}
