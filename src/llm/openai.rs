use std::error::Error;

use crate::config::Config;
use crate::llm::internal::split_prompt_and_history;
use crate::llm::tools::CheckCmdExist;
use crate::llm::LLM;
use async_trait::async_trait;
use rig::client::CompletionClient;

use rig::completion::{Chat, Prompt};
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
        let prompt_histories = split_prompt_and_history(user_prompts);
        let last_user_prompt = prompt_histories.0.unwrap();
        let agent = client
            .agent(&model)
            .temperature(llm.default_temperature)
            .preamble(system_prompt)
            .build();
        let resp = agent.chat(last_user_prompt, Vec::new()).await;
        resp.map_err(|e| Box::from(e))
    }

    async fn exec(
        &self,
        system_prompt: &str,
        user_prompts: Vec<&str>,
    ) -> Result<String, Box<dyn Error>> {
        let llm = &self.config.llm;
        let api_key = llm.api_key.as_ref().unwrap();
        let model = llm.model.clone();
        let client = Client::new(&api_key);
        let prompt_histories = split_prompt_and_history(user_prompts);
        let last_user_prompt = prompt_histories.0.unwrap();
        let agent = client
            .agent(&model)
            .tool(CheckCmdExist)
            .preamble(system_prompt)
            .temperature(llm.default_temperature)
            .build();
        let resp = agent.prompt(last_user_prompt).multi_turn(25).await;
        resp.map_err(|e| Box::from(e))
    }
}
