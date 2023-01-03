use sqlx::Pool;
use sqlx::Postgres;
use sqlx::query::QueryAs;
use sqlx::postgres::PgArguments;

use crate::model;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Todo {
    pub id: i64,
    pub cid: i64,
    pub title: String,
    pub status: TodoStatus
}

#[derive(Default, Debug, Clone)]
pub struct TodoPatch {
    pub cid: Option<i64>,
    pub title: Option<String>,
    pub status: Option<TodoStatus>
}

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(type_name="todo_status_enum")]
#[sqlx(rename_all="lowercase")]
pub enum TodoStatus {
    Open,
    Close
}

pub struct TodoMac;

impl TodoMac {
    pub async fn create(db: &Pool<Postgres>, data: TodoPatch) -> Result<Todo, model::Error> {
        let sql_statement = "insert into todo(cid, title) values($1, $2) returning id, cid, title, status";
        let query: QueryAs<Postgres, Todo, PgArguments> = sqlx::query_as(&sql_statement)
            .bind(123 as i64)
            .bind(data.title.unwrap_or_else(|| "untitled".to_string()));
        
        let todo: Todo = query.fetch_one(db).await?;
        
        return Ok(todo);
    }

    pub async fn list(db: &Pool<Postgres>) -> Result<Vec<Todo>, model::Error>{
        let sql_statement: &str = "select id, cid, title, status from todo order by id desc";

        // Build the sqlx-query
        let query: QueryAs<Postgres, Todo, PgArguments> = sqlx::query_as(&sql_statement);
        // Execute the query
        let todos: Vec<Todo> = query.fetch_all(db).await?;

        return Ok(todos);
    }
}

#[cfg(test)]
#[path ="../_tests/model_todo.rs"]
mod tests;
