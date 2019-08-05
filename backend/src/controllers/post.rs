use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use serde_derive::{Deserialize, Serialize};
use validator::Validate;
use validator_derive::Validate;

use crate::{
    dao,
    models::{BoardType, Post},
    util::{self, CachePool, DbPool, JwtSession, Result},
};

#[derive(Deserialize, Serialize, Validate)]
pub struct PostSubmission {
    thread_id: String,
    #[validate(length(max = 2000))]
    text: String,
}

pub fn create_post(
    db_pool: Data<DbPool>,
    cache_pool: Data<CachePool>,
    session: JwtSession,
    submission: Json<PostSubmission>,
) -> Result<HttpResponse> {
    if submission.validate().is_err() {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let user_id = session.user_id();

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
            }
            BoardType::Verified => return Ok(HttpResponse::Unauthorized().finish()),
        };

        let post = Post::new(
            submission.thread_id.to_owned(),
            user_id.map(|it| it.to_owned()),
            submission.text.to_owned(),
            util::parse_markdown(&submission.text),
        );

        dao::post::create_post(db_conn, post);
    }

    Ok(HttpResponse::Ok().finish())
}

pub fn update_post(
    db_pool: Data<DbPool>,
    cache_pool: Data<CachePool>,
    session: JwtSession,
) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

pub fn delete_post(
    db_pool: Data<DbPool>,
    cache_pool: Data<CachePool>,
    session: JwtSession,
) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}
