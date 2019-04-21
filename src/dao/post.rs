use diesel::{dsl::*, prelude::*};

use crate::{models, schema::posts::dsl::*};

pub fn create_post(conn: &MysqlConnection, post: models::Post) {
    insert_into(posts)
        .values(post)
        .execute(conn)
        .expect("pls wrk");
}
