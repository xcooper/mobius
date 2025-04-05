use clap::command;
use clap::ArgAction;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(about="A CLI for integrating with AI.", long_about=None)]
pub struct ParsedArgs {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(short, long, action = ArgAction::Count, help = "Print debug information")]
    pub verbose: u8,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = false, about = "Initialize configurations")]
    Init {
        #[arg(short, long, default_value = "openai", help = "The AI provider to use")]
        provider: String,
        #[arg(
            short,
            long,
            default_value = "gpt-3.5-turbo",
            help = "The AI model to use"
        )]
        model: String,
        #[arg(short, long, help = "The API key for accessing the AI provider")]
        api_key: Option<String>,
    },
    #[command(
        arg_required_else_help = true,
        about = "Sends the prompt along with the stdin to AI"
    )]
    Pipe {
        #[arg(short, long, help = "The prompt, use '-' for reading from stdin")]
        prompt: String,
        #[arg(short, long, help = "The system prompt")]
        system_prompt: Option<String>,
    },
    #[command(arg_required_else_help = true)]
    AutoComplete {
        #[arg(short, long)]
        prompt: String,
    },
}
