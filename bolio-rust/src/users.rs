use crate::models::{NewUser, User};
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::PgConnection;

pub fn create_user(
    conn: &PgConnection,
    new_user: &NewUser,
) -> Result<User, diesel::result::Error> {
    diesel::insert_into(users)
        .values(new_user)
        .get_result(conn)
}

pub fn read_user(conn: &PgConnection, user_id: i32) -> Result<User, diesel::result::Error> {
    users.find(user_id).first(conn)
}

pub fn update_user(
    conn: &PgConnection,
    user_id: i32,
    updated_user: &NewUser,
) -> Result<User, diesel::result::Error> {
    diesel::update(users.find(user_id))
        .set(updated_user)
        .get_result(conn)
}

pub fn update_user_last_login(
    conn: &PgConnection,
    user_id: i32,
    login_time: chrono::DateTime<chrono::Utc>,
) -> Result<User, diesel::result::Error> {
    diesel::update(users.find(user_id))
        .set(last_login.eq(Some(login_time))) // Update the `last_login` field
        .get_result(conn)
}

pub fn update_user_session(
    conn: &PgConnection,
    user_id: i32,
    new_session_id: Option<String>,
) -> Result<User, diesel::result::Error> {
    diesel::update(users.find(user_id))
        .set(session_id.eq(new_session_id)) // Update the `session_id` field
        .get_result(conn)
}

pub fn delete_user(conn: &PgConnection, user_id: i32) -> Result<usize, diesel::result::Error> {
    diesel::delete(users.find(user_id)).execute(conn)
}
