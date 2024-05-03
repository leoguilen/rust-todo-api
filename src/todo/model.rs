use chrono::{NaiveDateTime, Utc};
use serde::{
    Deserialize,
    Serialize
};
use sqlx::postgres::PgRow;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TodoStatus {
    Pending = 0,
    Doing = 1,
    Done = 2
}

impl TodoStatus {
    fn from_i16(row: i16) -> Option<Self> {
        match row {
            0 => Some(Self::Pending),
            1 => Some(Self::Doing),
            2 => Some(Self::Done),
            _ => None
        }
    }
}

impl Copy for TodoStatus {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TodoStatus,
    pub finished_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime 
}

impl sqlx::FromRow<'_, PgRow> for Todo {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: sqlx::Row::try_get(row, "id")?,
            title: sqlx::Row::try_get(row, "title")?,
            description: sqlx::Row::try_get(row, "description")?,
            status: TodoStatus::from_i16(sqlx::Row::try_get(row, "status").unwrap()).unwrap(),
            finished_at: sqlx::Row::try_get(row, "finished_at")?,
            created_at: sqlx::Row::try_get(row, "created_at")?
        })
    }
}

impl Todo {
    pub fn new(title: String, description: Option<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title,
            description,
            status: TodoStatus::Pending,
            finished_at: None,
            created_at: Utc::now().naive_utc()
        }
    }

    pub fn update(&mut self, title: String, description: Option<String>, status: Option<TodoStatus>) {
        self.title = title;
        self.description = description.or_else(|| self.description.clone());
        if let Some(status) = status {
            self.status = status;
            self.finished_at = match status {
                TodoStatus::Done => Some(Utc::now().naive_utc()),
                _ => None
            };
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TodoRequest {
    pub title: String,
    pub description: Option<String>,
    pub status: Option<TodoStatus>
}

impl TodoRequest {
    pub fn into_todo(self) -> Todo {
        Todo::new(self.title, self.description)
    }
}

#[derive(Debug, Serialize)]
pub struct TodoResponse {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TodoStatus,
    pub finished_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime
}

impl TodoResponse {
    pub fn from_todo(todo: &Todo) -> Self {
        Self {
            id: todo.id,
            title: todo.title.clone(),
            description: todo.description.clone(),
            status: todo.status,
            finished_at: todo.finished_at,
            created_at: todo.created_at
        }
    }
}