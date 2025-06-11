use crate::domain::activities::service::ActivityService;

pub struct AppServices {
    pub activity_service: ActivityService,
}

impl AppServices {
    pub fn new(activity_service: ActivityService) -> Self {
        Self { activity_service }
    }
}
