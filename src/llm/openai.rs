use std::error::Error;

use crate::config::Config;
use crate::llm::LLM;
use crate::CommandExecutionError;
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
    ChatCompletionRequestSystemMessageContent, ChatCompletionRequestUserMessage,
    ChatCompletionRequestUserMessageContent, CreateChatCompletionRequestArgs,
};
use async_openai::Client;
use async_trait::async_trait;

pub struct OpenAI<'a> {
    config: &'a Config,
}

impl<'a> OpenAI<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    fn init_client(&self) -> Result<Client<OpenAIConfig>, CommandExecutionError> {
        if self.config.llm.api_key.is_none() {
            return Err(CommandExecutionError::from(
                "The API key of OpenAI is missing.",
            ));
        }
        let api_key = self.config.llm.api_key.as_deref().unwrap();
        let oai_cfg = OpenAIConfig::new().with_api_key(api_key);
        let client = Client::with_config(oai_cfg);
        Ok(client)
    }
}

#[async_trait]
impl LLM for OpenAI<'_> {
    async fn chat(&self, system_prompt: &str, user_prompt: &str) -> Result<String, Box<dyn Error>> {
        let mut prompts: Vec<ChatCompletionRequestMessage> = Vec::new();
        prompts.push(ChatCompletionRequestMessage::System(
            ChatCompletionRequestSystemMessage {
                content: ChatCompletionRequestSystemMessageContent::Text(system_prompt.to_string()),
                name: None,
            },
        ));
        prompts.push(ChatCompletionRequestMessage::User(
            ChatCompletionRequestUserMessage {
                content: ChatCompletionRequestUserMessageContent::Text(user_prompt.to_string()),
                name: None,
            },
        ));

        let req = CreateChatCompletionRequestArgs::default()
            .model(self.config.llm.model.clone())
            .temperature(self.config.llm.default_temperature as f32)
            .messages(prompts)
            .build()
            .unwrap();
        let client = self.init_client()?;
        let resp = client.chat().create(req).await?;
        if let Some(choice) = resp.choices.first() {
            return match &choice.message.content {
                Some(content) => Ok(content.clone()),
                None => Err(Box::new(CommandExecutionError::from(
                    "no content in response",
                ))),
            };
        }
        Err(Box::new(CommandExecutionError::from(
            "error in OpenAI response.",
        )))
    }
}
