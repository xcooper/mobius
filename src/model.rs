use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, ValueEnum, PartialEq)]
pub enum Shell {
    Zsh,
    Bash,
    PowerShell,
}

impl From<Shell> for &str {
    fn from(value: Shell) -> Self {
        match value {
            Shell::Zsh => "ZSH",
            Shell::Bash => "BASH",
            Shell::PowerShell => "PowerShell",
        }
    }
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum OS {
    MacOS,
    Linux,
    Windows,
}

impl From<OS> for &str {
    fn from(val: OS) -> Self {
        match val {
            OS::MacOS => "MacOS",
            OS::Linux => "Linux",
            OS::Windows => "Windows",
        }
    }
}
