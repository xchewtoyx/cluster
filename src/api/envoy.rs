use reqwest;
use serde_json;
use warp::Filter;

use crate::consul::services;
use crate::envoy::eds;


#[derive(Debug)]
#[allow(dead_code)]
enum ApiError {
    Request(reqwest::Error),
    Json(serde_json::Error),
}

impl warp::reject::Reject for ApiError {}

pub fn envoy_routes(consul_server: String) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let consul_server = warp::any().map(move || consul_server.clone());
    let envoy_eds_route = warp::path!("api" / "envoy" / "eds" / String)
        .and(warp::get())
        .and(consul_server)
        .and_then(get_envoy_eds);

    envoy_eds_route
}

async fn get_envoy_eds(service_name: String, consul_server: String) -> Result<impl warp::Reply, warp::Rejection> {
    let services: Vec<services::Service> = services::get_service(&consul_server, &service_name)
        .await
        .map_err(|e| warp::reject::custom(ApiError::Request(e)))?;

    if services.is_empty() {
        return Err(warp::reject::not_found());  // or custom error
    }

    let eds_json = eds::transform_consul_to_eds(services)
        .await
        .map_err(|e| warp::reject::custom(ApiError::Json(e)))?;

    Ok(warp::reply::json(&eds_json))
}
