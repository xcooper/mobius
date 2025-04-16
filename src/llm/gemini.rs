use std::collections::HashMap;

use async_openai::config;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::LLM;
use crate::config::Config;

const ENDPOINT: &str = "https://generativelanguage.googleapis.com/";

pub struct Gemini<'a> {
    config: &'a Config,
    client: Client,
}

impl<'a> Gemini<'a> {
    pub fn new(config: &'a Config) -> Self {
        Gemini {
            config,
            client: reqwest::Client::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {}

#[derive(Debug, Serialize)]
struct GeminiRequest {}

impl<'a> LLM for Gemini<'a> {
    async fn chat(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
		let key = self.config.llm.api_key.as_ref().unwrap();
		let model = &self.config.llm.model;
		let req = GeminiRequest {};
        let resp: GeminiResponse = self
            .client
            .post(format!("{}/v1beta/models/{}:generateContent?key={}", ENDPOINT, model, key))
			.header("Content-Type", "application/json")
			.json(&req)
            .send()
            .await?
            .json()
            .await?;
        return Ok(String::new());
    }
}
