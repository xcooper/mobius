use std::error::Error;

use super::Message;
use crate::config::Config;
use crate::llm::LLM;
use async_trait::async_trait;
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const DEFAULT_URL: &str = "https://api.openai.com/v1";

pub struct OpenAI<'a> {
    config: &'a Config,
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIRequest {
    model: Option<String>,
    input: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIResponse {
    id: Option<String>,
    #[serde(with = "ts_milliseconds")]
    created_at: DateTime<Utc>,
    status: Option<String>,
    error: Option<OpenAIErrorResponse>,
    output: Option<Vec<OpenAIOutput>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIOutput {
    id: String,
    status: String,
    role: String,
    content: Vec<OpenAIResponseContent>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIResponseContent {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIErrorResponse {
    code: String,
    message: String,
}

impl<'a> OpenAI<'a> {
    pub fn new(config: &'a Config) -> Self {
        OpenAI {
            config,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LLM for OpenAI<'_> {
    async fn chat(
        &self,
        system_prompt: &str,
        user_prompts: Vec<&str>,
    ) -> Result<String, Box<dyn Error>> {
        let llm = &self.config.llm;
        let default_url = &DEFAULT_URL.to_string();
        let url = llm.url.as_ref().unwrap_or(default_url);
        let api_key = llm.api_key.as_ref().unwrap();
        let model = llm.model.clone();
        let input = concat_msgs(system_prompt, user_prompts);
        let req = self
            .client
            .post(format!("{url}/responses"))
            .bearer_auth(api_key)
            .header("Content-Type", "application/json")
            .json(&OpenAIRequest {
                model: Some(model),
                input: input,
            })
            .build()?;
        let resp: OpenAIResponse = self.client.execute(req).await?.json().await?;
        Ok(resp.output.unwrap()[0].content[0].text.clone())
    }
}

fn concat_msgs(sys_prompt: &str, user_prompts: Vec<&str>) -> Vec<Message> {
    let mut msgs: Vec<Message> = Vec::new();
    msgs.push(Message {
        role: super::Role::System,
        content: sys_prompt.clone().to_string(),
    });
    for user_prompt in user_prompts {
        msgs.push(Message {
            role: super::Role::User,
            content: user_prompt.clone().to_string(),
        });
    }
    return msgs;
}
