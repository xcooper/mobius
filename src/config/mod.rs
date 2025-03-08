use serde::{Deserialize, Serialize};
use std::env;
use std::fs::read_to_string;
use std::fs::write;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use toml::{from_str, to_string};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Provider {
    OpenAi,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LLM {
    pub provider: Provider,
    pub model: String,
    pub api_key: Option<String>,
    pub url: Option<String>,
    pub temperature: f32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Config {
    pub llm: LLM,
}

pub fn load_config(cfg_path: &PathBuf) -> Result<Config, Error> {
    let content = read_to_string(cfg_path)?;
    let config: Config = from_str(&content).unwrap();
    Ok(config)
}

pub fn save_config(config: &Config, cfg_path: &PathBuf) -> Result<(), Error> {
    write(cfg_path, to_string(config).unwrap())?;
    Ok(())
}

pub fn get_config_path() -> Result<PathBuf, Error> {
    let os = env::consts::OS;
    match os {
        "linux" | "macos" => {
            let xdg_cfg_home = env::var("XDG_CONFIG_HOME");
            if let Ok(cfg_path) = xdg_cfg_home {
                Ok(PathBuf::from(cfg_path).join("mobius/config.toml"))
            } else {
                Ok(PathBuf::from("~/.config/mobius/config.toml"))
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
                provider: Provider::OpenAi,
                model: "gpt-3.5-turbo".to_string(),
                api_key: Some("test".to_string()),
                url: None,
                temperature: 0.7,
            },
        };
        let tmp_cfg_file = env::temp_dir().join("config.toml");
        save_config(&config, &tmp_cfg_file).expect("Failed to save config");
        let loaded_config = load_config(&tmp_cfg_file).expect("Failed to load config");
        assert_eq!(config, loaded_config);
    }
}
