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


/// struct for typed errors of method [`api_v1_areas_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV1AreasGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v1_areas_id_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV1AreasIdDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v1_areas_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV1AreasPostError {
    Status422(),
    UnknownValue(serde_json::Value),
}


pub async fn api_v1_areas_get(configuration: &configuration::Configuration, api_key: &str) -> Result<Vec<models::ApiV1AreasGet200ResponseInner>, Error<ApiV1AreasGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_key = api_key;

    let uri_str = format!("{}/api/v1/areas", configuration.base_path);
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
        let entity: Option<ApiV1AreasGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn api_v1_areas_id_delete(configuration: &configuration::Configuration, api_key: &str, id: &str) -> Result<(), Error<ApiV1AreasIdDeleteError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_key = api_key;
    let p_id = id;

    let uri_str = format!("{}/api/v1/areas/{id}", configuration.base_path, id=crate::apis::urlencode(p_id));
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
        let entity: Option<ApiV1AreasIdDeleteError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn api_v1_areas_post(configuration: &configuration::Configuration, api_key: &str, api_v1_areas_post_request: Option<models::ApiV1AreasPostRequest>) -> Result<(), Error<ApiV1AreasPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_key = api_key;
    let p_api_v1_areas_post_request = api_v1_areas_post_request;

    let uri_str = format!("{}/api/v1/areas", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    req_builder = req_builder.query(&[("api_key", &p_api_key.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_api_v1_areas_post_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiV1AreasPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

