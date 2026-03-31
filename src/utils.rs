use anyhow::{anyhow, Result};

use crate::model::{Shell, OS};
use std::env::consts;
use std::env::var;

fn get_os() -> Result<OS> {
    let os_str = consts::OS;
    match os_str {
        "windows" => Result::Ok(OS::Windows),
        "macos" => Result::Ok(OS::MacOS),
        "linux" => Result::Ok(OS::Linux),
        _ => Result::Err(anyhow!("OS is unsupported")),
    }
}

fn get_shell() -> Result<Shell> {
    let os = get_os()?;
    if os == OS::Windows {
        if let Ok(val) = var("ShellId") {
            if val.to_lowercase().contains("powershell") {
                return Ok(Shell::PowerShell);
            }
        }
    } else {
        if let Ok(val) = var("SHELL") {
            let lower = val.to_lowercase();
            if lower.contains("zsh") {
                return Ok(Shell::Zsh);
            } else if lower.contains("bash") {
                return Ok(Shell::Bash);
            }
        }
    }
    Err(anyhow!("Unable to detect shell"))
}

#[cfg(test)]
mod test {
    use super::get_os;
    use crate::{
        model::{Shell, OS},
        utils::get_shell,
    };

    #[test]
    fn test_get_os() {
        assert!([OS::Windows, OS::Linux, OS::MacOS].contains(&get_os().unwrap()));
    }

    #[test]
    fn test_get_shell() {
        assert!([Shell::Bash, Shell::Zsh, Shell::PowerShell].contains(&get_shell().unwrap()));
    }
}
