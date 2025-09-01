use async_trait::async_trait;
use rig::{
    client::CompletionClient,
    completion::{CompletionModel, CompletionRequest},
    message::AssistantContent,
    providers::gemini::Client,
};

use crate::{
    config::Config,
    llm::{convert_to_messages, LLM},
};

pub struct Gemini<'a> {
    config: &'a Config,
}

impl<'a> Gemini<'a> {
    pub fn new(config: &'a Config) -> Self {
        Gemini { config }
    }
}

#[async_trait]
impl LLM for Gemini<'_> {
    async fn chat(
        &self,
        system_prompt: &str,
        user_prompts: Vec<&str>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let llm = &self.config.llm;
        let api_key = llm.api_key.clone().unwrap();
        let model = llm.model.clone();
        let client = Client::new(&api_key);
        let gemini = client.completion_model(&model);
        let req = CompletionRequest {
            preamble: Some(system_prompt.to_string()),
            chat_history: convert_to_messages(user_prompts),
            documents: Vec::new(),
            tools: Vec::new(),
            temperature: Some(llm.default_temperature),
            max_tokens: None,
            additional_params: None,
        };
        let resp = gemini.completion(req);
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
