use sqlx::Pool;
use sqlx::Postgres;

use crate::model::db::init_db;
use crate::model::todo::Todo;
use crate::model::todo::TodoMac;

#[tokio::test]
async fn model_todo_list() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db: Pool<Postgres> = init_db().await?;

    // -- ACTION
    let todos: Vec<Todo> = TodoMac::list(&db).await?;

    // -- CHECK
    assert_eq!(2, todos.len());
    // todo 101
    assert_eq!(101, todos[0].id);
    assert_eq!(123, todos[0].cid);
    assert_eq!("todo 101", todos[0].title);
    // todo 100
    assert_eq!(100, todos[1].id);
    assert_eq!(123, todos[1].cid);
    assert_eq!("todo 100", todos[1].title);
    
    
    Ok(())
}
