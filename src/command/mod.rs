use crate::args_parser::{Commands, ParedArgs};
use crate::config::{default_config, save_config, Provider};
use log::error;
use std::env;

pub fn do_init(args: &ParedArgs) -> Result<(), ()> {
    let cmd = &args.command;
    if let Commands::Init {
        api_key,
        model,
        provider,
    } = cmd
    {
        let mut new_config = default_config();
        new_config.llm.provider = Provider::from(provider);
        new_config.llm.model = model.clone();
        if let Some(key) = api_key {
            new_config.llm.api_key = Some(key.clone());
        } else if let Ok(key) = env::var("OPENAI_API_KEY") {
            new_config.llm.api_key = Some(key);
        } else {
            error!("No API key found. Please provide one using --api-key or set OPENAI_API_KEY environment variable.");
            return Err(());
        }

        if let Err(e) = save_config(&new_config) {
            error!("Failed to save config: {}", e);
            return Err(());
        }
    }
    Ok(())
}
