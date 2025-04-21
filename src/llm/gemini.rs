use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::LLM;
use crate::config::Config;

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

#[derive(Serialize, Deserialize)]
struct Part {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct Content {
    parts: Vec<Part>,
    role: String,
}

#[derive(Serialize)]
struct GeminiReq {
    contents: Vec<Content>,
    systemInstruction: Content,
}

#[derive(Deserialize)]
struct Candidate {
    content: Content,
}

#[derive(Deserialize)]
struct GeminiResp {
    candidates: Vec<Candidate>,
}

#[async_trait]
impl LLM for Gemini<'_> {
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
