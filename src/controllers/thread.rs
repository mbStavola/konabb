use actix_web::HttpResponse;
use actix_web::web::{Data, Json, Path, Query};
use chrono::NaiveDateTime;
use redis::Commands;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use validator_derive::Validate;

use crate::{
    dao, models,
    util::{self, CachePool, DbPool, JwtSession, LoginClaims, Result},
};

#[derive(Deserialize, Serialize, Validate)]
pub struct ThreadSubmission {
    board_id: String,
    #[validate(length(min = 1, max = "64"))]
    title: String,
    #[validate(length(min = 1, max = "2000"))]
    text: String,
}

pub fn create_thread(
    db_pool: Data<DbPool>,
    cache_pool: Data<CachePool>,
    session: Option<JwtSession<LoginClaims>>,
    submission: Json<ThreadSubmission>,
) -> Result<HttpResponse> {
    if submission.validate().is_err() {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let cache_conn = &cache_pool.get().unwrap();
    let user_id: Option<String> = session.map(|it| cache_conn.get(it.jti()).expect("fre"));

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
    page: Option<u16>,
    size: Option<u8>,
}

pub fn get_thread(
    db_pool: Data<DbPool>,
    thread_id: Path<Uuid>,
    query: Query<ThreadParams>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(0);
    let size = query.size.unwrap_or(20);

    let db_conn = &db_pool.get().unwrap();

    let thread = dao::thread::get_thread(db_conn, &thread_id.to_string());
    if thread.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }
    let thread = thread.unwrap();

    #[derive(Serialize)]
    struct PostModel {
        post_id: String,
        user_id: Option<String>,
        rendered_text: Option<String>,
        created_at: NaiveDateTime,
        updated_at: Option<NaiveDateTime>,
    };

    // Anonymize based on board type
    let board_type = dao::board::board_type_for_thread(db_conn, thread.thread_id());
    let f: Box<FnMut(models::Post) -> PostModel> = if board_type == models::BoardType::Anonymous {
        Box::new(|post| {
            PostModel {
                post_id: post.post_id().to_owned(),
                user_id: None,
                rendered_text: post.rendered_text().map(|it| it.to_owned()),
                created_at: post.created_at().clone(),
                updated_at: post.updated_at().map(|it| it.clone()),
            }
        })
    } else {
        Box::new(|post| {
            PostModel {
                post_id: post.post_id().to_owned(),
                user_id: post.user_id().map(|it| it.to_owned()),
                rendered_text: post.rendered_text().map(|it| it.to_owned()),
                created_at: post.created_at().clone(),
                updated_at: post.updated_at().map(|it| it.clone()),
            }
        })
    };

    let post_count = dao::post::count_for_thread(db_conn, &thread);
    let pages = (post_count as f32 / size as f32).ceil() as u16 - 1;

    let posts: Vec<PostModel> = dao::post::get_in_thread(db_conn, &thread, page, size)
        .into_iter()
        .map(f)
        .collect();

    #[derive(Serialize)]
    struct ThreadResponse {
        thread: models::Thread,
        posts: Vec<PostModel>,
        page: u16,
        size: u8,
        pages: u16,
    };

    let body = ThreadResponse { thread, posts, page, size, pages };

    Ok(HttpResponse::Ok().json(body))
}

pub fn delete_thread(
    db_pool: Data<DbPool>,
    cache_pool: Data<CachePool>,
    session: Option<JwtSession<LoginClaims>>,
    thread_id: Path<Uuid>,
) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}
