//! Spin up a HTTPServer

use crate::config::CONFIG;
use crate::routes::routes;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use listenfd::ListenFd;

use crate::repository::mongodb_repo::MongoRepo;
use crate::time::UtcDateTime;

pub async fn server(db: MongoRepo) -> std::io::Result<()> {
    let db_data = Data::new(db);
    let utc_date_time = Data::new(UtcDateTime::default());

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(db_data.clone())
            .app_data(utc_date_time.clone())
            .configure(routes)
    });

    if let Some(threads) = &CONFIG.web.threads {
        server = server.workers(threads.clone());
    }

    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind(&CONFIG.web.server)?
    };

    server.run().await
}
