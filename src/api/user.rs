use cached::proc_macro::cached;
use clap::Parser;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use warp::Filter;

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    iss: String,
    sub: String,
    exp: usize,
    // other claims can be added as necessary
}

#[derive(Parser, Debug)]
#[command(name = "rust_oidc_server")]
struct Args {
    /// Bind address for the webserver
    #[arg(long, default_value = "0.0.0.0:5000")]
    bind: String,
}

static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

pub fn user_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let user_token_route = warp::path!("api" / "user" / "token")
        .and(warp::get())
        .and(warp::header::optional::<String>("Authorization"))
        .and_then(get_user_token);

    let user_profile_route = warp::path!("api" / "user" / "profile")
        .and(warp::get())
        .and(warp::header::optional::<String>("Authorization"))
        .and_then(get_user_profile);

    user_token_route.or(user_profile_route)
}

pub async fn get_user_token(auth_header: Option<String>) -> Result<impl warp::Reply, warp::Rejection> {
    match parse_bearer(auth_header) {
        Ok(payload) => Ok(
            warp::reply::with_status(warp::reply::json(&payload), warp::http::StatusCode::OK)
        ),
        Err(e) => Ok(warp::reply::with_status(
            warp::reply::json(
                &serde_json::json!(
                    {"error": "Authorization header is missing or invalid", "details": e}
                )
            ),
            warp::http::StatusCode::UNAUTHORIZED,
        )),
    }
}

pub async fn get_user_profile(auth_header: Option<String>) -> Result<impl warp::Reply, warp::Rejection> {
    let payload = match parse_bearer(auth_header.clone()) {
        Ok(payload) => payload,
        Err(e) => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&serde_json::json!({"error": "Invalid token", "details": e})),
                warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    };

    let issuer = match payload.get("iss") {
        Some(iss) => iss.as_str().unwrap().to_string(),
        None => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&serde_json::json!({"error": "Missing 'iss' claim in token"})),
                warp::http::StatusCode::BAD_REQUEST,
            ));
        }
    };

    match fetch_oidc(issuer).await {
        Ok(oidc_config) => {
            if let Some(userinfo_endpoint) = oidc_config.get("userinfo_endpoint").expect("missing userinfo_endpoint").as_str() {
                match CLIENT
                    .get(userinfo_endpoint)
                    .header("Authorization", auth_header.unwrap_or_default())
                    .send()
                    .await
                {
                    Ok(response) => {
                        match response.json::<serde_json::Value>().await {
                            Ok(profile) => Ok(
                               warp::reply::with_status(warp::reply::json(&profile), warp::http::StatusCode::OK)
                            ),
                            Err(e) => Ok(warp::reply::with_status(
                                warp::reply::json(&serde_json::json!({"error": "Failed to parse user profile", "details": e.to_string()})),
                                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                            )),
                        }
                    }
                    Err(e) => Ok(warp::reply::with_status(
                        warp::reply::json(&serde_json::json!({"error": "Failed to fetch user profile", "details": e.to_string()})),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    )),
                }
            } else {
                Ok(warp::reply::with_status(
                    warp::reply::json(&serde_json::json!({"error": "Failed to fetch user profile: missing userinfo endpoint"})),
                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                ))
            }
        }
        Err(e) => Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({"error": "Failed to fetch user profile", "details": e})),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

fn parse_bearer(auth_header: Option<String>) -> Result<HashMap<String, serde_json::Value>, String> {
    if let Some(header) = auth_header {
        if header.starts_with("Bearer ") {
            let token = header.trim_start_matches("Bearer ");

            // Use a custom Validation object that disables signature validation
            let mut validation = Validation::new(Algorithm::RS256);
            validation.insecure_disable_signature_validation();

            // Decode the token, ignoring the signature
            let token_data = decode::<HashMap<String, serde_json::Value>>(
                &token,
                &DecodingKey::from_secret("".as_ref()),
                &validation,
            );

            match token_data {
                Ok(data) => return Ok(data.claims),
                Err(err) => return Err(format!("Failed to decode JWT: {}", err)),
            }
        }
    }
    Err(String::from("Authorization header is missing or invalid"))
}

#[cached(time = 600)]
async fn fetch_oidc(issuer: String) -> Result<HashMap<String, serde_json::Value>, String> {
    let config_url = format!("{}/.well-known/openid-configuration", issuer);
    println!("{}", config_url);
    match CLIENT.get(&config_url).send().await {
        Ok(response) => match response.json::<HashMap<String, serde_json::Value>>().await {
            Ok(config ) => Ok(config),
            Err(e) => Err(format!("Failed to parse OIDC config: {}", e)),
        },
        Err(e) => Err(format!("Failed to fetch OIDC config: {}", e)),
    }
}
