use diesel::{dsl::*, prelude::*};

use crate::{models, schema::threads::dsl::*};

pub fn create_thread(conn: &MysqlConnection, thread: models::Thread) {
    insert_into(threads)
        .values(thread)
        .execute(conn)
        .expect("pls work");
}

pub fn get_thread(conn: &MysqlConnection, t_thread_id: &String) -> Option<models::Thread> {
    return threads.filter(thread_id.eq(t_thread_id))
        .first::<models::Thread>(conn)
        .optional()
        .expect("")
}