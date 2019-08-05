use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use num::FromPrimitive;
use serde::{Deserialize, Serialize};
use validator::Validate;
use validator_derive::Validate;

use crate::{
    dao, models,
    models::{Board, BoardType},
    util::{DbPool, Result},
};
use actix_web::web::{Path, Query};

#[derive(Deserialize, Serialize, Validate)]
pub struct BoardSubmission {
    #[validate(length(min = 1, max = 36))]
    name: String,
    #[validate(length(min = 1, max = 5))]
    short_name: String,
    #[validate(length(max = 255))]
    description: Option<String>,
    board_type: u8,
}

pub fn create_board(
    db_pool: Data<DbPool>,
    submission: Json<BoardSubmission>,
) -> Result<HttpResponse> {
    if submission.validate().is_err() {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let board_type: BoardType = {
        let board_type_id = (&*submission).board_type;
        FromPrimitive::from_u8(board_type_id).unwrap()
    };

    let board = Board::new(
        submission.name.to_owned(),
        submission.short_name.to_owned(),
        submission.description.clone(),
        board_type,
    );

    let db_conn = &db_pool.get().unwrap();

    dao::board::create_board(db_conn, board);

    Ok(HttpResponse::Created().finish())
}

#[derive(Deserialize)]
pub struct BoardParams {
    page: Option<u16>,
    size: Option<u8>,
}

pub fn get_board(
    db_pool: Data<DbPool>,
    short_name: Path<String>,
    query: Query<BoardParams>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(0);
    let size = query.size.unwrap_or(20);

    let db_conn = &db_pool.get().unwrap();
    let board = dao::board::get_board_by_short_name(db_conn, &short_name);
    if board.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }
    let board = board.unwrap();

    let thread_count = dao::thread::count_for_board(db_conn, &board);
    let pages = (thread_count as f32 / size as f32).ceil() as u16 - 1;

    let threads = dao::thread::get_in_board(db_conn, &board, page, size);

    #[derive(Serialize)]
    struct BoardResponse {
        board: models::Board,
        threads: Vec<models::Thread>,
        page: u16,
        size: u8,
        pages: u16,
    };

    let body = BoardResponse {
        board,
        threads,
        page,
        size,
        pages,
    };

    Ok(HttpResponse::Ok().json(body))
}

pub fn list_boards(db_pool: Data<DbPool>) -> Result<HttpResponse> {
    let db_conn = &db_pool.get().unwrap();
    let boards = dao::board::list_boards(db_conn);
    Ok(HttpResponse::Ok().json(boards))
}
