use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::LLM;
use crate::config::Config;

const DEFAULT_API_URL: &str = "https://generativelanguage.googleapis.com/";

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
        let api_key = self.config.llm.api_key.as_ref().unwrap();
        let model = &self.config.llm.model;
        let def_val = &DEFAULT_API_URL.to_string();
        let api_url = self.config.llm.url.as_ref().unwrap_or(def_val);
        let req = self.client
            .post(format!("{api_url}/v1beta/models/{model}:generateContent"))
            .header("Content-Type", "application/json")
            .query(&[("key", api_key)])
            .json(&GeminiReq {
                contents: vec![Content {
                    parts: vec![Part{ text: user_prompt.to_string() }],
                    role: "user".to_string()
                }],
                systemInstruction: Content {
                    parts: vec![Part{ text: system_prompt.to_string() }],
                    role: "model".to_string()
                }
            })
            .build()?;
        let gemini_resp: GeminiResp = self.client.execute(req).await?.json().await?;
        return Ok(gemini_resp.candidates[0].content.parts[0].text.clone());
    }
}
