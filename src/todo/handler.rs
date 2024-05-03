use std::sync::Arc;

use axum::{
    extract::{Path, State}, http::{HeaderValue, StatusCode}, response::IntoResponse, Json
};
use uuid::Uuid;

use super::{
    model::{TodoRequest, TodoResponse},
    repository::Repository
};

pub struct Handler {
    repo: Repository
}

impl Handler {
    pub fn new(repo: Repository) -> Self {
        Self { repo }
    }

    async fn get_all(&self) -> impl IntoResponse {
        let todos = match self.repo.get_all().await {
            Ok(todos) => todos,
            Err(err) => {
                log::error!("Failed to get todos: {}", err);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to get todos"
                ).into_response()
            }
        };
        let res: Vec<TodoResponse> = todos
            .iter()
            .map(TodoResponse::from_todo)
            .collect();
        (StatusCode::OK, Json(res)).into_response()
    }

    async fn get_by_id(&self, id: Uuid) -> impl IntoResponse {
        let todo = match self.repo.get_by_id(id).await {
            Ok(todo) => todo,
            Err(err) => {
                if let sqlx::Error::RowNotFound = err {
                    log::warn!("Todo not found: id={}", id);
                    return (StatusCode::NOT_FOUND, ()).into_response();
                }
    
                log::error!("Failed to get todo: {}", err);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to get todo"
                ).into_response()
            }
        };
        
        let res = TodoResponse::from_todo(&todo);
        (StatusCode::OK, Json(res)).into_response()
    }

    async fn create(&self, req: TodoRequest) -> impl IntoResponse {
        let todo = match self.repo.create(req.into_todo()).await {
            Ok(todo) => todo,
            Err(err) => {
                log::error!("Failed to create todo: {}", err);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create todo"
                ).into_response()
            }
        };
        let mut res = (StatusCode::CREATED, Json(TodoResponse::from_todo(&todo))).into_response();
        res.headers_mut().insert("Location", HeaderValue::from_str(&format!("/todos/{}", todo.id)).unwrap());
        res
    }

    async fn update(&self, id: Uuid, req: TodoRequest) -> impl IntoResponse {
        let mut todo = match self.repo.get_by_id(id).await {
            Ok(todo) => todo,
            Err(err) => {
                if let sqlx::Error::RowNotFound = err {
                    log::warn!("Todo not found: id={}", id);
                    return (StatusCode::NOT_FOUND, ()).into_response();
                }
    
                log::error!("Failed to update todo: {}", err);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to update todo"
                ).into_response()
            }
        };

        todo.update(req.title, req.description, req.status);

        match self.repo.update(id, todo).await {
            Ok(todo) => (StatusCode::OK, Json(TodoResponse::from_todo(&todo))).into_response(),
            Err(err) => {
                log::error!("Failed to update todo: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to update todo"
                ).into_response()
            }
        }
    }

    async fn delete(&self, id: Uuid) -> impl IntoResponse {
        match self.repo.delete(id).await {
            Ok(_) => (StatusCode::NO_CONTENT, ()).into_response(),
            Err(err) => {
                if let sqlx::Error::RowNotFound = err {
                    log::warn!("Todo not found: id={}", id);
                    return (StatusCode::NOT_FOUND, ()).into_response();
                }

                log::error!("Failed to delete todo: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to update todo"
                ).into_response()
            }
        }
    }
}

pub async fn get_all(State(handler): State<Arc<Handler>>) -> impl IntoResponse {
    handler.get_all().await
}

pub async fn get_by_id(State(handler): State<Arc<Handler>>, Path(id): Path<Uuid>) -> impl IntoResponse {
    handler.get_by_id(id).await
}

pub async fn create(State(handler): State<Arc<Handler>>, Json(req): Json<TodoRequest>) -> impl IntoResponse {
    handler.create(req).await
}

pub async fn update(State(handler): State<Arc<Handler>>, Path(id): Path<Uuid>, Json(req): Json<TodoRequest>) -> impl IntoResponse {
    handler.update(id, req).await
}

pub async fn delete(State(handler): State<Arc<Handler>>, Path(id): Path<Uuid>) -> impl IntoResponse {
    handler.delete(id).await
}

#[cfg(test)]
mod tests {
    use super::*;

    // async fn test_get_all() {
    //     // Arrange
    //     // let expected = vec![];

    //     // Act
    //     let result = get_all().await;

    //     // Assert
    //     assert!(result.is_err());
    // }
}