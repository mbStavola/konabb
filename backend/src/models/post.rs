use chrono::{NaiveDateTime, Utc};
use diesel::{Associations, Identifiable, Insertable, Queryable};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{Thread, User},
    schema::posts,
};

#[derive(Associations, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[belongs_to(Thread)]
#[belongs_to(User)]
#[primary_key(post_id)]
#[table_name = "posts"]
pub struct Post {
    post_id: String,
    thread_id: String,
    user_id: Option<String>,
    raw_text: String,
    rendered_text: Option<String>,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
}

impl Post {
    pub fn new(
        thread_id: String,
        user_id: Option<String>,
        raw_text: String,
        rendered_text: Option<String>,
    ) -> Self {
        Post {
            post_id: Uuid::new_v4().to_string(),
            thread_id,
            user_id,
            raw_text,
            rendered_text,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }

    pub fn post_id(&self) -> &str {
        &self.post_id
    }

    pub fn thread_id(&self) -> &str {
        &self.thread_id
    }

    pub fn user_id(&self) -> Option<&String> {
        self.user_id.as_ref()
    }

    pub fn raw_text(&self) -> &str {
        &self.raw_text
    }

    pub fn rendered_text(&self) -> Option<&String> {
        self.rendered_text.as_ref()
    }

    pub fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    pub fn updated_at(&self) -> Option<&NaiveDateTime> {
        self.updated_at.as_ref()
    }
}
