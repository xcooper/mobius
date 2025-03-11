use crate::args_parser::{Commands, ParedArgs};
use crate::config::{default_config, save_config, Provider};
use std::env;
use std::error::Error;
use std::fmt::Display;

pub fn do_init(args: &ParedArgs) -> Result<(), CommandExecutionError> {
    let cmd = &args.command;
    if let Commands::Init {
        api_key,
        model,
        provider,
    } = cmd
    {
        let mut new_config = default_config();
        new_config.llm.provider = Provider::from(provider);
        new_config.llm.model = model.clone();
        if let Some(key) = api_key {
            new_config.llm.api_key = Some(key.clone());
        } else if let Ok(key) = env::var("OPENAI_API_KEY") {
            new_config.llm.api_key = Some(key);
        } else {
            return Err(CommandExecutionError::new("No API key found. Please provide one using --api-key or set OPENAI_API_KEY environment variable."));
        }

        if let Err(e) = save_config(&new_config) {
            return Err(CommandExecutionError::new("Failed to save config"));
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct CommandExecutionError<'a> {
    error_message: &'a str,
}

impl CommandExecutionError<'_> {
    fn new(msg: &str) -> CommandExecutionError {
        CommandExecutionError { error_message: msg }
    }
}

impl Error for CommandExecutionError<'_> {}

impl Display for CommandExecutionError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "command exec with error: {}", self.error_message)
    }
}
