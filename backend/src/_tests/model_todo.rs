use sqlx::Pool;
use sqlx::Postgres;

use crate::model::db::init_db;
use crate::model;
use crate::security::UserCtx;
use crate::security::utx_from_token;

use super::Todo;
use super::TodoMac;
use super::TodoPatch;
use super::TodoStatus;

#[tokio::test]
async fn model_todo_create() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db: Pool<Postgres> = init_db().await?;
    let utx: UserCtx = utx_from_token("123").await?;
    let data_fx: TodoPatch = TodoPatch {
        title: Some("test - model_todo_create 1".to_string()),
        ..Default::default()
    };

    // -- ACTION
    let todo_created: Todo = TodoMac::create(&db, &utx, data_fx.clone()).await?;

    // -- CHECK
    println!("\n\n->>{:?}", todo_created);
    assert!(todo_created.id >= 1000, "Id should be >= 1000");
    assert_eq!(data_fx.title.unwrap(), todo_created.title);
    assert_eq!(TodoStatus::Open, todo_created.status);

    return Ok(());
}

#[tokio::test]
async fn model_todo_get_ok() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db: Pool<Postgres> = init_db().await?;
    let utx: UserCtx = utx_from_token("123").await?;
    
    // -- ACTION
    let todo: Todo = TodoMac::get(&db, &utx, 100).await?;

    // --CHECK
    assert_eq!(100, todo.id);
    assert_eq!("todo 100", todo.title);
    assert_eq!(TodoStatus::Close, todo.status);

    return Ok(());
}

#[tokio::test]
async fn model_todo_get_wrong() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db: Pool<Postgres> = init_db().await?;
    let utx: UserCtx = utx_from_token("123").await?;
    let wrong_id: i64 = 999;
    
    // -- ACTION
    let todo: Result<Todo, model::Error> = TodoMac::get(&db, &utx, wrong_id).await;

    // --CHECK
    match todo {
        Ok(_) => assert!(false, "Should not succeed"),
        Err(model::Error::EntityNotFound(typ, id)) => {
            assert_eq!("todo", typ);
            assert_eq!(wrong_id.to_string(), id);
        },
        other_error => assert!(false, "Wrong Error {:?}", other_error)
    }

    return Ok(());
}

#[tokio::test]
async fn model_todo_update() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db: Pool<Postgres> = init_db().await?;
    let utx: UserCtx = utx_from_token("123").await?;
    let data_fx: TodoPatch = TodoPatch {
        title: Some("test - model_todo_update_ok 1".to_string()),
        ..Default::default()
    };
    let todo_fx: Todo = TodoMac::create(&db, &utx, data_fx.clone()).await?;
    let update_data_fx: TodoPatch = TodoPatch {
        title: Some("test - model_todo_update_ok 2".to_string()),
        ..Default::default()
    };

    // -- ACTION
    let todo: Todo = TodoMac::update(&db, &utx, todo_fx.id, update_data_fx.clone()).await?;

    // -- CHECK
    let todos: Vec<Todo> = TodoMac::list(&db, &utx).await?;
    assert_eq!(3, todos.len());
    assert_eq!(todo_fx.id, todo.id);
    assert_eq!(update_data_fx.title.unwrap(), todo.title);

    return Ok(());
}

#[tokio::test]
async fn model_todo_delete_simple() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db: Pool<Postgres> = init_db().await?;
    let utx: UserCtx = utx_from_token("123").await?;
    
    // -- ACTION
    let todo: Todo = TodoMac::delete(&db, &utx, 100).await?;

    // --CHECK
    assert_eq!(100, todo.id);
    assert_eq!("todo 100", todo.title);

    // --CHECK - list
    let todos: Vec<Todo> = sqlb::select().table("todo").fetch_all(&db).await?;
    assert_eq!(1, todos.len());

    return Ok(());
}

#[tokio::test]
async fn model_todo_list() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db: Pool<Postgres> = init_db().await?;
    let utx: UserCtx = utx_from_token("123").await?;

    // -- ACTION
    let todos: Vec<Todo> = TodoMac::list(&db, &utx).await?;

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
