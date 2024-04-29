use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;

use actix_multipart::form::{
    MultipartForm,
    tempfile::TempFile,
};
use actix_web::{Error, HttpResponse, post, Responder, web};
use actix_web::web::Data;
use futures::{future::ok, stream::once};
use futures_util::StreamExt;
use langchain_rust::document_loaders::{Loader, TextLoader};
use langchain_rust::embedding::Embedder;
use langchain_rust::embedding::openai::OpenAiEmbedder;
use langchain_rust::schemas::Document;
use langchain_rust::vectorstore::qdrant::{Store, StoreBuilder};
use langchain_rust::vectorstore::{VecStoreOptions, VectorStore};
use log::info;
use qdrant_client::client::QdrantClient;
use qdrant_client::prelude::SearchPoints;
use qdrant_client::qdrant::{Condition, Filter, PointsOperationResponse, PointStruct, SearchResponse, Vectors};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::{Timestamp, Uuid};
use crate::common::MY_EXPERIENCE;
use crate::config::app_config::AppState;
use crate::config::vector_db::create_vector_client;
use crate::llm::llama_embedded::create_llm_embedded;

#[post("/experiences/resume")]
pub async fn generate_resume(text_payload: String, app_state: Data<AppState>) -> Result<impl Responder, Error> {
    // Connect to Qdrant
    /*
        Cannot use because langchain-rust is not up-to-date with compatible payload
        let store = get_vector_store(app_state, true).await;
        let results = store
            .similarity_search(&text_payload, 10, &VecStoreOptions::default())
            .await
            .unwrap();
    */
    let mut buffer = String::from(text_payload.clone());
    let embedded_response = embedding(&app_state, &mut buffer).await;
    let documents = retreive_relevant_documents(&app_state, embedded_response).await;
    let body = once(ok::<_, Error>(web::Bytes::from(documents.clone().into_bytes())));
    Ok(HttpResponse::Ok().content_type("text/plain").streaming(body))
}

async fn retreive_relevant_documents(app_state: &Data<AppState>, embedded_response: Vec<Vec<f64>>) -> String {
    let client = app_state.qdrant_client.client.clone();
    let flat_vector_result = embedded_response.iter()
        .flat_map(|v| v.clone().iter().map(|x| *x as f32).collect::<Vec<f32>>())
        .collect::<Vec<f32>>();

    let results = client.search_points(&SearchPoints {
        collection_name: app_state.collection.clone(),
        vector: flat_vector_result.clone(),
        limit: 100,
        with_payload: Some(true.into()),
        filter: Some(Filter::any([
            Condition::matches("metadata.owner", MY_EXPERIENCE.clone().to_string()),
        ])),
        ..Default::default()
    })
        .await
        .unwrap();

    let test_result = results.result.clone().into_iter()
        .map(|scored_point| scored_point.payload)
        .collect::<Vec<_>>();
    dbg!(test_result.clone());

    let documents = results
        .result
        .into_iter()
        .map(|scored_point| {
            scored_point.payload.get("page_content")
                .and_then(|doc| doc.as_str())
                .map(|doc| doc.to_string())
                .unwrap_or_default()
        })
        .collect::<Vec<_>>()
        .join("\n");
    documents
}

#[post("/experiences/self")]
pub async fn save_experience(MultipartForm(form): MultipartForm<UploadForm>, app_state: Data<AppState>) -> Result<impl Responder, Error> {
    let mut buffer = String::new();
    for mut f in form.file {
        let file_name = f.file_name.unwrap();
        info!("New file upload: {}", file_name);
        f.file.read_to_string(&mut buffer)?;
    }
    /*
      Version 1

        // This line has been manual test that file able to received
        let response = embedding(&app_state, &mut buffer).await;
        // Transform to vector by embedded model and put to Qdrant
        let point_operation_resposne = upsert_to_vector(app_state, response).await?;

        info!("Documents added with information: {:?}", point_operation_resposne);
    */

    // Note: Version 2
    let store = get_vector_store(app_state, false).await;
    let documents = split_content(buffer.clone())
        .iter()
        .map(|s| Document::new(s).with_metadata({
            let mut metadata = HashMap::new();
            metadata.insert("owner".to_string(), json!(MY_EXPERIENCE.clone()));
            metadata
        }))
        .collect::<Vec<_>>();
    store
        .add_documents(&documents, &get_vecstore_options())
        .await
        .unwrap();
    Ok(HttpResponse::Ok().body("Your experience has been uploaded!"))
}

fn get_vecstore_options() -> VecStoreOptions {
    VecStoreOptions::default()
}

async fn get_vector_store(app_state: Data<AppState>, is_filter: bool) -> Store {
    let llm_config = app_state.app_config.llm.clone();
    if is_filter {
        StoreBuilder::new()
            .embedder(create_llm_embedded(llm_config.embedding_model_url, llm_config.embedding_model))
            .client(create_vector_client(app_state.app_config.clone()))
            .collection_name(app_state.collection.clone().as_str())
            .search_filter(Filter::must([
                Condition::matches("doc", MY_EXPERIENCE.clone().to_string()),
            ]))
            .build()
            .await
            .unwrap()
    } else {
        StoreBuilder::new()
            .embedder(create_llm_embedded(llm_config.embedding_model_url, llm_config.embedding_model))
            .client(create_vector_client(app_state.app_config.clone()))
            .collection_name(app_state.collection.clone().as_str())
            .build()
            .await
            .unwrap()
    }
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    file: Vec<TempFile>,
}

#[derive(Serialize, Deserialize)]
struct ResumeData {
    professional: Vec<Value>,
    education: Vec<Value>,
}

fn split_content(json_str: String) -> Vec<String> {
    // Simple split by comma (,)
    let data: ResumeData = serde_json::from_str(&json_str.clone()).unwrap();
    let educations = data.education.iter()
        .map(|x| String::from(format!("educations:{}", x.to_string())))
        .collect::<Vec<String>>();
    let experiences = data.professional.iter()
        .map(|x| String::from(format!("experiences:{}", x.to_string())))
        .collect::<Vec<String>>();
    let documents = educations.iter().chain(experiences.iter()).cloned().collect::<Vec<String>>();
    dbg!(documents.clone());
    documents
}

async fn upsert_to_vector(app_state: Data<AppState>, response: Vec<Vec<f64>>) -> Result<PointsOperationResponse, Error> {
    let payload = json!({
        "doc": MY_EXPERIENCE,
    });
    let points = response
        .iter()
        .map(|v| {
            let id = Uuid::now_v7().to_string();
            let vector: Vec<_> = v.clone().iter().map(|x| *x as f32).collect();
            let payload = payload.clone().try_into().unwrap();
            PointStruct::new(id.clone(), Vectors::from(vector), payload)
        })
        .collect::<Vec<_>>();
    let qdrant_client = app_state.qdrant_client.client.clone();
    let operation_info = qdrant_client
        .upsert_points_blocking(app_state.collection.clone(), None, points, None)
        .await
        .unwrap();
    Ok(operation_info)
}

async fn embedding(app_state: &Data<AppState>, buffer: &mut String) -> Vec<Vec<f64>> {
    let documents = vec![buffer.clone()];
    let embedded_model = app_state.llm_embedding_model.model_embedded.clone();
    // Getting vector result from experience
    let response = embedded_model.embed_documents(&documents.clone()).await.unwrap();
    response
}
