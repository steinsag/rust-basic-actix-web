use super::dto::{CreateActivityRequest, CreateActivityResponse};
use crate::domain::activities::service::ActivityService;
use crate::services::services::AppServices;

use actix_web::{web, HttpResponse, Responder};
use validator::Validate;

pub struct ActivityHandler {
    activity_service: ActivityService,
}

impl ActivityHandler {
    pub fn new(app_services: web::Data<AppServices>) -> Self {
        Self {
            activity_service: app_services.activity_service.clone(),
        }
    }

    pub async fn create_activity(
        handler: web::Data<Self>,
        activity_json: web::Json<CreateActivityRequest>,
    ) -> impl Responder {
        if activity_json.validate().is_err() {
            return HttpResponse::BadRequest().finish();
        }

        match handler
            .activity_service
            .create_shoutout(activity_json.activity.clone())
            .await
        {
            Ok(shoutout) => HttpResponse::Ok().json(CreateActivityResponse::from(shoutout)),
            Err(e) => HttpResponse::InternalServerError().body(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configure_app;
    use actix_web::http::StatusCode;
    use actix_web::{test, App};

    const EXPECTED_ERROR_MESSAGE: &'static str = "Activity must be between 3 and 20 characters";

    #[actix_web::test]
    async fn test_create_activity_validation() {
        let test_cases = vec![
            ("ab", EXPECTED_ERROR_MESSAGE),
            ("abcdefghijklmnopqrstuvwxyz", EXPECTED_ERROR_MESSAGE),
            ("", EXPECTED_ERROR_MESSAGE),
        ];

        let app = test::init_service(App::new().configure(configure_app(None))).await;

        for (given_activity, expected_error) in test_cases {
            let given_request = test::TestRequest::post()
                .uri("/api/activities")
                .set_json(&CreateActivityRequest {
                    activity: given_activity.to_string(),
                })
                .to_request();

            let actual_response = test::call_service(&app, given_request).await;

            assert_eq!(
                actual_response.status(),
                StatusCode::BAD_REQUEST,
                "Activity '{}' should fail validation with error: {}",
                given_activity,
                expected_error
            );
        }
    }
}
