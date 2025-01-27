use chrono::serde::ts_seconds;
use log::{debug, trace};
use reqwest::header;
use serde::Deserialize;
use std::convert::Into;
use chrono::{DateTime, Local, TimeDelta};

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
    pub expires_at: chrono::DateTime<chrono::Utc>,
    access_token: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Tracker {
    _id: String,
    _type: String,
    _version: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Position {
    pub time: i64,
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
        let mut default_headers = header::HeaderMap::new();
        default_headers.insert("X-Tractive-Client", TRACTIVE_CLIENT_ID.parse().unwrap());

        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .default_headers(default_headers)
            .build()
            .unwrap();

        let mut state = TractiveApi {
            email: email.to_string(),
            password: password.to_string(),
            auth: None,
            client,
        };

        state
    }

    pub async fn check_auth(&mut self) {
        match self.auth.as_ref() {
            Some(auth) => {
                debug!("Checking auth token expiry");
                let now = chrono::Utc::now();
                let expiry = auth.expires_at;
                let diff = expiry - now;
                trace!("Token expires in: {}", diff);
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

        let response = self.client.post(url).json(body).send().await;

        match response {
            Ok(body) => {
                let auth = body.json::<AuthTokenResponse>().await.unwrap();
                debug!(
                    "Authenticated with Tractive, token expires at: {}",
                    auth.expires_at
                );
                self.auth = Some(auth);
            }
            Err(e) => {
                panic!("Failed to authenticate with Tractive: {}", e);
            }
        }
    }

    pub async fn get_trackers(&self) -> Vec<Tracker> {
        let url = format!("{}/user/me/trackers", TRACTIVE_API_URL);

        let response = self.client.get(url)
            .header("Authorization", format!("Bearer {}", self.auth.as_ref().unwrap().access_token))
            .send()
            .await;

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

    pub async fn get_positions(&self, tracker: Tracker, from: DateTime<Local>, to: DateTime<Local>) -> Vec<Position> {
        debug!("Getting positions for tracker {} from {} to {}", tracker._id, from, to);

        let url = format!("{}/tracker/{}/positions", TRACTIVE_API_URL, tracker._id);

        let response = self.client.get(url)
            .query(&[("time_from", from.timestamp()), ("time_to", to.timestamp())])
            .query(&[("format", "json_segments")])
            .header("Authorization", format!("Bearer {}", self.auth.as_ref().unwrap().access_token))
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
