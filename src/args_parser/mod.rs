use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ParsingError {
	pub message: String
}

impl Display for ParsingError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "parse args error: {}", self.message)
	}
}

impl Error for ParsingError {}

pub enum Command {
	Invalid,
	Config,
}

impl From<&String> for Command {
	fn from(value: &String) -> Self {
		match value.to_lowercase().as_str() {
			"config" => Command::Config,
			_ => Command::Invalid,
		}
	}
}

#[derive(Default)]
pub struct ParsedArgs {
	pub command: Option<Command>,
	pub is_help: bool,
}

pub fn parse(args: &Vec<String>) -> Result<ParsedArgs, ParsingError> {
	let mut parsed_args = ParsedArgs::default();
	for arg in args {
		match arg.as_str() {
			"-h" | "--help" => {
				parsed_args.is_help = true;
			},
			c => {
				if !c.starts_with("-") {
					parsed_args.command = Some(arg.into());
				}
			},
		}
	}
	return Ok(parsed_args);
}
