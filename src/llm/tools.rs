use std::{env::consts::OS, process::Command};

use rig::{completion::ToolDefinition, tool::Tool};
use serde_json::json;

use crate::CommandExecutionError;

struct CheckCmdExist;

impl Tool for CheckCmdExist {
    const NAME: &'static str = "check_cmd_exist";
    
    type Error = CommandExecutionError;
    
    type Args = String;
    
    type Output = bool;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: self.name(),
            description: "Checks if a given command exists in the system's PATH. Returns true if the command is found, false otherwise.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "cmd": {
                        "type": "string",
                        "description": "The name of the command to check for existence."
                    }
                },
                "required": ["cmd"]
            }),
        }
    }

    async fn call(
        &self,
        args: Self::Args,
    ) -> Result<Self::Output, Self::Error> {
        let cmd = args;
        return match OS {
            "linux" | "macos" => {
                let output = Command::new("command").args(vec!["-v", &cmd]).output();
                return Ok(output.is_ok());
            }
            "windows" => {
                let output = Command::new("Get-Command")
                    .args(vec!["-Name", &cmd, "-ErrorAction", "SilentlyContinue"])
                    .output();
                return Ok(output.is_ok());
            }
            _ => Ok(false),
        };
    }
}
