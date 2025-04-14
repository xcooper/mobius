use crate::args_parser::{Commands, ParsedArgs};
use crate::config::{default_config, load_config, save_config};
use crate::llm::{get_llm, get_llm_url, LLM};
use crate::model::Shell;
use crate::{echo, CommandExecutionError};
use std::env;
use std::io::{stdin, Read};

pub fn do_init(args: &ParsedArgs) -> Result<(), CommandExecutionError> {
    let cmd = &args.command;
    if let Commands::Init {
        api_key,
        llm_url,
        model,
        provider,
    } = cmd
    {
        let mut new_config = default_config();
        new_config.llm.provider = provider.clone();
        new_config.llm.model = model.clone();
        if let Some(u) = llm_url {
            new_config.llm.url = Some(u.clone());
        } else {
            new_config.llm.url = get_llm_url(&new_config.llm.provider);
        }
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

pub async fn do_chat(args: &ParsedArgs) -> Result<(), CommandExecutionError> {
    let cmd = &args.command;
    let config = load_config().map_err(|_| CommandExecutionError::new("can not load config"))?;
    if let Commands::Chat {
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
    if let Commands::AutoComplete { shell } = &args.command {
        return match env::consts::OS {
            "linux" => {
                if let Some(s) = shell {
                    return if s == &Shell::Bash {
                        echo!(include_str!("../../scripts/bash_autocomplete.bash"));
                        Ok(())
                    } else if s == &Shell::Zsh {
                        echo!(include_str!("../../scripts/zsh_autocomplete.zsh"));
                        Ok(())
                    } else {
                        Err(CommandExecutionError::new("Must specify shell"))
                    }
                } else {
                    Err(CommandExecutionError::new("Unsupported Platform"))
                }
            },
            "macos" => {
                echo!(include_str!("../../scripts/zsh_autocomplete.zsh"));
                Ok(())
            },
            "windows" => {
                echo!(include_str!("../../scripts/pwsh_autocomplete.ps1"));
                Ok(())
            },
            _ => Err(CommandExecutionError::new("Unsupported Platform"))
        }
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
