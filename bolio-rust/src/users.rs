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

pub fn delete_user(conn: &PgConnection, user_id: i32) -> Result<usize, diesel::result::Error> {
    diesel::delete(users.find(user_id)).execute(conn)
}
