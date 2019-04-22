use actix_web::{
    HttpResponse,
    web::{Data, Json, Path},
};
use chrono::Duration;
use redis::Commands;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use validator_derive::Validate;

use crate::{
    dao,
    models,
    util::{
        self,
        JwtSession,
        LoginClaims,
        CachePool,
        DbPool,
        Result,
    },
};

#[derive(Deserialize, Serialize, Validate)]
pub struct UserSubmission {
    username: String,
    password: String,
    #[validate(email)]
    email: Option<String>,
}

/// User create
pub fn create_user(
    db_pool: Data<DbPool>,
    submission: Json<UserSubmission>,
) -> Result<HttpResponse> {
    if submission.validate().is_err() {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let db_conn = &db_pool.get().unwrap();
    let existing: bool = dao::user::username_or_email_taken(
        db_conn,
        &submission.username,
        submission.email.as_ref(),
    );

    if existing {
        return Ok(HttpResponse::Conflict().finish());
    }

    let hashed = util::hash(&submission.password).expect("Couldn't hash password");

    let user = models::User::new(
        submission.username.to_owned(),
        submission.email.to_owned(),
        hashed,
    );

    dao::user::create_user(db_conn, user);

    Ok(HttpResponse::Created().finish())
}

/// User update
pub fn update_user(
    db_pool: Data<DbPool>,
    cache_pool: Data<CachePool>,
    session: JwtSession<LoginClaims>,
    submission: Json<models::UserUpdate>,
) -> Result<HttpResponse> {
    if submission.validate().is_err() {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let cache_conn = &cache_pool.get().unwrap();
    let sm: String = cache_conn.get(session.jti()).expect("fre");

    let db_conn = &db_pool.get().unwrap();

    if let Some(e) = &submission.email() {
        let existing: bool = dao::user::email_taken(db_conn, e);

        if existing {
            return Ok(HttpResponse::Conflict().finish());
        }
    }

    let e = submission.email().map(|it| it.clone());

    let hashed = submission
        .password()
        .map(|pswd| util::hash(pswd).expect("Couldn't hash password"));

    let user_update = models::UserUpdate::new(submission.user_id().to_owned(), hashed, e);

    dao::user::update_user(db_conn, submission.user_id(), user_update);

    Ok(HttpResponse::NoContent().finish())
}

/// User details
pub fn get_user(
    pool: Data<DbPool>,
    id: Path<Uuid>,
) -> Result<HttpResponse> {
    let db_conn = &pool.get().unwrap();
    let user = dao::user::get_user(db_conn, &id.to_string());

    #[derive(Deserialize, Serialize)]
    struct UserView {
        user_id: String,
        username: String,
        email: Option<String>,
    }

    let response: HttpResponse = user
        .map(|user| UserView {
            user_id: user.user_id().to_string(),
            username: user.username().to_string(),
            email: user.email().map(|it| it.clone()),
        })
        .map(|user| HttpResponse::Ok().json(user))
        .unwrap_or_else(|| HttpResponse::NotFound().finish());

    Ok(response)
}

#[derive(Deserialize, Serialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

/// User login
pub fn login(
    db_pool: Data<DbPool>,
    cache_pool: Data<CachePool>,
    submission: Json<LoginForm>,
) -> Result<HttpResponse> {
    let db_conn = &db_pool.get().unwrap();
    let user = dao::user::get_user_by_username(db_conn, &submission.username);

    if user.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }

    let user = user.unwrap();
    let valid = util::verify(&submission.password, user.password()).expect("pls hash");

    if !valid {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let (id, token) = util::issue();
    let cache_conn = &cache_pool.get().unwrap();
    let _: () = cache_conn.set(&id, user.user_id()).expect("fre");

    #[derive(Serialize)]
    struct AuthResponse {
        access_token: String,
        token_type: String,
        // Should be enum?
        expires_in: i64,
    }

    let response = AuthResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in: Duration::hours(1).num_seconds(),
    };

    Ok(HttpResponse::Ok().json(response))
}
