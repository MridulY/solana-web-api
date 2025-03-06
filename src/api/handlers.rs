use std::str::FromStr;

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use crate::solana::client::{
    add_todo_on_solana, fetch_task_by_id, update_todo_on_solana, delete_todo_on_solana,
};

#[derive(Serialize, Deserialize)]
struct TodoRequest {
    text: String,  
}

#[derive(Serialize, Deserialize)]
struct UpdateTodoRequest {
    is_done: bool,  
}

#[derive(Serialize, Deserialize)]
struct FetchTodosRequest {
    user_wallet: String,  
}

#[derive(Serialize, Deserialize)]
struct TodoResponse {
    id: String,        
    text: String,
    is_done: bool,
    created_at: i64,
    updated_at: i64,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/todos")
            .route("", web::post().to(add_todo))
            .route("/{id}", web::get().to(get_task_by_id)) 
            .route("/{id}", web::put().to(update_todo))
            .route("/{id}", web::delete().to(delete_todo))
    );
}

async fn add_todo(todo: web::Json<TodoRequest>) -> impl Responder {
    match add_todo_on_solana(&todo.text).await {
        Ok((sig, task_id)) => HttpResponse::Ok().json(TodoResponse {
            id: task_id, 
            text: todo.text.clone(),
            is_done: false,
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}


pub async fn get_task_by_id(task_id: web::Path<String>) -> impl Responder {
    match fetch_task_by_id(&task_id.into_inner()).await {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

async fn update_todo(
    id: web::Path<String>,
    update: web::Json<UpdateTodoRequest>,
) -> impl Responder {
    let task_id = match Pubkey::from_str(&id) {
        Ok(pk) => pk,
        Err(_) => return HttpResponse::BadRequest().json("Invalid task ID"),
    };

    match update_todo_on_solana(&task_id.to_string(), update.is_done).await {
        Ok(sig) => HttpResponse::Ok().json(format!("Todo updated, tx: {}", sig)),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

async fn delete_todo(
    id: web::Path<String>,
) -> impl Responder {
    let task_id = match Pubkey::from_str(&id) {
        Ok(pk) => pk,
        Err(_) => return HttpResponse::BadRequest().json("Invalid task ID"),
    };

    match delete_todo_on_solana(&task_id.to_string()).await {
        Ok(sig) => HttpResponse::Ok().json(format!("Todo deleted, tx: {}", sig)),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}
