use crate::args_parser::{Commands, ParsedArgs};
use crate::config::{default_config, load_config, save_config};
use crate::llm::get_llm;
use crate::model::{HotKey, Shell};
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
        }
        if let Some(key) = api_key {
            new_config.llm.api_key = Some(key.clone());
        }
        if let Err(_) = save_config(&new_config) {
            return Err(CommandExecutionError::new("Failed to save config"));
        }
    }
    Ok(())
}

pub async fn do_chat(args: &ParsedArgs) -> Result<(), CommandExecutionError> {
    let cmd = &args.command;
    let config = load_config()
        .map_err(|e| CommandExecutionError::new(format!("can not load config: {}", e)))?;
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
                vec![&user_prompt],
            )
            .await
        {
            Ok(o) => {
                echo!(o);
                Ok(())
            }
            Err(e) => Err(CommandExecutionError::new(format!("{:?}", e))),
        };
    }
    Err(CommandExecutionError::new("invalid command"))
}

pub async fn do_exec(args: &ParsedArgs) -> Result<(), CommandExecutionError> {
    let cmd = &args.command;
    let config = load_config()
        .map_err(|e| CommandExecutionError::new(format!("can not load config: {}", e)))?;
    if let Commands::Exec {
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
            .exec(
                system_prompt.as_ref().map_or(&default_sys_prompt(), |v| v),
                vec![&user_prompt],
            )
            .await
        {
            Ok(o) => {
                echo!(o);
                Ok(())
            }
            Err(e) => Err(CommandExecutionError::new(format!("{:?}", e))),
        };
    }
    Err(CommandExecutionError::new("invalid command"))
}

pub async fn do_autocomplete(args: &ParsedArgs) -> Result<(), CommandExecutionError> {
    if let Commands::AutoComplete { shell, hot_key } = &args.command {
        let env_val = hot_key
            .as_ref()
            .map_or(HotKey::CtrlSlash.to_env(), HotKey::to_env);
        return match env::consts::OS {
            "linux" => {
                if let Some(s) = shell {
                    return if s == &Shell::Bash {
                        echo!(format!("MOBIUS_KEY_BINDING={}", env_val));
                        fix_eof(
                            include_str!("../../scripts/bash_autocomplete.bash"),
                            "linux",
                        );
                        Ok(())
                    } else if s == &Shell::Zsh {
                        echo!(format!("MOBIUS_KEY_BINDING={}", env_val));
                        fix_eof(include_str!("../../scripts/zsh_autocomplete.zsh"), "linux");
                        Ok(())
                    } else if s == &Shell::PowerShell {
                        echo!(format!("$MOBIUS_KEY_BINDING = \"{}\"", env_val));
                        fix_eof(include_str!("../../scripts/pwsh_autocomplete.ps1"), "linux");
                        Ok(())
                    } else {
                        Err(CommandExecutionError::new("Must specify shell"))
                    };
                } else {
                    Err(CommandExecutionError::new("Unsupported Platform"))
                }
            }
            "macos" => {
                echo!(format!("MOBIUS_KEY_BINDING={}", env_val));
                fix_eof(include_str!("../../scripts/zsh_autocomplete.zsh"), "macos");
                Ok(())
            }
            "windows" => {
                echo!(format!("$MOBIUS_KEY_BINDING = \"{}\"", env_val));
                fix_eof(
                    include_str!("../../scripts/pwsh_autocomplete.ps1"),
                    "windows",
                );
                Ok(())
            }
            _ => Err(CommandExecutionError::new("Unsupported Platform")),
        };
    }
    Err(CommandExecutionError::new("invalid command"))
}

fn default_sys_prompt() -> String {
    String::from("You are a good assistant, the response should be concise.")
}

fn fix_eof(file_content: &str, os: &str) {
    match os {
        "linux" | "macos" => {
            echo!(file_content.replace("\r\n", "\n"));
        }
        "windows" => {
            if !file_content.contains("\r\n") {
                echo!(file_content.replace("\n", "\r\n"));
            } else {
                echo!(file_content.to_string());
            }
        }
        _ => panic!("Unsupported platform"),
    }
}
