use chrono::serde::ts_seconds;
use chrono::{DateTime, Local, TimeDelta};
use geojson::Value::Point;
use geojson::{Feature, Geometry, JsonObject};
use log::{debug, error, trace};
use reqwest::header;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;
use std::convert::Into;
use std::error::Error;

#[derive(Clone)]
pub struct TractiveApi {
    email: String,
    password: String,
    pub auth: Option<AuthTokenResponse>,
    client: reqwest::Client,
}

#[derive(Deserialize, Clone)]
pub struct AuthTokenResponse {
    user_id: String,
    client_id: String,
    #[serde(with = "ts_seconds")]
    pub expires_at: DateTime<chrono::Utc>,
    access_token: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Tracker {
    pub _id: String,
    pub _type: String,
    pub _version: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Position {
    #[serde(with = "ts_seconds")]
    pub time: DateTime<chrono::Utc>,
    pub latlong: [f64; 2],
    pub alt: f64,
    pub speed: Option<f64>,
    pub course: Option<f64>,
    pub pos_uncertainty: f64,
    pub sensor_used: String,
}

const TRACTIVE_API_URL: &str = "https://graph.tractive.com/4";
const TRACTIVE_CLIENT_ID: &str = "5728aa1fc9077f7c32000186";
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:134.0) Gecko/20100101 Firefox/134.0";

const AUTH_RENEW_THRESHOLD: TimeDelta = TimeDelta::hours(24);

impl TractiveApi {
    pub async fn connect(email: &str, password: &str) -> Self {
        debug!("Connecting to Tractive API");

        let cb = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .build();

        let client = match cb {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to create tractive client: {}", e);
                panic!("Source: {}", e.source().unwrap());
            },
        };

        let state = TractiveApi {
            email: email.to_string(),
            password: password.to_string(),
            auth: None,
            client,
        };

        state
    }

    async fn auth_headers(&mut self) -> HeaderMap {
        self.check_auth().await;
        let a = self.auth.as_ref().unwrap();

        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", a.access_token)).unwrap(),
        );
        headers.insert(
            "X-Tractive-Client",
            HeaderValue::from_str(&a.client_id).unwrap(),
        );
        headers.insert(
            "X-Tractive-User",
            HeaderValue::from_str(&a.user_id).unwrap(),
        );

        headers
    }

    pub async fn check_auth(&mut self) {
        match self.auth.as_ref() {
            Some(auth) => {
                trace!("Checking auth token expiry");
                let now = chrono::Utc::now();
                let expiry = auth.expires_at;
                let diff = expiry - now;
                trace!("Token expires in: {} hours", diff.num_hours());
                if diff < AUTH_RENEW_THRESHOLD {
                    log::info!("Auth token close to expiry, re-authenticating");
                    self.authenticate().await;
                }
            }
            None => {
                log::info!("No auth token, authenticating");
                self.authenticate().await;
            }
        }
    }

    async fn authenticate(&mut self) {
        let url = format!("{}/auth/token", TRACTIVE_API_URL);

        let body = &serde_json::json!({
            "platform_email": self.email,
            "platform_token": self.password,
            "grant_type": "tractive"
        });

        let response = self
            .client
            .post(url)
            .header("X-Tractive-Client", TRACTIVE_CLIENT_ID)
            .json(body)
            .send()
            .await;

        match response {
            Ok(body) => {
                let body_text = body.text().await.unwrap();

                let auth: AuthTokenResponse = serde_json::from_str(&body_text).unwrap();
                debug!(
                    "Authenticated with Tractive, token expires at: {}",
                    auth.expires_at
                );
                self.auth = Some(auth);
            }
            Err(e) => {
                error!("Failed to authenticate with Tractive: {}", e);
                panic!("Source: {}", e.source().unwrap());
            }
        }
    }

    pub async fn get_trackers(&mut self) -> Vec<Tracker> {
        debug!("Getting trackers");

        let url = format!("{}/user/me/trackers", TRACTIVE_API_URL);
        let auth = self.auth_headers().await;

        let response = self.client.get(url).headers(auth).send().await;

        trace!("Got response: {:?}", response);

        match response {
            Ok(body) => {
                let body_text = body.text().await.unwrap();
                trace!("Got response: {:?}", body_text);

                let trackers: Vec<Tracker> = serde_json::from_str(&body_text).unwrap();
                trace!("Got trackers: {:?}", trackers);

                trackers
            }
            Err(e) => {
                panic!("Failed to get trackers from Tractive: {}", e);
            }
        }
    }

    pub async fn get_positions(
        &mut self,
        tracker: Tracker,
        from: DateTime<Local>,
        to: DateTime<Local>,
    ) -> Vec<Position> {
        debug!(
            "Getting positions for tracker {} from {} to {}",
            tracker._id, from, to
        );

        let url = format!("{}/tracker/{}/positions", TRACTIVE_API_URL, tracker._id);
        let auth = self.auth_headers().await;

        let response = self
            .client
            .get(url)
            .query(&[("time_from", from.timestamp()), ("time_to", to.timestamp())])
            .query(&[("format", "json_segments")])
            .headers(auth)
            .send()
            .await;

        match response {
            Ok(body) => {
                let body_text = body.text().await.unwrap();
                trace!("Got response: {:?}", body_text);

                let positions: Vec<Vec<Position>> = serde_json::from_str(&body_text).unwrap();
                trace!("Got positions: {:?}", positions);

                if positions.len() != 1 {
                    panic!("Expected 1 segment, got {}", positions.len());
                }

                positions[0].clone()
            }
            Err(e) => {
                panic!("Failed to get positions from Tractive: {}", e);
            }
        }
    }
}

impl Into<Feature> for &Position {
    fn into(self) -> Feature {
        let geometry = Geometry {
            bbox: None,
            value: Point(vec![self.latlong[1].into(), self.latlong[0].into()]),
            foreign_members: None,
        };

        //TODO: Derive Deserialize on the Position struct?
        let mut properties = JsonObject::new();
        properties.insert("timestamp".to_string(), self.time.to_rfc3339().into());
        properties.insert("altitude".to_string(), self.alt.into());
        properties.insert("speed".to_string(), self.speed.into());
        properties.insert("course".to_string(), self.course.into());
        properties.insert(
            "horizontal_accuracy".to_string(),
            self.pos_uncertainty.into(),
        );
        properties.insert("sensor_used".to_string(), self.sensor_used.clone().into());

        Feature {
            bbox: None,
            geometry: Some(geometry),
            id: None,
            properties: Some(properties),
            foreign_members: None,
        }
    }
}
