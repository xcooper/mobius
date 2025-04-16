
use async_openai::config;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::LLM;
use crate::config::Config;

pub struct Gemini<'a> {
    config: &'a Config,
}

impl<'a> Gemini<'a> {
    pub fn new(config: &'a Config) -> Self {
        Gemini {
            config,
        }
    }
}

impl<'a> LLM for Gemini<'a> {
    async fn chat(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
		let key = self.config.llm.api_key.as_ref().unwrap();
		let model = &self.config.llm.model;
        return Ok(String::new());
    }
}
