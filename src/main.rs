mod config;

use crate::config::Config;
use actix_web::{App, HttpServer};
use color_eyre::Result;

#[actix_rt::main]
async fn main() -> Result<()> {
    let config = Config::from_env().expect("server configuration");

    HttpServer::new(move || App::new())
        .bind(format!("{}:{}", &config.host, &config.port))?
        .run()
        .await?;

    Ok(())
}
