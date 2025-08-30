use std::{env::consts::OS, process::Command};

pub(super) fn check_cmd_exist(
	cmd: &str,
) -> bool {
	return match OS {
		"linux" | "macos" => {
			let output = Command::new("command")
				.args(vec!["-v", cmd])
				.output();
			return output.is_ok();
		},
		"windows" => {
			let output = Command::new("Get-Command")
				.args(vec!["-Name", cmd, "-ErrorAction", "SilentlyContinue"])
				.output();
			return output.is_ok();
		},
		_ => false,
	}
}