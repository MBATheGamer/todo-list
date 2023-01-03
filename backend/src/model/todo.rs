use sqlb::HasFields;
use sqlx::Pool;
use sqlx::Postgres;

use crate::model;
use crate::security::UserCtx;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Todo {
    pub id: i64,
    pub cid: i64,
    pub title: String,
    pub status: TodoStatus
}

#[derive(sqlb::Fields, Default, Debug, Clone)]
pub struct TodoPatch {
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
sqlb::bindable!(TodoStatus);

pub struct TodoMac;

impl TodoMac {
    pub async fn create(db: &Pool<Postgres>, utx: &UserCtx, data: TodoPatch) -> Result<Todo, model::Error> {
        let mut fields = data.fields();
        fields.push(("cid", 123).into());
        let sql_builder = sqlb::insert()
            .table("todo")
            .data(fields)
            .returning(&["id", "cid", "title", "status"]);

        let todo: Todo = sql_builder.fetch_one(db).await?;
        
        return Ok(todo);
    }

    pub async fn list(db: &Pool<Postgres>, utx: &UserCtx) -> Result<Vec<Todo>, model::Error>{
        let sql_builder = sqlb::select()
            .table("todo")
            .columns(&["id", "cid", "title", "status"])
            .order_by("!id");

        // Execute the query
        let todos: Vec<Todo> = sql_builder.fetch_all(db).await?;

        return Ok(todos);
    }
}

#[cfg(test)]
#[path ="../_tests/model_todo.rs"]
mod tests;
