// use super::{PgPool, RedisPool};
// use crate::api::error;
// use actix_session::Session;
use actix_web::web::{self};
// use actix_web::web::{self, Data, Json};
use actix_web::Responder;
use actix_web::{HttpResponse, Result};
// use oauth2::reqwest::async_http_client;
// use oauth2::AuthorizationCode;
// use oauth2::TokenResponse;
// use redis::Commands;
// use serde::{Deserialize, Serialize};
// use std::env;
pub mod session;
mod util;

// use self::session::AuthUser;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/health_check").route(web::get().to(health_check)));
    // .service(web::resource("/login").route(web::post().to(login)));
}
async fn health_check() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}
// async fn login(
//     session: Session,
//     req: Json<LoginRequest>,
//     pg_pool: Data<PgPool>,
//     redis_pool: Data<RedisPool>,
// ) -> Result<impl Responder> {
//     //extracting the authorization code from the request body
//     let code = AuthorizationCode::new(req.code.clone());

//     //exchanging the authorization code for the access token
//     let access_token = util::client()
//         .exchange_code(code)
//         .request_async(async_http_client)
//         .await
//         .map_err(|err| error::handle_error(err.into()))?
//         .access_token()
//         .secret()
//         .clone();

//     let url =
//         env::var("GOOGLE_OAUTH_USER_INFO_URL").expect("GOOGLE_OAUTH_USER_INFO_URL must be set"); //url for getting user info from google

//     //exchanging the access token for the user info
//     let client = reqwest::Client::new();
//     let response = client
//         .get(url)
//         .header("Authorization", format!("Bearer {access_token}"))
//         .send()
//         .await;

//     let userinfo: UserInfoFromGoogle = response
//         .map_err(|err| error::handle_error(err.into()))?
//         .json()
//         .await
//         .map_err(|err| error::handle_error(err.into()))?;
//     let email = userinfo.email;
//     let name = userinfo.name;

//     //checking if the user exists in db else creating a new user
//     let user = web::block(move || {
//         let mut conn = pg_pool.get()?;
//         util::get_oauth_user(&mut conn, &email, &name)
//     })
//     .await?
//     .map_err(|err| error::handle_error(err.into()))?;

//     //generating jwt token
//     let (token, expiring_time, device) = util::generate_jwt_token(user.id).unwrap();

//     //get redis connection from redis pool
//     let mut redis_conn = redis_pool
//         .get()
//         .map_err(|err| error::handle_error(err.into()))?;

//     //set device id in redis db
//     redis_conn
//         .set(user.id, device + &expiring_time)
//         .map_err(|err| error::handle_error(err.into()))?;

//     // insert the jwt token in the session cookie
//     session
//         .insert("token", token.clone())
//         .expect("Failed to insert token in session");

//     Ok(HttpResponse::Ok()
//         .append_header(("expiry_time", expiring_time))
//         .json(Json(LoginResponse {
//             user_id: user.id,
//             username: user.username,
//             name: user.name,
//             avatar_id: user.avatar_id,
//             attacks_won: user.attacks_won,
//             defenses_won: user.defenses_won,
//             trophies: user.trophies,
//             artifacts: user.artifacts,
//             email: user.email,
//             token: Some(token),
//         })))
// }
