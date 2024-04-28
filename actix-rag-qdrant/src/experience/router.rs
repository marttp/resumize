use std::io::Read;

use actix_multipart::form::{
    MultipartForm,
    tempfile::TempFile,
};
use actix_web::{Error, HttpResponse, post, Responder};
use log::info;

#[post("/experiences/resume")]
pub async fn generate_resume() -> impl Responder {
    HttpResponse::Ok().body("Here your resume content")
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
    Ok(HttpResponse::Ok())
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    file: Vec<TempFile>,
}