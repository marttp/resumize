use actix_cors::Cors;
use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web, dev, http::{header, StatusCode}, Result, middleware::Logger};
use env_logger::{Env};
use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers};

#[get("/health")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "OK";
    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn add_error_header<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("Error"),
    );
    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}

#[derive(Clone, Debug)]
struct AppState {
    app_name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let app_state = AppState {
        app_name: "resume_generate_server".to_string()
    };
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .wrap(Cors::default())
            .wrap(Logger::default())
            .wrap(
                ErrorHandlers::new()
                    .handler(StatusCode::INTERNAL_SERVER_ERROR, add_error_header),
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}