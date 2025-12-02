mod models;
mod db;
mod handlers;
mod routes;

use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./data/todos.db".to_string());
    
    if !database_url.contains("?mode=") {
        database_url.push_str("?mode=rwc");
    }
    
    let host = env::var("HOST")
        .unwrap_or_else(|_| "127.0.0.1".to_string());
    
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    log::info!("=== Backend Starting ===");
    log::info!("DATABASE_URL from env: {:?}", env::var("DATABASE_URL"));
    log::info!("Connecting to database: {}", database_url);

    let pool = db::database::create_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    log::info!("Initializing database...");
    db::database::init_db(&pool)
        .await
        .expect("Failed to initialize database");

    log::info!("Database initialized successfully");

    let server_address = format!("{}:{}", host, port);
    log::info!("Starting server at http://{}", server_address);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(|| async {
                actix_web::HttpResponse::Ok().json(serde_json::json!({
                    "message": "Rust Todo API",
                    "version": "1.0.0",
                    "endpoints": {
                        "health": "GET /health"
                    }
                }))
            }))
            .route("/health", web::get().to(|| async { actix_web::HttpResponse::Ok().json(serde_json::json!({"status": "ok"})) }))
            .configure(routes::todo_routes::config)
    })
    .bind(&server_address)?
    .run()
    .await
}