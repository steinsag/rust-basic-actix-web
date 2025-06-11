use actix_web::web;
use actix_web::web::ServiceConfig;

pub mod domain;
pub mod routes;
pub mod services;

use crate::domain::activities::service::ActivityService;
use services::services::AppServices;

pub fn configure_app(
    custom_services: Option<web::Data<AppServices>>,
) -> impl FnOnce(&mut ServiceConfig) + Clone {
    move |cfg: &mut ServiceConfig| {
        let services = custom_services.unwrap_or_else(get_app_services);

        cfg.app_data(services.clone())
            .configure(|c| routes::configure_routes(c, services.clone()));
    }
}

fn get_app_services() -> web::Data<AppServices> {
    web::Data::new(AppServices::new(ActivityService::new()))
}
