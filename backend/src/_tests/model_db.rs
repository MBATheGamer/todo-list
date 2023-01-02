use sqlx::{Pool, Postgres, postgres::PgRow};

use super::init_db;

#[tokio::test]
async fn model_db_init_db() -> Result<(), Box<dyn std::error::Error>> {
    // ACTION
    let db: Pool<Postgres> = init_db().await?;
    
    // CHECK
    let result: Vec<PgRow> = sqlx::query("select * from todo").fetch_all(&db).await?;
    assert_eq!(2, result.len(), "number of seed todos");

    return Ok(());
}
