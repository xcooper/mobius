mod internal;
mod tools;

use crate::model::Provider;
use rig::client::CompletionClient;
use rig::completion::Chat;
use rig::message::Message;
use rig::providers::{gemini, openai};

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
        let resp = match llm.provider {
            Provider::OpenAI => {
                let client: openai::Client = openai::Client::new(&api_key)?;
                client
                    .agent(&llm.model)
                    .preamble(system_prompt)
                    .tool(CheckCmdExist)
                    .temperature(llm.default_temperature)
                    .additional_params(serde_json::json!({}))
                    .build()
                    .chat(last_user_prompt, Vec::<Message>::new())
                    .await
            }
            Provider::Gemini => {
                let client: gemini::Client = gemini::Client::new(&api_key)?;
                client
                    .agent(&llm.model)
                    .preamble(system_prompt)
                    .tool(CheckCmdExist)
                    .temperature(llm.default_temperature)
                    .additional_params(serde_json::json!({}))
                    .build()
                    .chat(last_user_prompt, Vec::<Message>::new())
                    .await
            }
        };
        resp.map_err(Box::from)
    }
}

pub fn get_llm(config: &Config) -> Box<dyn LLM + '_> {
    Box::new(GenericLLM { config })
}
