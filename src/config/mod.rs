use std::env;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

pub enum Provider {
    OpenAi,
}

pub struct LLM {
    pub provider: Provider,
    pub model: String,
    pub api_key: Option<String>,
    pub url: Option<String>,
    pub temperature: f32,
}

pub struct Config {
    pub llm: LLM,
}

pub fn load_config() -> Config {
    todo!()
}

pub fn save_config() {
    todo!()
}

fn get_config_path() -> Result<PathBuf, std::io::Error> {
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
}
