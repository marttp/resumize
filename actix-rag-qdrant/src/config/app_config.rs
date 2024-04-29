use std::sync::Arc;

use config::Config;

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
                port: settings.get_int("server.port").unwrap() as u16,
            },
            vector_db: QdrantConfig {
                url: settings.get_string("db.url").unwrap(),
            },
            llm: Llm {
                model: settings.get_string("llm.model").unwrap(),
                embedding_model: "llm.embedding_model".to_string(),
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
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct Llm {
    pub model: String,
    pub embedding_model: String,
}
