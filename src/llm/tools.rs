use log::debug;
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::process::Command;

use crate::model::Shell;
use crate::CommandExecutionError;

pub(super) struct CheckCmdExist;

#[derive(Debug, Deserialize)]
pub struct CheckCmdExistArgs {
    cmds: Vec<String>,
    shell: Shell,
}

#[derive(Debug, Serialize)]
pub struct CheckCmdExistResp {
    result: HashMap<String, bool>,
}

impl CheckCmdExist {
    fn check_single_cmd(&self, cmd: &str, shell: &Shell) -> Result<bool, CommandExecutionError> {
        debug!("checking cmd: {}", cmd);
        match shell {
            Shell::Zsh => Command::new("zsh")
                .args(vec!["-c", format!("command -v {}", cmd).as_str()])
                .output()
                .map(|out| out.status.success())
                .map_err(CommandExecutionError::new),
            Shell::Bash => Command::new("bash")
                .args(vec!["-c", format!("command -v {}", cmd).as_str()])
                .output()
                .map(|out| out.status.success())
                .map_err(CommandExecutionError::new),
            Shell::PowerShell => Command::new("pwsh")
                .args(vec![
                    "-Command",
                    &format!("Get-Command -Name {} -ErrorAction SilentlyContinue", cmd),
                ])
                .output()
                .map(|out| out.status.success())
                .map_err(|e| CommandExecutionError::new(e)),
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
                    },
                    "shell": {
                        "type": "string",
                        "enum": ["Zsh", "Bash", "PowerShell"],
                        "description": "The shell the command runs in.",
                    }
                },
                "required": ["cmds", "shell"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let cmds = &args.cmds;
        let shell = &args.shell;
        let mut all_results = HashMap::new();
        for cmd in cmds {
            let single_result = self.check_single_cmd(cmd, shell)?;
            all_results.insert(cmd.clone(), single_result);
        }
        debug!("check command results: {:?}", all_results);
        Ok(CheckCmdExistResp {
            result: all_results,
        })
    }
}
