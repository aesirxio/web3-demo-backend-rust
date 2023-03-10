//! Place all Actix routes here, multiple route configs can be used and
//! combined.

use crate::handlers::health::get_health;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Healthcheck
        .route("/health", web::get().to(get_health));
}
