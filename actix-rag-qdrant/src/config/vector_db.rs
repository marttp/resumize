use qdrant_client::client::QdrantClient;
use std::sync::Arc;
use log::info;
use qdrant_client::prelude::{CreateCollection, Distance};
use qdrant_client::qdrant::{VectorParams, vectors_config, VectorsConfig};
use crate::config::app_config::AppConfig;

#[derive(Clone)]
pub struct QdrantDb {
    pub client: Arc<QdrantClient>,
}

pub fn create_vector_client(setting: AppConfig) -> QdrantClient {
    let url = setting.vector_db.url.clone();
    QdrantClient::from_url(&url).build().unwrap()
}

pub async fn create_collection(qdrant_client: &Arc<QdrantDb>, collection_name: String) -> Result<(), ()> {
    let result = qdrant_client.client.delete_collection(collection_name.clone()).await;
    match result {
        Ok(_) => {
            info!("Deleted collection: {} successful", collection_name.clone());
        }
        Err(_) => {}
    }
    qdrant_client.client
        .create_collection(&CreateCollection {
            collection_name: collection_name.clone(),
            vectors_config: Some(VectorsConfig {
                config: Some(vectors_config::Config::Params(VectorParams {
                    size: 4096,
                    distance: Distance::Cosine.into(),
                    ..Default::default()
                })),
            }),
            ..Default::default()
        })
        .await
        .unwrap();
    Ok(())
}

impl QdrantDb {
    pub fn new(client: QdrantClient) -> Arc<Self> {
        let db = QdrantDb {
            client: Arc::new(client),
        };
        Arc::new(db)
    }
}