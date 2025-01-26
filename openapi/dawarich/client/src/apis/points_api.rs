/*
 * API V1
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`api_v1_owntracks_points_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV1OwntracksPointsPostError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v1_points_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV1PointsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v1_points_id_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV1PointsIdDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v1_points_tracked_months_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV1PointsTrackedMonthsGetError {
    Status401(),
    UnknownValue(serde_json::Value),
}


pub async fn api_v1_owntracks_points_post(configuration: &configuration::Configuration, api_key: &str, api_v1_owntracks_points_post_request: Option<models::ApiV1OwntracksPointsPostRequest>) -> Result<(), Error<ApiV1OwntracksPointsPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_key = api_key;
    let p_api_v1_owntracks_points_post_request = api_v1_owntracks_points_post_request;

    let uri_str = format!("{}/api/v1/owntracks/points", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    req_builder = req_builder.query(&[("api_key", &p_api_key.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_api_v1_owntracks_points_post_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiV1OwntracksPointsPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn api_v1_points_get(configuration: &configuration::Configuration, api_key: &str, start_at: Option<&str>, end_at: Option<&str>, page: Option<i32>, per_page: Option<i32>, order: Option<&str>) -> Result<Vec<models::ApiV1PointsGet200ResponseInner>, Error<ApiV1PointsGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_key = api_key;
    let p_start_at = start_at;
    let p_end_at = end_at;
    let p_page = page;
    let p_per_page = per_page;
    let p_order = order;

    let uri_str = format!("{}/api/v1/points", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("api_key", &p_api_key.to_string())]);
    if let Some(ref param_value) = p_start_at {
        req_builder = req_builder.query(&[("start_at", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_at {
        req_builder = req_builder.query(&[("end_at", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_per_page {
        req_builder = req_builder.query(&[("per_page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_order {
        req_builder = req_builder.query(&[("order", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiV1PointsGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn api_v1_points_id_delete(configuration: &configuration::Configuration, api_key: &str, id: &str) -> Result<(), Error<ApiV1PointsIdDeleteError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_key = api_key;
    let p_id = id;

    let uri_str = format!("{}/api/v1/points/{id}", configuration.base_path, id=crate::apis::urlencode(p_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    req_builder = req_builder.query(&[("api_key", &p_api_key.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiV1PointsIdDeleteError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn api_v1_points_tracked_months_get(configuration: &configuration::Configuration, api_key: &str) -> Result<Vec<models::ApiV1PointsTrackedMonthsGet200ResponseInner>, Error<ApiV1PointsTrackedMonthsGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_key = api_key;

    let uri_str = format!("{}/api/v1/points/tracked_months", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("api_key", &p_api_key.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiV1PointsTrackedMonthsGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

