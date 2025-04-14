use serde::{Deserialize, Serialize};
use crate::model::Provider;
use std::env;
use std::fs::write;
use std::fs::{create_dir_all, read_to_string};
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use toml::{from_str, to_string};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LLM {
    pub provider: Provider,
    pub model: String,
    pub api_key: Option<String>,
    pub url: Option<String>,
    pub default_temperature: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Config {
    pub llm: LLM,
}

pub fn default_config() -> Config {
    let api_key = env::var("OPENAI_API_KEY").ok();
    Config {
        llm: LLM {
            provider: Provider::OpenAI,
            model: "gpt-3.5-turbo".to_string(),
            api_key,
            url: Some(String::from("https://api.openai.com/v1")),
            default_temperature: 0.0,
        },
    }
}

pub fn load_config() -> Result<Config, Error> {
    let cfg_path = get_config_path()?;
    load_config_from(&cfg_path)
}

fn load_config_from(cfg_path: &PathBuf) -> Result<Config, Error> {
    let content = read_to_string(cfg_path)?;
    let toml_parsing: Result<Config, toml::de::Error> = from_str(&content);
    toml_parsing.map_err(|e| Error::new(ErrorKind::InvalidData, e))
}

pub fn save_config(config: &Config) -> Result<(), Error> {
    let cfg_path = get_config_path()?;
    if !cfg_path.exists() {
        if let Some(parent) = cfg_path.parent() {
            create_dir_all(parent)?;
        }
    }
    save_config_to(&cfg_path, config)
}

fn save_config_to(cfg_path: &PathBuf, config: &Config) -> Result<(), Error> {
    write(cfg_path, to_string(config).unwrap())
}

pub fn get_config_path() -> Result<PathBuf, Error> {
    let os = env::consts::OS;
    match os {
        "linux" | "macos" => {
            let xdg_cfg_home = env::var("XDG_CONFIG_HOME");
            if let Ok(cfg_path) = xdg_cfg_home {
                Ok(PathBuf::from(cfg_path).join("mobius/config.toml"))
            } else {
                let home = env::var("HOME").unwrap();
                Ok(PathBuf::from(home).join(".config/mobius/config.toml"))
            }
        }
        "windows" => {
            let appdata = env::var("APPDATA");
            if let Ok(cfg_path) = appdata {
                Ok(PathBuf::from(cfg_path).join("mobius\\config.toml"))
            } else {
                Err(Error::new(
                    ErrorKind::NotFound,
                    "%APPDATA% cannot be resolved",
                ))
            }
        }
        _ => Err(Error::new(ErrorKind::Unsupported, "OS not supported")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;

    #[test]
    fn test_get_config_path() {
        let os = env::consts::OS;
        let path = get_config_path().unwrap();
        match os {
            "linux" | "macos" => {
                assert_eq!(path.to_str().unwrap(), "~/.config/mobius/config.toml");
            }
            "windows" => {
                assert_eq!(
                    path.to_str().unwrap(),
                    format!("{}\\mobius\\config.toml", env::var("APPDATA").unwrap())
                );
            }
            _ => {
                panic!("OS not supported");
            }
        }
    }

    #[test]
    fn test_read_write_config() {
        let config = Config {
            llm: LLM {
                provider: Provider::OpenAI,
                model: "gpt-3.5-turbo".to_string(),
                api_key: Some("test".to_string()),
                url: None,
                default_temperature: 0.7,
            },
        };
        let tmp_cfg_file = env::temp_dir().join("test_read_write_config.toml");
        remove_file(&tmp_cfg_file).ok();
        save_config_to(&tmp_cfg_file, &config).expect("Failed to save config");
        let loaded_config = load_config_from(&tmp_cfg_file).expect("Failed to load config");
        assert_eq!(config, loaded_config);
    }
}
