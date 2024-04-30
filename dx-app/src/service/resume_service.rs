use std::error::Error;
use log::info;
use reqwest::multipart::Part;

pub async fn upload_file(file_name: String, contents: Vec<u8>) -> Result<(), Box<dyn Error>> {
    let part = Part::bytes(contents).file_name(file_name);
    let file = reqwest::multipart::Form::new().part("file", part);
    let response = reqwest::Client::new()
        .post("http://localhost:8000/experiences/self")
        .multipart(file)
        .send()
        .await?;
    info!("Upload response: {:?}", response);
    Ok(())
}

pub async fn get_resume_recommendation(job_title: String, job_description: String) -> Result<String, Box<dyn Error>> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "text/plain".parse()?);
    let request = reqwest::Client::new()
        .post(format!("http://localhost:8000/experiences/resume/{}", job_title))
        .headers(headers)
        .body(job_description);
    let response = request.send().await?;
    let body = response.text().await?;
    Ok(body)
}