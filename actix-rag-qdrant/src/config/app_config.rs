use std::sync::Arc;

use config::Config;
use crate::config::vector_db::QdrantDb;
use crate::llm::llama_embedded::EmbeddedModelAccessor;
use crate::llm::llama_model::ModelAccessor;

#[derive(Clone)]
pub struct AppState {
    pub app_name: String,
    pub qdrant_client: Arc<QdrantDb>,
    pub llm_model: Arc<ModelAccessor>,
    pub llm_embedding_model: Arc<EmbeddedModelAccessor>,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server: Server,
    pub vector_db: QdrantConfig,
    pub llm: Llm,
}

impl AppConfig {
    pub fn new() -> Arc<Self> {
        let settings = Config::builder()
            .add_source(config::File::with_name("./Setting.toml"))
            .build()
            .unwrap();

        Arc::new(AppConfig {
            server: Server {
                name: settings.get_string("server.name").unwrap(),
                port: settings.get_int("server.port").unwrap() as u16,
            },
            vector_db: QdrantConfig {
                url: settings.get_string("db.url").unwrap(),
            },
            llm: Llm {
                model_url: settings.get_string("llm.model_url").unwrap(),
                model: settings.get_string("llm.model").unwrap(),
                embedding_model_url: settings.get_string("llm.embedding_model_url").unwrap(),
                embedding_model: settings.get_string("llm.embedding_model").unwrap(),
            },
        })
    }
}

#[derive(Debug, Clone)]
pub struct QdrantConfig {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct Server {
    pub name: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct Llm {
    pub model_url: String,
    pub model: String,
    pub embedding_model_url: String,
    pub embedding_model: String,
}
