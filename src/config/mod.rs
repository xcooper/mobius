pub enum Provider {
    OpenAi,
}

pub struct LLM {
    pub provider: Provider,
    pub model: String,
    pub api_key: Option<String>,
    pub url: Option<String>,
    pub temperature: f32,
}

pub struct Config {
    pub llm: LLM,
}

pub fn load_config() -> Config {
    todo!()
}

pub fn save_config() {
    todo!()
}
