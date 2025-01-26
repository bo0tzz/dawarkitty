/*
 * API V1
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiV1OverlandBatchesPostRequestProperties {
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "altitude", skip_serializing_if = "Option::is_none")]
    pub altitude: Option<f64>,
    #[serde(rename = "speed", skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    #[serde(rename = "horizontal_accuracy", skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    #[serde(rename = "vertical_accuracy", skip_serializing_if = "Option::is_none")]
    pub vertical_accuracy: Option<f64>,
    #[serde(rename = "motion", skip_serializing_if = "Option::is_none")]
    pub motion: Option<Vec<String>>,
    #[serde(rename = "pauses", skip_serializing_if = "Option::is_none")]
    pub pauses: Option<bool>,
    #[serde(rename = "activity", skip_serializing_if = "Option::is_none")]
    pub activity: Option<String>,
    #[serde(rename = "desired_accuracy", skip_serializing_if = "Option::is_none")]
    pub desired_accuracy: Option<f64>,
    #[serde(rename = "deferred", skip_serializing_if = "Option::is_none")]
    pub deferred: Option<f64>,
    #[serde(rename = "significant_change", skip_serializing_if = "Option::is_none")]
    pub significant_change: Option<String>,
    #[serde(rename = "locations_in_payload", skip_serializing_if = "Option::is_none")]
    pub locations_in_payload: Option<f64>,
    #[serde(rename = "device_id", skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "wifi", skip_serializing_if = "Option::is_none")]
    pub wifi: Option<String>,
    #[serde(rename = "battery_state", skip_serializing_if = "Option::is_none")]
    pub battery_state: Option<String>,
    #[serde(rename = "battery_level", skip_serializing_if = "Option::is_none")]
    pub battery_level: Option<f64>,
}

impl ApiV1OverlandBatchesPostRequestProperties {
    pub fn new() -> ApiV1OverlandBatchesPostRequestProperties {
        ApiV1OverlandBatchesPostRequestProperties {
            timestamp: None,
            altitude: None,
            speed: None,
            horizontal_accuracy: None,
            vertical_accuracy: None,
            motion: None,
            pauses: None,
            activity: None,
            desired_accuracy: None,
            deferred: None,
            significant_change: None,
            locations_in_payload: None,
            device_id: None,
            wifi: None,
            battery_state: None,
            battery_level: None,
        }
    }
}

