use crate::config::CONFIG;
use anyhow::Result;

use mongodb::options::{ClientOptions, ConnectionString};
use mongodb::{Client, Database};

#[derive(Debug)]
pub struct MongoRepo {
    pub db: Database,
}

impl MongoRepo {
    pub async fn init() -> Result<Self> {
        let conn_string = ConnectionString::parse(&CONFIG.mongo.mongouri).unwrap();
        let mut client_options = ClientOptions::parse_connection_string(conn_string).await?;

        client_options.max_pool_size = CONFIG.mongo.max_pool_size;

        Ok(MongoRepo {
            db: Client::with_options(client_options)
                .unwrap()
                .database(&CONFIG.mongo.dbname),
        })
    }
}
