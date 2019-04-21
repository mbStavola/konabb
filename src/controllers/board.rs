use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use diesel::{r2d2::ConnectionManager as DieselConnectionManager, MysqlConnection};
use num::FromPrimitive;
use r2d2::Pool;
use serde::{Deserialize, Serialize};
use validator::Validate;
use validator_derive::Validate;

use crate::{
    dao,
    models::{Board, BoardType},
    util::Result,
};

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
    db_pool: Data<Pool<DieselConnectionManager<MysqlConnection>>>,
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