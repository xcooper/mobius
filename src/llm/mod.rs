mod gemini;
mod openai;
mod tools;
mod internal;

use gemini::Gemini;

use crate::config::Config;
use crate::llm::openai::OpenAI;
use crate::model::Provider;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;
pub(self) use internal::convert_to_messages;

#[async_trait]
pub trait LLM {
    async fn chat(
        &self,
        system_prompt: &str,
        user_prompts: Vec<&str>,
    ) -> Result<String, Box<dyn Error>>;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: Role,
    content: String,
}

pub fn get_llm(config: &Config) -> Box<dyn LLM + '_> {
    let provider = &config.llm.provider;
    match provider {
        Provider::OpenAI => Box::new(OpenAI::new(config)),
        Provider::Gemini => Box::new(Gemini::new(config)),
    }
}
