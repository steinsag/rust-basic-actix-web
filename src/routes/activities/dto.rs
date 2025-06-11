use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub(super) struct CreateActivityRequest {
    #[validate(length(
        min = 3,
        max = 20,
        message = "Activity must be between 3 and 20 characters"
    ))]
    pub activity: String,
}

#[derive(Serialize, Deserialize)]
pub(super) struct CreateActivityResponse {
    pub message: String,
}

impl From<crate::domain::activities::models::Shoutout> for CreateActivityResponse {
    fn from(shoutout: crate::domain::activities::models::Shoutout) -> Self {
        Self {
            message: shoutout.message,
        }
    }
}
