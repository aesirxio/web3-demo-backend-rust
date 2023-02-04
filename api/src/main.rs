#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate core;

pub mod repository;

mod config;
mod errors;
pub mod handlers;
mod helpers;
mod routes;
mod server;
mod time;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let db = repository::mongodb_repo::MongoRepo::init().await?;

    server::server(db).await.map_err(anyhow::Error::from)
}
