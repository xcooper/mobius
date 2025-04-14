use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum Shell {
    Zsh,
    Bash,
    PowerShell,
}

#[derive(Serialize, Deserialize, Debug, Clone, ValueEnum, PartialEq)]
pub enum Provider {
    OpenAI,
}

impl From<&String> for Provider {
    fn from(s: &String) -> Self {
        match s.as_str() {
            "openai" => Provider::OpenAI,
            _ => panic!("Invalid provider"),
        }
    }
}