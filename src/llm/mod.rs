mod internal;
mod tools;

use rig::client::builder::DynClientBuilder;
use rig::completion::Chat;
use rig::message::Message;
use rig::providers::gemini::completion::gemini_api_types::AdditionalParameters;

use crate::config::Config;
use crate::llm::internal::split_prompt_and_history;
use crate::llm::tools::CheckCmdExist;
use async_trait::async_trait;
use std::error::Error;

#[async_trait(?Send)]
pub trait LLM {
    async fn chat(
        &self,
        system_prompt: &str,
        user_prompts: Vec<&str>,
    ) -> Result<String, Box<dyn Error>>;

    async fn exec(
        &self,
        system_prompt: &str,
        user_prompts: Vec<&str>,
    ) -> Result<String, Box<dyn Error>>;
}

struct GenericLLM<'a> {
    config: &'a Config,
}

#[async_trait(?Send)]
impl LLM for GenericLLM<'_> {
    async fn chat(
        &self,
        system_prompt: &str,
        user_prompts: Vec<&str>,
    ) -> Result<String, Box<dyn Error>> {
        let llm = &self.config.llm;
        let api_key = llm.api_key.clone().unwrap();
        let prompt_histories = split_prompt_and_history(user_prompts);
        let last_user_prompt = prompt_histories.0.unwrap();
        let clnt_bldr = DynClientBuilder::new();
        let agent = clnt_bldr
            .agent_with_api_key_val(&llm.provider.to_rig_provider(), &llm.model, api_key)?
            .preamble(system_prompt)
            .temperature(llm.default_temperature)
            .additional_params(serde_json::to_value(AdditionalParameters::default())?)
            .build();
        let resp = agent.chat(last_user_prompt, Vec::<Message>::new()).await;
        resp.map_err(|e| Box::from(e))
    }

    async fn exec(
        &self,
        system_prompt: &str,
        user_prompts: Vec<&str>,
    ) -> Result<String, Box<dyn Error>> {
        let llm = &self.config.llm;
        let api_key = llm.api_key.clone().unwrap();
        let prompt_histories = split_prompt_and_history(user_prompts);
        let last_user_prompt = prompt_histories.0.unwrap();
        let clnt_bldr = DynClientBuilder::new();
        let agent = clnt_bldr
            .agent_with_api_key_val(&llm.provider.to_rig_provider(), &llm.model, api_key)?
            .preamble(system_prompt)
            .tool(CheckCmdExist)
            .temperature(llm.default_temperature)
            .additional_params(serde_json::to_value(AdditionalParameters::default())?)
            .build();
        let resp = agent.chat(last_user_prompt, Vec::<Message>::new()).await;
        resp.map_err(|e| Box::from(e))
    }
}

pub fn get_llm(config: &Config) -> Box<dyn LLM + '_> {
    Box::new(GenericLLM { config: config })
}
