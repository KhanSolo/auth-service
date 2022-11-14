use color_eyre::Result;
use dotenv::dotenv;
use eyre::WrapErr;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
}

impl Config {
    pub fn from_env() -> Result<Config> {
        dotenv().ok();
        let mut c = config::Config::new();
        let from_env = config::Environment::default();
        c.merge(from_env)?;

        c.try_into().context("loading config from environment")
    }
}
