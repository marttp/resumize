use std::sync::Arc;

use langchain_rust::embedding::openai::OpenAiEmbedder;
use langchain_rust::llm::OpenAIConfig;

#[derive(Clone)]
pub struct EmbeddedModelAccessor {
    pub model_embedded: Arc<OpenAiEmbedder<OpenAIConfig>>
}

impl EmbeddedModelAccessor {
    pub fn new(url: String, model: String) -> Arc<Self> {
        let model_embedded = create_llm_embedded(url, model);
        let embedded = EmbeddedModelAccessor {
            model_embedded: Arc::new(model_embedded),
        };
        Arc::new(embedded)
    }

}

pub fn create_llm_embedded(url: String, model: String) -> OpenAiEmbedder<OpenAIConfig> {
    OpenAiEmbedder::default()
        .with_config(
            OpenAIConfig::default()
                .with_api_base(url)
                .with_api_key("Nothing")
        )
        .with_model(model)
}