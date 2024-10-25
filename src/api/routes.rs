use warp::Filter;

use crate::api::envoy;
use crate::api::user;

pub fn routes(consul_server: &str) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    user::user_routes().or(envoy::envoy_routes(consul_server.to_string()))
}