use crate::args_parser::{Commands, ParedArgs};
use crate::config::{default_config, load_config, save_config, Provider};
use crate::llm::{get_llm, LLM};
use crate::{echo, CommandExecutionError};
use std::env;

const DEF_SYS_PROMPT: &str = "You are a helpful assistant";

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

pub async fn do_pipe(args: &ParedArgs) -> Result<(), CommandExecutionError> {
    let cmd = &args.command;
    let config = load_config().map_err(|_| CommandExecutionError::new("can not load config"))?;
    if let Commands::Pipe {
        prompt,
        system_prompt,
    } = cmd
    {
        let llm = get_llm(&config);
        return match llm
            .chat(
                &system_prompt.as_ref().map_or(DEF_SYS_PROMPT, |v| v),
                &prompt,
            )
            .await
        {
            Ok(o) => {
                echo!(o);
                Ok(())
            }
            Err(e) => Err(CommandExecutionError::from_string(format!("{:?}", e))),
        };
    }
    Err(CommandExecutionError::new("invalid command"))
}
