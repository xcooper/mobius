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
    Gemini,
}
