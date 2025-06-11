use super::models::Shoutout;

#[derive(Clone)]
pub struct ActivityService {}

impl ActivityService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create_shoutout(&self, activity_name: String) -> Result<Shoutout, String> {
        Ok(Shoutout {
            message: format!("Let's get started with {}!", activity_name),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ACTIVITY_SERVICE: ActivityService = ActivityService {};

    #[tokio::test]
    async fn test_create_activity() {
        const GIVEN_ACTIVITY: &str = "Rust";
        const EXPECTED_MESSAGE: &str = "Let's get started with Rust!";

        let actual_shoutout = ACTIVITY_SERVICE
            .create_shoutout(GIVEN_ACTIVITY.to_string())
            .await
            .unwrap();

        assert_eq!(actual_shoutout.message, EXPECTED_MESSAGE);
    }
}
