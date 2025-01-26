use chrono::serde::ts_seconds;
use clokwerk::TimeUnits;
use log::{debug, trace};
use reqwest::header;
use serde::Deserialize;
use std::convert::Into;

pub struct TractiveApi {
    email: String,
    password: String,
    pub auth: Option<AuthTokenResponse>,
    client: reqwest::Client,
}

#[derive(Deserialize)]
pub struct AuthTokenResponse {
    user_id: String,
    client_id: String,
    #[serde(with = "ts_seconds")]
    pub expires_at: chrono::DateTime<chrono::Utc>,
    access_token: String,
}

const TRACTIVE_API_URL: &str = "https://graph.tractive.com/4";
const TRACTIVE_CLIENT_ID: &str = "5728aa1fc9077f7c32000186";
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:134.0) Gecko/20100101 Firefox/134.0";

const AUTH_RENEW_THRESHOLD: i64 = 1.day().into();

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

        state.check_auth().await;

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
                if diff.num_seconds() < AUTH_RENEW_THRESHOLD {
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
}
