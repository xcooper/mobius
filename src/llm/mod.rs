mod gemini;
mod internal;
mod openai;
mod tools;

use gemini::Gemini;

use crate::config::Config;
use crate::llm::openai::OpenAI;
use crate::model::Provider;
use async_trait::async_trait;
pub(self) use internal::convert_to_messages;
use std::error::Error;

#[async_trait]
pub trait LLM {
    async fn chat(
        &self,
        system_prompt: &str,
        user_prompts: Vec<&str>,
    ) -> Result<String, Box<dyn Error>>;
}

pub fn get_llm(config: &Config) -> Box<dyn LLM + '_> {
    let provider = &config.llm.provider;
    match provider {
        Provider::OpenAI => Box::new(OpenAI::new(config)),
        Provider::Gemini => Box::new(Gemini::new(config)),
    }
}
