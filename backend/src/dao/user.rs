use diesel::{dsl::*, prelude::*};

use crate::{models, schema::users::dsl::*};

pub fn username_or_email_taken(
    conn: &MysqlConnection,
    t_username: &str,
    t_email: Option<&String>,
) -> bool {
    let filter = username.eq(t_username).or(email.eq(t_email));

    select(exists(users.filter(filter)))
        .get_result(conn)
        .expect("")
}

pub fn email_taken(conn: &MysqlConnection, t_email: &str) -> bool {
    select(exists(users.filter(email.eq(t_email))))
        .get_result(conn)
        .expect("")
}

pub fn get_user_by_username(conn: &MysqlConnection, t_username: &str) -> Option<models::User> {
    users
        .filter(username.eq(t_username))
        .first::<models::User>(conn)
        .optional()
        .expect("")
}

pub fn get_user(conn: &MysqlConnection, id: &str) -> Option<models::User> {
    users
        .filter(user_id.eq(id))
        .first::<models::User>(conn)
        .optional()
        .expect("")
}

pub fn get_users(conn: &MysqlConnection, ids: &Vec<String>) -> Vec<models::User> {
    users.filter(user_id.eq_any(ids))
        .load(conn)
        .expect("")
}

pub fn create_user(conn: &MysqlConnection, user: models::User) {
    insert_into(users)
        .values(user)
        .execute(conn)
        .expect("pls work");
}

pub fn update_user(conn: &MysqlConnection, t_user_id: &str, user_update: models::UserUpdate) {
    update(users.filter(user_id.eq(t_user_id)))
        .set(user_update)
        .execute(conn)
        .expect("");
}
