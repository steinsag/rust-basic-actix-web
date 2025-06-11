use crate::services::services::AppServices;
use actix_web::web::{self, ServiceConfig};

mod activities;

pub fn configure_routes(cfg: &mut ServiceConfig, services: web::Data<AppServices>) {
    cfg.service(web::scope("/api").configure(|c| activities::configure(c, services)));
}
