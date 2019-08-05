use diesel::{dsl::*, prelude::*};

use crate::{models, schema::threads::dsl::*};

pub fn create_thread(conn: &MysqlConnection, thread: models::Thread) {
    insert_into(threads)
        .values(thread)
        .execute(conn)
        .expect("pls work");
}

pub fn count_for_board(conn: &MysqlConnection, board: &models::Board) -> i64 {
    models::Thread::belonging_to(board)
        .select(count(board_id))
        .first(conn)
        .expect("")
}

pub fn get_in_board(
    conn: &MysqlConnection,
    board: &models::Board,
    page: u16,
    size: u8,
) -> Vec<models::Thread> {
    let offset = (page * size as u16) as i64;

    models::Thread::belonging_to(board)
        .order(created_at.asc())
        .offset(offset)
        .limit(size as i64)
        .load(conn)
        .expect("")
}

pub fn get_thread(conn: &MysqlConnection, t_thread_id: &String) -> Option<models::Thread> {
    return threads
        .filter(thread_id.eq(t_thread_id))
        .first::<models::Thread>(conn)
        .optional()
        .expect("");
}
