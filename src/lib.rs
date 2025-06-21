use std::{error::Error, fmt::Display};

pub mod args_parser;
pub mod command;
pub mod config;
pub mod llm;
pub mod model;

macro_rules! echo {
    ($arg:expr) => {
        println!("{}", $arg)
    };
}
pub(crate) use echo;

#[derive(Debug)]
pub struct CommandExecutionError {
    error_message: String,
}

impl From<String> for CommandExecutionError {
    fn from(msg: String) -> Self {
        CommandExecutionError {
            error_message: msg,
        }
    }
}

impl From<&str> for CommandExecutionError {
    fn from(msg: &str) -> Self {
        CommandExecutionError {
            error_message: msg.to_string(),
        }
    }
}

impl Error for CommandExecutionError {}

impl Display for CommandExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "command exec with error: {}", self.error_message)
    }
}
