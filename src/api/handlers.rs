use std::str::FromStr;

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use crate::solana::client::{
    add_todo_on_solana, fetch_task_by_id, update_todo_on_solana, delete_todo_on_solana,
};

// Request structure for creating a new task
#[derive(Serialize, Deserialize)]
struct TodoRequest {
    text: String, // Task description
}

// Request structure for updating a task's completion status
#[derive(Serialize, Deserialize)]
struct UpdateTodoRequest {
    is_done: bool, // New completion status (true/false)
}

// Request structure for fetching tasks by user wallet (Not used in this code)
#[derive(Serialize, Deserialize)]
struct FetchTodosRequest {
    user_wallet: String, // Public key of the user's wallet
}

// Response structure for returning task details
#[derive(Serialize, Deserialize)]
struct TodoResponse {
    id: String,        // Unique task ID (Solana account public key)
    text: String,      // Task description
    is_done: bool,     // Completion status
    created_at: i64,   // Creation timestamp
    updated_at: i64,   // Last update timestamp
}

// Configures the API routes under `/api/todos`
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/todos")
            .route("", web::post().to(add_todo))        // Create a new task
            .route("/{id}", web::get().to(get_task_by_id)) // Fetch task by ID
            .route("/{id}", web::put().to(update_todo)) // Update task status
            .route("/{id}", web::delete().to(delete_todo)) // Delete a task
    );
}

// Handler for creating a new task
async fn add_todo(todo: web::Json<TodoRequest>) -> impl Responder {
    match add_todo_on_solana(&todo.text).await {
        Ok((sig, task_id)) => HttpResponse::Ok().json(TodoResponse {
            id: task_id, 
            text: todo.text.clone(),
            is_done: false, // New tasks are initially marked as not done
            created_at: chrono::Utc::now().timestamp(), // Current timestamp
            updated_at: chrono::Utc::now().timestamp(), // Same as creation initially
        }),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

// Handler for fetching a specific task by its ID
pub async fn get_task_by_id(task_id: web::Path<String>) -> impl Responder {
    match fetch_task_by_id(&task_id.into_inner()).await {
        Ok(task) => HttpResponse::Ok().json(task), // Return the task details
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

// Handler for updating a task's completion status
async fn update_todo(
    id: web::Path<String>,
    update: web::Json<UpdateTodoRequest>,
) -> impl Responder {
    // Validate task ID format
    let task_id = match Pubkey::from_str(&id) {
        Ok(pk) => pk,
        Err(_) => return HttpResponse::BadRequest().json("Invalid task ID"),
    };

    match update_todo_on_solana(&task_id.to_string(), update.is_done).await {
        Ok(sig) => HttpResponse::Ok().json(format!("Todo updated, tx: {}", sig)), // Return transaction ID
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

// Handler for deleting a task
async fn delete_todo(id: web::Path<String>) -> impl Responder {
    // Validate task ID format
    let task_id = match Pubkey::from_str(&id) {
        Ok(pk) => pk,
        Err(_) => return HttpResponse::BadRequest().json("Invalid task ID"),
    };

    match delete_todo_on_solana(&task_id.to_string()).await {
        Ok(sig) => HttpResponse::Ok().json(format!("Todo deleted, tx: {}", sig)), // Return transaction ID
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}
