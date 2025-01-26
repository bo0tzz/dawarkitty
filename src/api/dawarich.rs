use dawarich::apis::points_api::{ApiV1PointsGetError, api_v1_points_get};
use dawarich::apis::{Error, configuration};
use dawarich::models::ApiV1PointsGet200ResponseInner;
use std::ops::Sub;

pub struct DawarichApi {
    configuration: configuration::Configuration,
}

impl DawarichApi {
    pub fn connect(host: &str, api_key: &str) -> Self {
        let mut cfg = configuration::Configuration::new();
        cfg.base_path = host.to_owned();
        cfg.api_key = Some(configuration::ApiKey {
            prefix: None,
            key: api_key.to_owned(),
        });

        DawarichApi { configuration: cfg }
    }

    pub async fn get_recent_points(
        &self,
    ) -> Result<Vec<ApiV1PointsGet200ResponseInner>, Error<ApiV1PointsGetError>> {
        let yesterday = chrono::Local::now().sub(chrono::Duration::days(1));
        let start_at = &yesterday.format("%Y-%m-%d").to_string();
        let points = api_v1_points_get(
            &self.configuration,
            &self.configuration.api_key.as_ref().unwrap().key,
            Some(start_at),
            None,
            None,
            None,
            None, // Ignore pagination and ordering
        )
        .await;

        points
    }
}
