use std::io::Read;

use actix_multipart::form::{
    MultipartForm,
    tempfile::TempFile,
};
use actix_web::{Error, HttpResponse, post, Responder, web};
use futures::{future::ok, stream::once};
use futures_util::StreamExt;
use langchain_rust::document_loaders::{Loader, TextLoader};
use langchain_rust::embedding::Embedder;
use langchain_rust::embedding::openai::OpenAiEmbedder;
use log::info;
use crate::config::app_config::AppState;

#[post("/experiences/resume")]
pub async fn generate_resume(text_payload: String) -> impl Responder {
    // TODO: Add communicate to LLM
    println!("Received text payload with size {}", text_payload.len());
    HttpResponse::Ok().body(text_payload.clone())
}

#[post("/experiences/self")]
pub async fn save_experience(MultipartForm(form): MultipartForm<UploadForm>, app_state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let mut buffer = String::new();
    for mut f in form.file {
        let file_name = f.file_name.unwrap();
        info!("New file upload: {}", file_name);
        f.file.read_to_string(&mut buffer)?;
    }
    // This line has been manual test that file able to received
    // Transform to vector by embedded model and put to Qdrant
    // Simple split by comma (,)
    let documents = buffer.clone()
        .split(",")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let embedded_model = app_state.llm_embedding_model.model_embedded.clone();
    let response = embedded_model.embed_documents(&documents.clone()).await.unwrap();
    // TODO: Put to Qdrant
    Ok(HttpResponse::Ok().body("Your experience has been uploaded!"))
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    file: Vec<TempFile>,
}