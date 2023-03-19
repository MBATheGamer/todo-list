use sqlx::Pool;
use sqlx::Postgres;
use warp::Filter;

use std::path::Path;
use std::sync::Arc;

pub async fn start_web(web_folder: &str, web_port: u16, db: Arc<Pool<Postgres>>) -> Result<(), Error> {
    if !Path::new(web_folder).exists() {
        return Err(Error::FailStartWebFolderNotFound(web_folder.to_string()));
    }

    let content = warp::fs::dir(web_folder.to_string());
    let root_index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", web_folder)));
    let static_site = content.or(root_index);

    let routes = static_site;

    println!("Start 127.0.0.1:{} at {}", web_port, web_folder);
    warp::serve(routes).run(([127, 0, 0, 1], web_port)).await;

    return Ok(());
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Web server failed to start because web-folder '{0}' not found!")]
    FailStartWebFolderNotFound(String)
}
