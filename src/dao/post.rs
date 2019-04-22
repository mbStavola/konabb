use diesel::{dsl::*, prelude::*};

use crate::{models, schema::posts::dsl::*};

pub fn create_post(conn: &MysqlConnection, post: models::Post) {
    insert_into(posts)
        .values(post)
        .execute(conn)
        .expect("pls wrk");
}

pub fn count_for_thread(conn: &MysqlConnection, thread: &models::Thread) -> i64 {
    models::Post::belonging_to(thread)
        .select(count(post_id))
        .first(conn)
        .expect("")
}

pub fn get_in_thread(conn: &MysqlConnection, thread: &models::Thread, page: u16, size: u8) -> Vec<models::Post> {
    let offset = (page * size as u16) as i64;

    models::Post::belonging_to(thread)
        .order(created_at.asc())
        .offset(offset)
        .limit(size as i64)
        .load(conn)
        .expect("")
}