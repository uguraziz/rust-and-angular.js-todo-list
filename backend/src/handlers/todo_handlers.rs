use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;
use crate::models::todo::{CreateTodo, UpdateTodo};
use crate::db::database;

pub async fn get_todos(pool: web::Data<SqlitePool>) -> impl Responder {
    match database::get_all_todos(pool.get_ref()).await {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(e) => {
            log::error!("Error fetching todos: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch todos"
            }))
        }
    }
}

pub async fn get_todo(pool: web::Data<SqlitePool>, id: web::Path<i64>) -> impl Responder {
    match database::get_todo_by_id(pool.get_ref(), *id).await {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Todo not found"
        })),
        Err(e) => {
            log::error!("Error fetching todo: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch todo"
            }))
        }
    }
}

pub async fn create_todo(
    pool: web::Data<SqlitePool>,
    todo: web::Json<CreateTodo>
) -> impl Responder {
    match database::create_todo(pool.get_ref(), todo.into_inner()).await {
        Ok(created_todo) => HttpResponse::Created().json(created_todo),
        Err(e) => {
            log::error!("Error creating todo: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create todo"
            }))
        }
    }
}

pub async fn update_todo(
    pool: web::Data<SqlitePool>,
    id: web::Path<i64>,
    todo: web::Json<UpdateTodo>
) -> impl Responder {
    match database::update_todo(pool.get_ref(), *id, todo.into_inner()).await {
        Ok(Some(updated_todo)) => HttpResponse::Ok().json(updated_todo),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Todo not found"
        })),
        Err(e) => {
            log::error!("Error updating todo: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update todo"
            }))
        }
    }
}

pub async fn delete_todo(pool: web::Data<SqlitePool>, id: web::Path<i64>) -> impl Responder {
    match database::delete_todo(pool.get_ref(), *id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Todo not found"
        })),
        Err(e) => {
            log::error!("Error deleting todo: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete todo"
            }))
        }
    }
}

pub async fn toggle_todo(pool: web::Data<SqlitePool>, id: web::Path<i64>) -> impl Responder {
    match database::toggle_todo(pool.get_ref(), *id).await {
        Ok(Some(updated_todo)) => HttpResponse::Ok().json(updated_todo),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Todo not found"
        })),
        Err(e) => {
            log::error!("Error toggling todo: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to toggle todo"
            }))
        }
    }
}