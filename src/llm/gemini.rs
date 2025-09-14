use std::error::Error;

use async_trait::async_trait;
use rig::providers::gemini::{completion::gemini_api_types::AdditionalParameters, Client};
use rig::{client::CompletionClient, completion::Chat};
use serde_json::to_value;

use crate::config::Config;
use crate::llm::{internal::split_prompt_and_history, tools::CheckCmdExist, LLM};

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
    ) -> Result<String, Box<dyn Error>> {
        let llm = &self.config.llm;
        let api_key = llm.api_key.clone().unwrap();
        let model = llm.model.clone();
        let client = Client::new(&api_key);
        let prompt_histories = split_prompt_and_history(user_prompts);
        let last_user_prompt = prompt_histories.0.unwrap();
        let agent = client
            .agent(&model)
            .preamble(system_prompt)
            .temperature(llm.default_temperature)
            .additional_params(to_value(AdditionalParameters::default()).unwrap())
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
        let api_key = llm.api_key.clone().unwrap();
        let model = llm.model.clone();
        let client = Client::new(&api_key);
        let prompt_histories = split_prompt_and_history(user_prompts);
        let last_user_prompt = prompt_histories.0.unwrap();
        let agent = client
            .agent(&model)
            .preamble(system_prompt)
            .tool(CheckCmdExist)
            .temperature(llm.default_temperature)
            .additional_params(to_value(AdditionalParameters::default()).unwrap())
            .build();
        let resp = agent.chat(last_user_prompt, Vec::new()).await;
        resp.map_err(|e| Box::from(e))
    }
}
