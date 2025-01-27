use geojson::Feature;
use log::debug;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};

pub struct DawarichApi {
    host: String,
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkPoints {
    pub locations: Vec<Feature>,
}

impl DawarichApi {
    pub fn new(host: &str, api_key: &str) -> Self {
        let mut default_headers = header::HeaderMap::new();
        default_headers.insert("Authorization", api_key.parse().unwrap());
        let client = Client::builder()
            .default_headers(default_headers)
            .build()
            .unwrap();

        DawarichApi {
            host: host.to_owned(),
            client,
        }
    }

    pub async fn insert_bulk_points(&self, points: BulkPoints) {
        let url = format!("{}/api/v1/points", self.host);
        let res = self.client.post(&url).json(&points).send().await.unwrap();
        assert!(res.status().is_success());
        debug!("{:?}", res.text().await.unwrap());
    }

}

impl From<Vec<Feature>> for BulkPoints {
    fn from(locations: Vec<Feature>) -> Self {
        BulkPoints {
            locations
        }
    }
}