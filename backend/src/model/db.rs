use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use sqlx::Pool;
use sqlx::Postgres;
use sqlx::postgres::PgPoolOptions;

const PG_HOST: &str = "localhost";
const PG_ROOT_DB: &str = "postgres";
const PG_ROOT_USER: &str = "postgres";
const PG_ROOT_PASSWORD: &str = "postgres";

const PG_APP_DB: &str = "app_db";
const PG_APP_USER: &str = "app_user";
const PG_APP_PASSWORD: &str = "app_password";
const PG_MAX_CONNECTIONS: u32 = 5;

const SQL_DIR: &str = "sql/";
const SQL_RECREATE: &str = "sql/00-recreate-db.sql";

pub async fn init_db() -> Result<Pool<Postgres>, sqlx::Error> {
    // -- Create the db with PG_ROOT (dev only)
    {
        let root_db: Pool<Postgres> = new_db_pool(PG_HOST, PG_ROOT_DB, PG_ROOT_USER, PG_ROOT_PASSWORD, 1).await?;
        pexec(&root_db, SQL_RECREATE).await?;
    }

    // -- Run the app sql files
    let app_db: Pool<Postgres> = new_db_pool(PG_HOST, PG_APP_DB, PG_APP_USER, PG_APP_PASSWORD, PG_MAX_CONNECTIONS).await?;
    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .into_iter()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    paths.sort();

    // Execute each SQL files
    for path in paths {
        if let Some(path) = path.to_str() {
            // only .sql and not the recreate
            if path.ends_with(".sql") && path != SQL_RECREATE {
                pexec(&app_db, &path).await?;
            }
        }
    }

    return new_db_pool(PG_HOST, PG_APP_DB, PG_APP_USER, PG_APP_PASSWORD, PG_MAX_CONNECTIONS).await;
}

async fn pexec(db: &Pool<Postgres>, file: &str) -> Result<(), sqlx::Error> {
    let content: String = fs::read_to_string(file).map_err(|exception| {
        println!("Error reading {} (cause: {:?})", file, exception);
        return exception;
    })?;
    
    let sql_statements: Vec<&str> = content.split(";").collect();

    for sql_statement in sql_statements {
        match sqlx::query(&sql_statement).execute(db).await {
            Ok(_) => (),
            Err(exception) => println!("WARNING - pexec - SQL file '{}' FAILED cause: {}", file, exception)
        }
    }

    return Ok(());
}

async fn new_db_pool(host: &str, db: &str, user: &str, password: &str, max_connections: u32)
    -> Result<Pool<Postgres>, sqlx::Error> {
    let connection_string = format!("postgres://{}:{}@{}/{}", user, password, host, db);
    return PgPoolOptions::new()
        .max_connections(max_connections)
        .connect_timeout(Duration::from_millis(500))
        .connect(&connection_string)
        .await;
}

#[cfg(test)]
#[path ="../_tests/model_db.rs"]
mod tests;
