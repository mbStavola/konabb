use diesel::{Insertable, Queryable};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use validator_derive::Validate;

use crate::schema::users;

#[derive(Clone, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "users"]
pub struct User {
    user_id: String,
    username: String,
    email: Option<String>,
    password: String,
}

impl User {
    pub fn new(username: String, email: Option<String>, password: String) -> Self {
        User {
            user_id: Uuid::new_v4().to_string(),
            username,
            email,
            password,
        }
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn email(&self) -> Option<&String> {
        self.email.as_ref()
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

#[derive(AsChangeset, Deserialize, Serialize, Validate)]
#[table_name = "users"]
pub struct UserUpdate {
    user_id: String,
    password: Option<String>,
    #[validate(email)]
    email: Option<String>,
}

impl UserUpdate {
    pub fn new(user_id: String, password: Option<String>, email: Option<String>) -> Self {
        UserUpdate {
            user_id,
            password,
            email,
        }
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn email(&self) -> Option<&String> {
        self.email.as_ref()
    }

    pub fn password(&self) -> Option<&String> {
        self.password.as_ref()
    }
}
