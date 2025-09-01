use std::error::Error;

use crate::config::Config;
use crate::llm::{convert_to_messages, LLM};
use async_trait::async_trait;
use rig::client::CompletionClient;
use rig::completion::{CompletionModel, CompletionRequest};

use rig::completion::message::AssistantContent;
use rig::providers::openai::Client;

pub struct OpenAI<'a> {
    config: &'a Config,
}

impl<'a> OpenAI<'a> {
    pub fn new(config: &'a Config) -> Self {
        OpenAI { config }
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
        let client = Client::new(&api_key);
        let gpt = client.completion_model(model.as_str());
        let req = CompletionRequest {
            preamble: Some(system_prompt.to_string()),
            chat_history: convert_to_messages(user_prompts),
            documents: Vec::new(),
            tools: Vec::new(),
            temperature: Some(llm.default_temperature),
            max_tokens: None,
            additional_params: None,
        };
        let resp = gpt.completion(req);
        resp.await
            .map(|r| r.choice)
            .map(|o| o.first())
            .map(|a| match a {
                AssistantContent::Text(text) => text.text,
                _ => String::new(),
            })
            .map_err(|ce| Box::from(ce))
    }
}
