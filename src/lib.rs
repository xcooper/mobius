pub mod args_parser;
pub mod command;
pub mod config;

macro_rules! echo {
    ($arg:expr) => {
        println!("{}", $arg);
    };
}
