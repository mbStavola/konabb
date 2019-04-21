use chrono::{NaiveDateTime, Utc};
use diesel::{Associations, Identifiable, Insertable, Queryable};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{Board, User},
    schema::threads
};

#[derive(Associations, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[belongs_to(Board)]
#[belongs_to(User)]
#[primary_key(thread_id)]
#[table_name = "threads"]
pub struct Thread {
    thread_id: String,
    board_id: String,
    user_id: Option<String>,
    title: String,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
}

impl Thread {
    pub fn new(board_id: String, user_id: Option<String>, title: String) -> Self {
        Thread {
            thread_id: Uuid::new_v4().to_string(),
            board_id,
            user_id,
            title,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }

    pub fn thread_id(&self) -> &str {
        &self.thread_id
    }

    pub fn board_id(&self) -> &str {
        &self.board_id
    }

    pub fn user_id(&self) -> Option<&String> {
        self.user_id.as_ref()
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    pub fn updated_at(&self) -> Option<&NaiveDateTime> {
        self.updated_at.as_ref()
    }
}
