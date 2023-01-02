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
}

pub struct TodoMac;

impl TodoMac {
    pub async fn list(db: &Pool<Postgres>) -> Result<Vec<Todo>, model::Error>{
        let sql_statement: &str = "select id, cid, title from todo order by id desc";

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
