use warp::Filter;

use crate::api::user;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let user_token_route = warp::path!("api" / "user" / "token")
        .and(warp::get())
        .and(warp::header::optional::<String>("Authorization"))
        .and_then(user::get_user_token);

    let user_profile_route = warp::path!("api" / "user" / "profile")
        .and(warp::get())
        .and(warp::header::optional::<String>("Authorization"))
        .and_then(user::get_user_profile);

    user_token_route.or(user_profile_route)
}