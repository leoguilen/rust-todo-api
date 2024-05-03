use sqlx::{Pool, Postgres};

use super::model::Todo;

const GET_ALL_QUERY: &str = "SELECT * FROM todos";
const GET_BY_ID_QUERY: &str = "SELECT * FROM todos WHERE id = $1";
const INSERT_COMMAND: &str = "INSERT INTO todos (id, title, description, status, finished_at, created_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *";
const UPDATE_COMMAND: &str = "UPDATE todos SET title = $1, description = $2, status = $3, finished_at = $4 WHERE id = $5 RETURNING *";
const DELETE_COMMAND: &str = "DELETE FROM todos WHERE id = $1";

pub struct Repository {
    pool: Pool<Postgres>,
}

impl Repository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Todo>, sqlx::Error> {
        match sqlx::query_as(&GET_ALL_QUERY)
            .fetch_all(&self.pool)
            .await
        {
            Ok(rows) => Ok(rows),
            Err(err) => Err(err),
        }
    }

    pub async fn get_by_id(&self, id: uuid::Uuid) -> Result<Todo, sqlx::Error> {
        match sqlx::query_as(&GET_BY_ID_QUERY)
            .bind(id)
            .fetch_one(&self.pool)
            .await
        {
            Ok(row) => Ok(row),
            Err(err) => Err(err),
        }
    }
    
    pub async fn create(&self, todo: Todo) -> Result<Todo, sqlx::Error> {
        match sqlx::query_as(&INSERT_COMMAND)
            .bind(todo.id)
            .bind(todo.title)
            .bind(todo.description)
            .bind(todo.status as i16)
            .bind(todo.finished_at)
            .bind(todo.created_at)
            .fetch_one(&self.pool)
            .await
        {
            Ok(row) => Ok(row),
            Err(err) => Err(err),
        }
    }
    
    pub async fn update(&self, id: uuid::Uuid, todo: Todo) -> Result<Todo, sqlx::Error> {
        match sqlx::query_as(&UPDATE_COMMAND)
            .bind(todo.title)
            .bind(todo.description)
            .bind(todo.status as i16)
            .bind(todo.finished_at)
            .bind(id)
            .fetch_one(&self.pool)
            .await
        {
            Ok(row) => Ok(row),
            Err(err) => Err(err),
        }
    }
    
    pub async fn delete(&self, id: uuid::Uuid) -> Result<(), sqlx::Error> {
        match sqlx::query(&DELETE_COMMAND)
            .bind(id)
            .execute(&self.pool)
            .await
        {
            Ok(res) => match res.rows_affected() {
                0 => Err(sqlx::Error::RowNotFound),
                _ => Ok(()),
            },
            Err(err) => Err(err),
        }
    }
}