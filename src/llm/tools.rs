use std::collections::HashMap;
use std::{env::consts::OS, process::Command};

use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::CommandExecutionError;

pub(super) struct CheckCmdExist;

#[derive(Debug, Deserialize)]
pub struct CheckCmdExistArgs {
    cmds: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct CheckCmdExistResp {
    result: HashMap<String, bool>
}

impl CheckCmdExist {
    fn check_single_cmd(&self, cmd: &str) -> Result<bool, CommandExecutionError> {
        match OS {
            "linux" | "macos" => {
                Command::new("command")
                    .args(vec!["-v", &cmd])
                    .output()
                    .map(|out| out.status.success())
                    .map_err(|e| CommandExecutionError::new(e))
            }
            "windows" => {
                Command::new("Get-Command")
                    .args(vec!["-Name", &cmd, "-ErrorAction", "SilentlyContinue"])
                    .output()
                    .map(|out| out.status.success())
                    .map_err(|e| CommandExecutionError::new(e))
            }
            _ => Ok(false),
        }
    }
}

impl Tool for CheckCmdExist {
    const NAME: &'static str = "check_cmd_exist";

    type Error = CommandExecutionError;
    type Args = CheckCmdExistArgs;
    type Output = CheckCmdExistResp;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: self.name(),
            description: "Checks if all given commands exist in the system's PATH. Returns true if all commands are found, false otherwise.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "cmds": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "An array of command names to check for existence."
                    }
                },
                "required": ["cmds"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let cmds = &args.cmds;
        let mut all_results = HashMap::new();
        for cmd in cmds {
            let single_result = self.check_single_cmd(cmd)?;
            all_results.insert(cmd.clone(), single_result);
        }
        Ok(CheckCmdExistResp { result: all_results })
    }
}
