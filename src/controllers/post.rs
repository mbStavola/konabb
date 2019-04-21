use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use diesel::{r2d2::ConnectionManager as DieselConnectionManager, MysqlConnection};
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use redis::Commands;
use serde_derive::{Deserialize, Serialize};
use validator::Validate;
use validator_derive::Validate;

use crate::{
    dao,
    models::{Post, BoardType},
    util::{self, JwtSession, LoginClaims, Result},
};

#[derive(Deserialize, Serialize, Validate)]
pub struct PostSubmission {
    thread_id: String,
    #[validate(length(max = "2000"))]
    text: String,
}

pub fn create_post(
    db_pool: Data<Pool<DieselConnectionManager<MysqlConnection>>>,
    cache_pool: Data<Pool<RedisConnectionManager>>,
    session: Option<JwtSession<LoginClaims>>,
    submission: Json<PostSubmission>,
) -> Result<HttpResponse> {
    if submission.validate().is_err() {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let user_id: Option<String> = {
        let cache_conn = &cache_pool.get().unwrap();
        session.map(|it| cache_conn.get(it.jti()).expect("fre"))
    };

    {
        let db_conn = &db_pool.get().unwrap();
        let board_type = dao::board::board_type_for_thread(db_conn, &submission.thread_id);

        let user_id = match board_type {
            BoardType::All | BoardType::Anonymous => user_id,
            BoardType::Identified => {
                if user_id.is_none() {
                    return Ok(HttpResponse::Unauthorized().finish());
                }
                user_id
            },
            BoardType::Verified => return Ok(HttpResponse::Unauthorized().finish()),
        };

        let post = Post::new(
            submission.thread_id.to_owned(),
            user_id,
            submission.text.to_owned(),
            util::parse_markdown(&submission.text),
        );

        dao::post::create_post(db_conn, post);
    }

    Ok(HttpResponse::Ok().finish())
}

pub fn update_post(
    db_pool: Data<Pool<DieselConnectionManager<MysqlConnection>>>,
    cache_pool: Data<Pool<RedisConnectionManager>>,
    session: JwtSession<LoginClaims>,
) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

pub fn delete_post(
    db_pool: Data<Pool<DieselConnectionManager<MysqlConnection>>>,
    cache_pool: Data<Pool<RedisConnectionManager>>,
    session: JwtSession<LoginClaims>,
) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}
