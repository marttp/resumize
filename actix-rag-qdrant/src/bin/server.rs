use actix_cors::Cors;
use actix_web::{App, http::StatusCode, HttpServer, middleware::Logger, web};
use actix_web::middleware::ErrorHandlers;
use env_logger::Env;

use actix_rag_qdrant::common::{add_error_header, health_checker_handler};
use actix_rag_qdrant::config::app_config::{AppConfig, AppState};
use actix_rag_qdrant::config::vector_db::{create_vector_client, QdrantDb};
use actix_rag_qdrant::experience::router::{generate_resume, save_experience};
use actix_rag_qdrant::llm::llama_embedded::EmbeddedModelAccessor;
use actix_rag_qdrant::llm::llama_model::ModelAccessor;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let app_config = AppConfig::new();

    let qdrant_client = QdrantDb::new(create_vector_client(app_config.clone()));
    let llm_model = ModelAccessor::new(
        app_config.llm.model_url.clone(),
        app_config.llm.model.clone()
    );
    let llm_embedding_model = EmbeddedModelAccessor::new(
        app_config.llm.embedding_model_url.clone(),
        app_config.llm.embedding_model.clone()
    );

    let app_state = AppState {
        app_name: app_config.server.name.clone(),
        qdrant_client,
        llm_model,
        llm_embedding_model,
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
        .bind(("127.0.0.1", app_config.server.port))?
        .run()
        .await
}