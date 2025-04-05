use crate::args_parser::{Commands, ParsedArgs};
use crate::config::{default_config, load_config, save_config, Provider};
use crate::llm::{get_llm, LLM};
use crate::{echo, CommandExecutionError};
use std::env;
use std::io::{stdin, Read};

pub fn do_init(args: &ParsedArgs) -> Result<(), CommandExecutionError> {
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

        if let Err(_) = save_config(&new_config) {
            return Err(CommandExecutionError::new("Failed to save config"));
        }
    }
    Ok(())
}

pub async fn do_pipe(args: &ParsedArgs) -> Result<(), CommandExecutionError> {
    let cmd = &args.command;
    let config = load_config().map_err(|_| CommandExecutionError::new("can not load config"))?;
    if let Commands::Pipe {
        prompt,
        system_prompt,
    } = cmd
    {
        let mut user_prompt = String::from(prompt);
        if prompt == "-" {
            let mut stdin = stdin().lock();
            stdin
                .read_to_string(&mut user_prompt)
                .map_err(|_| CommandExecutionError::new("can not read stdin"))?;
        }
        let llm = get_llm(&config);
        return match llm
            .chat(
                system_prompt.as_ref().map_or(&default_sys_prompt(), |v| v),
                &user_prompt,
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

pub async fn do_autocomplete(args: &ParsedArgs) -> Result<(), CommandExecutionError> {
    let cmd = &args.command;
    if let Commands::AutoComplete { prompt } = cmd {
        let user_prompt = prompt;
        let config =
            load_config().map_err(|_| CommandExecutionError::new("can not load config"))?;
        let llm = get_llm(&config);
        return match llm.chat(&default_sys_prompt(), user_prompt).await {
            Ok(o) => {
                echo!(o);
                Ok(())
            }
            Err(e) => Err(CommandExecutionError::from_string(format!("{:?}", e))),
        };
    }
    Err(CommandExecutionError::new("invalid command"))
}

fn default_sys_prompt() -> String {
    match env::consts::OS {
        "linux" => "Be a Linux shell command assistant, only response with command, no wrapping quotes, be concise."
            .to_string(),
        "macos" => {
            "Be a Zsh command assistant, only response with command, no wrapping quotes, be concise.".to_string()
        }
        "windows" => {
            "Be a Windows power shell assistant, only response with command, no wrapping quotes, be concise."
                .to_string()
        }
        _ => "Be a shell command assistant, only response with command, no wrapping quotes, be concise.".to_string(),
    }
}
