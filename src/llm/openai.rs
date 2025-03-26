use std::error::Error;

use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
    ChatCompletionRequestSystemMessageContent, ChatCompletionRequestUserMessage,
    ChatCompletionRequestUserMessageContent, CreateChatCompletionRequestArgs,
};
use async_openai::Client;

use crate::config::Config;
use crate::llm::LLM;
use crate::CommandExecutionError;

pub struct OpenAI<'a> {
    client: Client<OpenAIConfig>,
    config: &'a Config,
}

impl<'a> OpenAI<'a> {
    pub fn new(config: &'a Config) -> Self {
        let mut open_ai = Self {
            client: Client::new(),
            config,
        };
        let api_key = config.llm.api_key.as_deref().unwrap();
        let oai_cfg = OpenAIConfig::new().with_api_key(api_key);
        open_ai.client = Client::with_config(oai_cfg);
        open_ai
    }
}

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
            .temperature(0.0)
            .messages(prompts)
            .build()
            .unwrap();
        let resp = self.client.chat().create(req).await;

        if let Ok(r) = resp {
            if let Some(choice) = r.choices.first() {
                return match &choice.message.content {
                    Some(content) => Ok(content.clone()),
                    None => Err(Box::new(CommandExecutionError::new(
                        "no content in response",
                    ))),
                };
            }
        }
        Err(Box::new(CommandExecutionError::new(
            "error in OpenAI response.",
        )))
    }
}
