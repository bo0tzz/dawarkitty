use std::fmt::{Debug, Formatter};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EnvConfig {
    pub dawarich_host: String,
    pub dawarich_api_key: String,
    pub tractive_email: String,
    pub tractive_password: String,
    #[serde(default = "default_prometheus_port")]
    pub prometheus_port: u16,
}

fn default_prometheus_port() -> u16 {
    9090
}

pub fn load_env() -> EnvConfig {
    match envy::from_env::<EnvConfig>() {
        Ok(config) => config,
        Err(error) => panic!("Failed to load environment variables: {}", error),
    }
}

impl Debug for EnvConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnvConfig")
            .field("dawarich_host", &self.dawarich_host)
            .field("dawarich_api_key", &"***********")
            .field("tractive_email", &self.tractive_email)
            .field("tractive_password", &"***********")
            .field("prometheus_port", &self.prometheus_port)
            .finish()
    }
}