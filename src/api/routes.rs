use warp::Filter;

use crate::api::envoy;
use crate::api::user;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    user::user_routes().or(envoy::envoy_routes())
}