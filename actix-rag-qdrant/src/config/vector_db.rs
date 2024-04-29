use qdrant_client::client::QdrantClient;
use std::sync::Arc;
use crate::config::app_config::AppConfig;

#[derive(Clone)]
pub struct QdrantDb {
    pub client: Arc<QdrantClient>,
}

pub fn create_vector_client(setting: Arc<AppConfig>) -> QdrantClient {
    let url = setting.vector_db.url.clone();
    QdrantClient::from_url(&url).build().unwrap()
}

impl QdrantDb {
    pub fn new(client: QdrantClient) -> Arc<Self> {
        let db = QdrantDb {
            client: Arc::new(client),
        };
        Arc::new(db)
    }
}