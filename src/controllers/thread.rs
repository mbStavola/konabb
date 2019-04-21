use actix_web::web::{Data, Json, Path, Query};
use actix_web::HttpResponse;
use diesel::{r2d2::ConnectionManager as DieselConnectionManager, MysqlConnection};
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use redis::Commands;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use validator_derive::Validate;

use crate::{
    dao, models,
    util::{self, JwtSession, LoginClaims, Result},
};

#[derive(Deserialize, Serialize, Validate)]
pub struct ThreadSubmission {
    board_id: String,
    #[validate(length(max = "64"))]
    title: String,
    #[validate(length(max = "2000"))]
    text: String,
}

pub fn create_thread(
    db_pool: Data<Pool<DieselConnectionManager<MysqlConnection>>>,
    cache_pool: Data<Pool<RedisConnectionManager>>,
    session: Option<JwtSession<LoginClaims>>,
    submission: Json<ThreadSubmission>,
) -> Result<HttpResponse> {
    if submission.validate().is_err() {
        return Ok(HttpResponse::BadRequest().finish());
    }

    println!("{:?}", session);

    let cache_conn = &cache_pool.get().unwrap();
    let user_id: Option<String> = session.map(|it| cache_conn.get(it.jti()).expect("fre"));

    println!("{:?}", user_id);

    let thread = models::Thread::new(
        submission.board_id.to_owned(),
        user_id.clone(),
        submission.title.to_owned(),
    );

    let post = models::Post::new(
        thread.thread_id().to_owned(),
        user_id,
        submission.text.to_owned(),
        util::parse_markdown(&submission.text),
    );

    let db_conn = &db_pool.get().unwrap();

    dao::thread::create_thread(db_conn, thread);
    dao::post::create_post(db_conn, post);

    Ok(HttpResponse::Created().finish())
}

#[derive(Deserialize)]
pub struct ThreadParams {
    page: u16,
    size: u8,
}

pub fn get_thread(
    db_pool: Data<Pool<DieselConnectionManager<MysqlConnection>>>,
    thread_id: Path<Uuid>,
    query: Query<ThreadParams>,
) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

pub fn delete_thread(
    db_pool: Data<Pool<DieselConnectionManager<MysqlConnection>>>,
    cache_pool: Data<Pool<RedisConnectionManager>>,
    session: Option<JwtSession<LoginClaims>>,
    thread_id: Path<Uuid>,
) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}
