use self::handler::ActivityHandler;
use crate::services::services::AppServices;

use actix_web::web;
use actix_web::web::ServiceConfig;

mod dto;
mod handler;

pub fn configure(cfg: &mut ServiceConfig, services: web::Data<AppServices>) {
    let activity_handler = web::Data::new(ActivityHandler::new(services));

    cfg.service(
        web::scope("/activities")
            .app_data(activity_handler)
            .route("", web::post().to(ActivityHandler::create_activity)),
    );
}
