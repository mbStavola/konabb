use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable};
use num::{FromPrimitive, ToPrimitive};
use num_derive::{FromPrimitive, ToPrimitive};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::boards;

#[derive(Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "boards"]
pub struct Board {
    board_id: String,
    name: String,
    short_name: String,
    description: Option<String>,
    board_type: u8,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
}

impl Board {
    pub fn new(
        name: String,
        short_name: String,
        description: Option<String>,
        board_type: BoardType,
    ) -> Self {
        Board {
            board_id: Uuid::new_v4().to_string(),
            name,
            short_name,
            description: None,
            board_type: board_type.to_u8().unwrap(),
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }

    pub fn board_id(&self) -> &str {
        &self.board_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn short_name(&self) -> &str {
        &self.short_name
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn board_type(&self) -> BoardType {
        FromPrimitive::from_u8(self.board_type).unwrap()
    }

    pub fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    pub fn updated_at(&self) -> Option<&NaiveDateTime> {
        self.updated_at.as_ref()
    }
}

#[derive(Debug, Deserialize, FromPrimitive, PartialEq, ToPrimitive, Serialize)]
pub enum BoardType {
    All = 0,
    Anonymous = 1,
    Identified = 2,
    Verified = 3,
}
