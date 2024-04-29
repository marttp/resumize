use std::sync::Arc;

use langchain_rust::llm::{OpenAI, OpenAIConfig};

#[derive(Clone)]
pub struct ModelAccessor {
    pub model: Arc<OpenAI<OpenAIConfig>>
}

impl ModelAccessor {
    pub fn new(url: String, model: String) -> Arc<Self> {
        let model = create_llm(url, model);
        let embedded = ModelAccessor {
            model: Arc::new(model),
        };
        Arc::new(embedded)
    }
}

pub fn create_llm(url: String, model: String) -> OpenAI<OpenAIConfig> {
    OpenAI::default()
        .with_config(
            OpenAIConfig::default()
                .with_api_base(url)
                .with_api_key("Nothing")
        )
        .with_model(model)
}