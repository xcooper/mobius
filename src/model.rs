use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, ValueEnum, PartialEq)]
pub enum Shell {
    Zsh,
    Bash,
    PowerShell,
}

#[derive(Serialize, Deserialize, Debug, Clone, ValueEnum, PartialEq)]
pub enum Provider {
    OpenAI,
    Gemini,
}

#[derive(Serialize, Deserialize, Debug, Clone, ValueEnum, PartialEq)]
pub enum HotKey {
    CtrlSlash,
    AltSlash,
}

impl HotKey {
    pub fn to_env(&self) -> &str {
        match self {
            HotKey::CtrlSlash => "CTRL_SLASH",
            HotKey::AltSlash => "ALT_SLASH",
        }
    }
}
