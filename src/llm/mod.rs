mod openai;

use crate::model::Provider;
use crate::config::Config;
use crate::llm::openai::OpenAI;
use std::error::Error;
use std::future::Future;

pub trait LLM {
    fn chat(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> impl Future<Output = Result<String, Box<dyn Error>>> + Send;
}

pub fn get_llm<'a>(config: &'a Config) -> impl LLM + 'a {
    let provider = &config.llm.provider;
    match provider {
        Provider::OpenAI => OpenAI::new(config),
    }
}

pub fn get_llm_url(provider: &Provider) -> Option<String> {
    match provider {
        Provider::OpenAI => Some(String::from("https://api.openai.com/v1")),
    }
}