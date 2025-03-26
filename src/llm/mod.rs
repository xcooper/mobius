mod openai;

use crate::config::{Config, Provider};
use crate::llm::openai::OpenAI;
use std::error::Error;

pub trait LLM {
    async fn chat(&self, system_prompt: &str, user_prompt: &str) -> Result<String, Box<dyn Error>>;
}

pub fn get_llm<'a>(config: &'a Config) -> impl LLM + 'a {
    let provider = &config.llm.provider;
    match provider {
        Provider::OpenAI => OpenAI::new(config),
    }
}
