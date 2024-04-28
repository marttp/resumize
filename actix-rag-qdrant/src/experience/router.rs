use std::io::Read;

use actix_multipart::form::{
    MultipartForm,
    tempfile::TempFile,
};
use actix_web::{Error, HttpResponse, post, Responder, web};
use futures::{future::ok, stream::once};
use log::info;

#[post("/experiences/resume")]
pub async fn generate_resume(text_payload: web::Json<String>) -> impl Responder {
    // TODO: Add communicate to LLM
    println!("Received text payload with size {}", text_payload.len());
    HttpResponse::Ok().body(text_payload.clone())
}

#[post("/experiences/self")]
pub async fn save_experience(MultipartForm(form): MultipartForm<UploadForm>) -> Result<impl Responder, Error> {
    let mut buffer = String::new();
    for mut f in form.file {
        let file_name = f.file_name.unwrap();
        info!("New file upload: {}", file_name);
        f.file.read_to_string(&mut buffer)?;
    }
    // This line has been manual test that file able to received
    // TODO: Transform to vector by embedded model and put to Qdrant
    // let body = once(ok::<_, Error>(web::Bytes::from(buffer.clone().into_bytes())));
    // Ok(HttpResponse::Ok().content_type("application/json").streaming(body))
    Ok(HttpResponse::Ok().body("Your experience has been uploaded!"))
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    file: Vec<TempFile>,
}