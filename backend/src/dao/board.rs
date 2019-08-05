use diesel::{dsl::*, prelude::*};
use num::FromPrimitive;

use crate::{models, schema::boards::dsl::*};

pub fn create_board(conn: &MysqlConnection, board: models::Board) {
    insert_into(boards)
        .values(board)
        .execute(conn)
        .expect("pls work");
}

pub fn get_board_by_short_name(
    conn: &MysqlConnection,
    t_short_name: &str,
) -> Option<models::Board> {
    boards
        .filter(short_name.eq(t_short_name))
        .first::<models::Board>(conn)
        .optional()
        .expect("")
}

pub fn board_type_for_thread(conn: &MysqlConnection, thread_id: &str) -> models::BoardType {
    use crate::schema::threads;
    let board_type_id = boards
        .inner_join(threads::table)
        .select(board_type)
        .filter(threads::thread_id.eq(thread_id))
        .first::<u8>(conn)
        .expect("");

    FromPrimitive::from_u8(board_type_id).unwrap()
}

pub fn list_boards(conn: &MysqlConnection) -> Vec<models::Board> {
    boards.load(conn).expect("")
}
