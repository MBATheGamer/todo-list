use sqlb::DeleteSqlBuilder;
use sqlb::Field;
use sqlb::HasFields;
use sqlb::InsertSqlBuilder;
use sqlb::Raw;
use sqlb::SelectSqlBuilder;
use sqlb::UpdateSqlBuilder;
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
    const TABLE: &'static str = "todo";
    const COLUMNS: &'static [&'static str] = &["id", "cid", "title", "status"];
}

impl TodoMac {
    pub async fn create(db: &Pool<Postgres>, _utx: &UserCtx, data: TodoPatch) -> Result<Todo, model::Error> {
        let mut fields: Vec<Field> = data.fields();
        fields.push(("cid", 123).into());
        let sql_builder: InsertSqlBuilder = sqlb::insert()
            .table(Self::TABLE)
            .data(fields)
            .returning(Self::COLUMNS);

        let todo: Todo = sql_builder.fetch_one(db).await?;
        
        return Ok(todo);
    }

    pub async fn get(db: &Pool<Postgres>, _utx: &UserCtx, id: i64) -> Result<Todo, model::Error> {
        let sql_builder: SelectSqlBuilder = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::COLUMNS)
            .and_where_eq("id", id);

        let todo: Result<Todo, sqlx::Error> = sql_builder.fetch_one(db).await;

        return handle_fetch_one_result(todo, Self::TABLE, id);
    }

    pub async fn update(db: &Pool<Postgres>, utx: &UserCtx, id: i64, data: TodoPatch) -> Result<Todo, model::Error> {
        let mut fields: Vec<Field> = data.fields();
        fields.push(("mid", utx.user_id).into());
        fields.push(("ctime", Raw("now()")).into());
        let sql_builder: UpdateSqlBuilder = sqlb::update()
            .table(Self::TABLE)
            .data(fields)
            .and_where_eq("id", id)
            .returning(Self::COLUMNS);

        let todo: Result<Todo, sqlx::Error> = sql_builder.fetch_one(db).await;

        return handle_fetch_one_result(todo, Self::TABLE, id);
    }

    pub async fn delete(db: &Pool<Postgres>, _utx: &UserCtx, id: i64) -> Result<Todo, model::Error> {
        let sql_builder: DeleteSqlBuilder = sqlb::delete()
            .table(Self::TABLE)
            .returning(Self::COLUMNS)
            .and_where_eq("id", id);

        let todo: Result<Todo, sqlx::Error> = sql_builder.fetch_one(db).await;

        return handle_fetch_one_result(todo, Self::TABLE, id);
    }

    pub async fn list(db: &Pool<Postgres>, _utx: &UserCtx) -> Result<Vec<Todo>, model::Error>{
        let sql_builder: SelectSqlBuilder = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::COLUMNS)
            .order_by("!id");

        // Execute the query
        let todos: Vec<Todo> = sql_builder.fetch_all(db).await?;

        return Ok(todos);
    }
}

fn handle_fetch_one_result(result: Result<Todo, sqlx::Error>, typ: &'static str, id: i64)
    -> Result<Todo, model::Error> {
    return result.map_err(|sqlx_error| match sqlx_error {
        sqlx::Error::RowNotFound => model::Error::EntityNotFound(typ, id.to_string()),
        other => model::Error::SqlxError(other)
    });
}

#[cfg(test)]
#[path ="../_tests/model_todo.rs"]
mod tests;
