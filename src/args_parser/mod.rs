use std::{error::Error, fmt::Display};
use sub_parsers::parse_init_cmd_args;

mod sub_parsers;

#[derive(Debug)]
pub struct ParsingError {
	pub message: String
}

impl ParsingError {
	fn new(message: String) -> ParsingError {
		return ParsingError {
			message: message,
		}
	}
}

impl Display for ParsingError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "parse args error: {}", self.message)
	}
}

impl Error for ParsingError {}

pub enum Command {
	Invalid,
	Init,
}

impl From<&String> for Command {
	fn from(value: &String) -> Self {
		match value.to_lowercase().as_str() {
			"init" => Command::Init,
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
	let arg = &args[0];
	match arg.as_str() {
		"-h" | "--help" => {
			parsed_args.is_help = true;
		},
		_ => {
			parsed_args.command = Some(arg.into());
		},
	}
	return if let Some(cmd) = &parsed_args.command {
		match cmd {
			Command::Invalid => {
				Err(ParsingError::new(format!("'{}' is not a valid command", arg)))
			},
			Command::Init => {
				parse_init_cmd_args(&mut parsed_args)
			},
		}
	} else {
		panic!("must have a command")
	}
}
