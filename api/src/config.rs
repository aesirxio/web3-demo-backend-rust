//! Inject dotenv and env variables into the Config struct
//!
//! The envy crate injects environment variables into a struct.
//!
//! dotenv allows environment variables to be augmented/overwriten by a
//! .env file.
//!
//! This file throws the Config struct into a CONFIG lazy_static to avoid
//! multiple processing.

use dotenv::dotenv;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct LoadConfig {
    pub rust_backtrace: Option<u8>,
    pub rust_log: Option<String>,
    pub http_port: String,
    pub dbuser: String,
    pub dbpass: String,
    pub dbhost: Option<String>,
    pub dbport: Option<String>,
    pub dbname: String,
    pub test_dbname: Option<String>,
    pub threads: Option<usize>,
    pub max_pool_size: Option<u32>,
}

pub struct MongoConfig {
    pub max_pool_size: Option<u32>,
    pub dbname: String,
    pub mongouri: String,
}

pub struct WebConfig {
    pub threads: Option<usize>,
    pub server: String,
}

pub struct RustConfig {
    pub rust_backtrace: u8,
    pub rust_log: String,
}

pub struct Config {
    pub mongo: MongoConfig,
    pub web: WebConfig,
    pub rust: RustConfig,
}

// Throw the Config struct into a CONFIG lazy_static to avoid multiple processing
lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

/// Use envy to inject dotenv and env vars into the Config struct
fn get_config() -> Config {
    dotenv().ok();

    match envy::from_env::<LoadConfig>() {
        Ok(load_config) => {
            let dbname: String;
            if cfg!(test) {
                // Use different database for tests
                dbname = load_config.test_dbname.unwrap_or("testdb".to_string());
            } else {
                dbname = load_config.dbname;
            }

            Config {
                mongo: MongoConfig {
                    max_pool_size: load_config.max_pool_size,
                    dbname,
                    mongouri: [
                        "mongodb://",
                        &load_config.dbuser,
                        ":",
                        &load_config.dbpass,
                        "@",
                        &load_config.dbhost.unwrap_or("localhost".to_string()),
                        ":",
                        &load_config.dbport.unwrap_or("27017".to_string()),
                        "/?retryWrites=true&w=majority&authSource=admin",
                    ]
                    .join(""),
                },
                web: WebConfig {
                    threads: load_config.threads,
                    server: "0.0.0.0:".to_string() + &load_config.http_port,
                },
                rust: RustConfig {
                    rust_backtrace: load_config.rust_backtrace.unwrap_or(0),
                    rust_log: load_config.rust_log.unwrap_or("error".to_string()),
                },
            }
        }
        Err(error) => panic!("Configuration Error: {:#?}", error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_a_config() {
        let config = get_config();
        assert_ne!(config.web.server, "".to_string());
    }

    #[test]
    fn it_gets_a_config_from_the_lazy_static() {
        let config = &CONFIG;
        assert_ne!(config.web.server, "".to_string());
    }
}
