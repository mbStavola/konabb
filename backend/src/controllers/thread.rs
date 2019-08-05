use actix_web::web::{Data, Json, Path, Query};
use actix_web::HttpResponse;
use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use validator_derive::Validate;

use crate::{
    dao, models,
    util::{self, CachePool, DbPool, JwtSession, Result},
};
use std::collections::{HashSet, HashMap};

#[derive(Deserialize, Serialize, Validate)]
pub struct ThreadSubmission {
    board_id: String,
    #[validate(length(min = 1, max = 64))]
    title: String,
    #[validate(length(min = 1, max = 2000))]
    text: String,
}

pub fn create_thread(
    db_pool: Data<DbPool>,
    cache_pool: Data<CachePool>,
    session: JwtSession,
    submission: Json<ThreadSubmission>,
) -> Result<HttpResponse> {
    if submission.validate().is_err() {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let cache_conn = &mut cache_pool.get().unwrap();
    let user_id = session.user_id();

    let thread = models::Thread::new(
        submission.board_id.to_owned(),
        user_id.clone().map(|it| it.to_owned()),
        submission.title.to_owned(),
    );

    let post = models::Post::new(
        thread.thread_id().to_owned(),
        user_id.map(|it| it.to_owned()),
        submission.text.to_owned(),
        util::parse_markdown(&submission.text),
    );

    let db_conn = &db_pool.get().unwrap();

    let thread_id = thread.thread_id().to_owned();
    dao::thread::create_thread(db_conn, thread);
    dao::post::create_post(db_conn, post);

    Ok(HttpResponse::Created().body(thread_id))
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
    let page = query.page.unwrap_or(0).max(0);
    let size = query.size.unwrap_or(20).min(20);

    let db_conn = &db_pool.get().unwrap();

    let thread = dao::thread::get_thread(db_conn, &thread_id.to_string());
    if thread.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }
    let thread = thread.unwrap();

    let post_count = dao::post::count_for_thread(db_conn, &thread);
    let pages = (post_count as f32 / size as f32).ceil() as u16 - 1;

    let posts: Vec<models::Post> = dao::post::get_in_thread(db_conn, &thread, page, size);

    // TODO(Matt): This is copied from the user controller... find a way to better organize REST models
    #[derive(Clone, Deserialize, Serialize)]
    struct UserView {
        user_id: String,
        username: String,
        email: Option<String>,
    }

    #[derive(Serialize)]
    struct PostModel {
        post_id: String,
        user: Option<UserView>,
        text: String,
        created_at: NaiveDateTime,
        updated_at: Option<NaiveDateTime>,
    };

    // Anonymize based on board type
    let board_type = dao::board::board_type_for_thread(db_conn, thread.thread_id());
    let users: HashMap<String, UserView> = if board_type != models::BoardType::Anonymous{
        // SPEED(Matt): We only need to make this a Vec in order to pass it into get users-- requiring two collects
        let user_ids: Vec<String> = posts.iter()
            .filter_map(|post| post.user_id().map(|it| it.to_owned()))
            .collect::<HashSet<String>>()
            .into_iter()
            .collect();

        dao::user::get_users(db_conn, &user_ids)
            .into_iter()
            .map(|user| {
                let view = UserView {
                    user_id: user.user_id().to_owned(),
                    username: user.username().to_owned(),
                    email: user.email().map(|email| email.to_owned())
                };

                (user.user_id().to_owned(), view)
            })
            .collect()
    } else {
        HashMap::with_capacity(0)
    };

    let posts: Vec<PostModel> = dao::post::get_in_thread(db_conn, &thread, page, size)
        .into_iter()
        .map(|post| {
            let user = post.user_id()
                .and_then(|user_id| users.get(user_id))
                .map(|user| user.clone());

            PostModel {
                post_id: post.post_id().to_owned(),
                user,
                text: post.raw_text().to_owned(),
                created_at: post.created_at().clone(),
                updated_at: post.updated_at().map(|it| it.clone()),
            }
        })
        .collect();

    #[derive(Serialize)]
    struct ThreadResponse {
        thread: models::Thread,
        posts: Vec<PostModel>,
        page: u16,
        size: u8,
        pages: u16,
    };

    let body = ThreadResponse {
        thread,
        posts,
        page,
        size,
        pages,
    };

    Ok(HttpResponse::Ok().json(body))
}

pub fn delete_thread(
    db_pool: Data<DbPool>,
    cache_pool: Data<CachePool>,
    session: Option<JwtSession>,
    thread_id: Path<Uuid>,
) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}
