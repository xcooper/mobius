use crate::model::Provider;
use crate::model::Shell;
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
        #[arg(short, long, help = "The AI provider to use")]
        provider: Provider,
        #[arg(short, long, help = "The AI model to use, e.g., gpt-4o")]
        model: String,
        #[arg(short, long, help = "The API key for accessing the AI provider")]
        api_key: Option<String>,
        #[arg(
            long,
            help = "The URL for accessing the AI provider, only need by in-house LLMs."
        )]
        llm_url: Option<String>,
    },
    #[command(
        arg_required_else_help = true,
        about = "Sends the prompt along with the stdin to AI"
    )]
    Chat {
        #[arg(short, long, help = "The prompt, use '-' for reading from stdin")]
        prompt: String,
        #[arg(short, long, help = "The system prompt")]
        system_prompt: Option<String>,
    },
    #[command(
        arg_required_else_help = false,
        about = "Generate CLI auto-complete script based on the OS and the shell"
    )]
    AutoComplete {
        #[arg(long)]
        shell: Option<Shell>,
    },
}
