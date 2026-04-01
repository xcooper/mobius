use anyhow::{anyhow, Result};

use crate::model::{Shell, OS};
use std::env::consts;
use std::env::var;
use std::env::current_dir;
use std::path::PathBuf;

pub fn get_os() -> Result<OS> {
    let os_str = consts::OS;
    match os_str {
        "windows" => Result::Ok(OS::Windows),
        "macos" => Result::Ok(OS::MacOS),
        "linux" => Result::Ok(OS::Linux),
        _ => Result::Err(anyhow!("OS is unsupported")),
    }
}

pub fn get_shell() -> Result<Shell> {
    let os = get_os()?;
    if os == OS::Windows {
        if let Ok(val) = var("ShellId") {
            if val.to_lowercase().contains("powershell") {
                return Ok(Shell::PowerShell);
            }
        }
    } else if let Ok(val) = var("SHELL") {
        let lower = val.to_lowercase();
        if lower.contains("zsh") {
            return Ok(Shell::Zsh);
        } else if lower.contains("bash") {
            return Ok(Shell::Bash);
        }
    }
    Err(anyhow!("Unable to detect shell"))
}

pub fn get_cwd() -> Result<PathBuf> {
    current_dir().map_err(|e| anyhow!("Unable to get current working directory: {}", e))
}

#[cfg(test)]
mod test {
    use super::{get_cwd, get_os, get_shell};
    use crate::model::{Shell, OS};

    #[test]
    fn test_get_os() {
        assert!([OS::Windows, OS::Linux, OS::MacOS].contains(&get_os().unwrap()));
    }

    #[test]
    fn test_get_shell() {
        assert!([Shell::Bash, Shell::Zsh, Shell::PowerShell].contains(&get_shell().unwrap()));
    }

    #[test]
    fn test_get_cwd() {
        let cwd = get_cwd().unwrap();
        assert!(cwd.is_absolute());
    }
}
