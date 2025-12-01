use actix_web::web;
use crate::handlers::todo_handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/todos")
            .route("", web::get().to(todo_handlers::get_todos))
            .route("", web::post().to(todo_handlers::create_todo))
            .route("/{id}", web::get().to(todo_handlers::get_todo))
            .route("/{id}", web::put().to(todo_handlers::update_todo))
            .route("/{id}", web::delete().to(todo_handlers::delete_todo))
            .route("/{id}/toggle", web::patch().to(todo_handlers::toggle_todo))
    );
}