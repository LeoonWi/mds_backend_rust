use dotenv::dotenv;
use std::{env, error::Error};

pub struct Config {
    pub db_url: String,
    pub ip: String,
    pub port: String,
}

impl Config {
    pub fn build() -> Result<Config, Box<dyn Error>> {
        dotenv().ok();

        let database_url;
        {
            let user = env::var("POSTGRES_USER")?;
            let password = env::var("POSTGRES_PASSWORD")?;
            let host = env::var("POSTGRES_HOST")?;
            let port = env::var("POSTGRES_PORT")?;
            let db = env::var("POSTGRES_DB")?;
            let args = match env::var("POSTGRES_ARGS") {
                Ok(string) => "?".to_string() + &string,
                Err(_) => "".to_string(),
            };

            database_url = format!(
                "postgres://{}:{}/{}:{}/{}{}",
                user, password, host, port, db, args
            );
        }

        let ip = env::var("SERVER_HOST")?;
        let port = env::var("SERVER_PORT")?;

        Ok(Config {
            ip: ip,
            db_url: database_url,
            port: port,
        })
    }
}
