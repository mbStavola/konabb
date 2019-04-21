use diesel::{dsl::*, prelude::*};

use crate::{models, schema::threads::dsl::*};

pub fn create_thread(conn: &MysqlConnection, thread: models::Thread) {
    insert_into(threads)
        .values(thread)
        .execute(conn)
        .expect("pls work");
}
