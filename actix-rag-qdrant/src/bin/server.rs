use actix_cors::Cors;
use actix_web::{App, http::StatusCode, HttpServer, middleware::Logger, web};
use actix_web::middleware::ErrorHandlers;
use env_logger::Env;
use actix_rag_qdrant::common::{add_error_header, health_checker_handler};
use actix_rag_qdrant::experience::router::{generate_resume, save_experience};

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
            .service(generate_resume)
            .service(save_experience)
            .service(health_checker_handler)
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