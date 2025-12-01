use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::Result;
use crate::models::todo::{Todo, CreateTodo, UpdateTodo};

pub async fn create_pool(database_url: &str) -> Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    
    Ok(pool)
}

pub async fn init_db(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT,
            completed BOOLEAN NOT NULL DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_all_todos(pool: &SqlitePool) -> Result<Vec<Todo>> {
    let todos = sqlx::query_as::<_, Todo>(
        "SELECT id, title, description, completed, created_at, updated_at FROM todos ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;

    Ok(todos)
}

pub async fn get_todo_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Todo>> {
    let todo = sqlx::query_as::<_, Todo>(
        "SELECT id, title, description, completed, created_at, updated_at FROM todos WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(todo)
}

pub async fn create_todo(pool: &SqlitePool, todo: CreateTodo) -> Result<Todo> {
    let result = sqlx::query(
        "INSERT INTO todos (title, description) VALUES (?, ?)"
    )
    .bind(&todo.title)
    .bind(&todo.description)
    .execute(pool)
    .await?;

    let id = result.last_insert_rowid();
    
    let created_todo = get_todo_by_id(pool, id).await?;
    
    Ok(created_todo.unwrap())
}

pub async fn update_todo(pool: &SqlitePool, id: i64, todo: UpdateTodo) -> Result<Option<Todo>> {
    let existing_todo = get_todo_by_id(pool, id).await?;
    
    if existing_todo.is_none() {
        return Ok(None);
    }

    let existing = existing_todo.unwrap();

    sqlx::query(
        "UPDATE todos SET title = ?, description = ?, completed = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
    )
    .bind(todo.title.unwrap_or(existing.title))
    .bind(todo.description.or(existing.description))
    .bind(todo.completed.unwrap_or(existing.completed))
    .bind(id)
    .execute(pool)
    .await?;

    get_todo_by_id(pool, id).await
}

pub async fn delete_todo(pool: &SqlitePool, id: i64) -> Result<bool> {
    let result = sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn toggle_todo(pool: &SqlitePool, id: i64) -> Result<Option<Todo>> {
    let todo = get_todo_by_id(pool, id).await?;
    
    if todo.is_none() {
        return Ok(None);
    }

    let current_status = todo.unwrap().completed;

    sqlx::query(
        "UPDATE todos SET completed = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
    )
    .bind(!current_status)
    .bind(id)
    .execute(pool)
    .await?;

    get_todo_by_id(pool, id).await
}