use std::{env, sync::Arc};

use web::start_web;

mod model;
mod security;
mod web;

const DEFAULT_WEB_FOLDER: &'static str = "web-folder/";
const DEFAULT_WEB_PORT: u16 = 8080;

#[tokio::main]
async fn main() {
    let mut args: Vec<String> = env::args().collect();
    let web_folder = args.pop().unwrap_or_else(|| DEFAULT_WEB_FOLDER.to_string());
    let web_port = DEFAULT_WEB_PORT;

    let db = model::init_db().await.expect("Cannot init db");
    let db = Arc::new(db);

    match start_web(&web_folder, web_port, db).await {
        Ok(_) => println!("Server ended"),
        Err(exception) => println!("ERROR - web server failed to start. Cause {:?}", exception)    
    }
}
