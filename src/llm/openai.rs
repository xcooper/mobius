use std::error::Error;

use super::Message;
use crate::config::Config;
use crate::llm::LLM;
use async_trait::async_trait;
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use rig::client::CompletionClient;
use rig::completion::{CompletionModel, CompletionRequest, CompletionRequestBuilder};
use rig::message::{Text, UserContent};
use rig::OneOrMany;
use serde::{Deserialize, Serialize};

use rig::providers::openai::Client as RigClient;
use rig::completion::Message as RigMessage;
use rig::completion::Message::User;
use rig::completion::message::AssistantContent;

const DEFAULT_URL: &str = "https://api.openai.com/v1";

pub struct OpenAI<'a> {
    config: &'a Config,
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
        let api_key = llm.api_key.as_ref().unwrap();
        let model = llm.model.clone();
        let client = RigClient::new(&api_key);
        let gpt = client.completion_model(model.as_str());
        let req = CompletionRequest { 
            preamble: Some(system_prompt.to_string()),
            chat_history: convert_to_messages(user_prompts),
            documents: Vec::new(),
            tools: Vec::new(),
            temperature: Some(1.0),
            max_tokens: None,
            additional_params: None,
        };
        let resp = gpt.completion(req);
        resp.await
            .map(|r| r.choice)
            .map(|o| o.first())
            .map(|a| match a {
                AssistantContent::Text(text) => text.text,
                AssistantContent::ToolCall(tool_call) => todo!(),
                AssistantContent::Reasoning(reasoning) => todo!(),
            })
            .map_err(|ce| Box::from(ce))
    }
}

fn convert_to_messages(user_prompts: Vec<&str>) -> OneOrMany<RigMessage> {
    let rig_msgs: Vec<RigMessage> = user_prompts
        .iter()
        .map(|&p| User {
            content: OneOrMany::one(
                UserContent::Text(Text { text: p.to_string() })
            ) 
        })
        .collect();
    OneOrMany::many(rig_msgs).unwrap()
}
