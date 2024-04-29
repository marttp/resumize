use actix_web::{dev, get, HttpResponse, Responder};
use actix_web::http::header;
use actix_web::middleware::ErrorHandlerResponse;
pub const MY_EXPERIENCE: &str = "Thanaphoom Babparn"; // Enhancement by getting data from login user

#[get("/health")]
pub async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "OK";
    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}

pub fn add_error_header<B>(mut res: dev::ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("Error"),
    );
    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}