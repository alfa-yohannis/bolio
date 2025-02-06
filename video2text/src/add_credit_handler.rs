use crate::{
    schema::{credit_transactions, users},
    DbPool,
};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use askama::Template;
use diesel::prelude::*;
use serde::Deserialize;

// Define a template for the Add Credit page
#[derive(Template)]
#[template(path = "add_credit.html")]
struct AddCreditTemplate {
    session_id: Option<String>, // Session ID from cookies
    username: String,
}

pub async fn show(req: HttpRequest) -> impl Responder {
    // Retrieve session_id from cookies
    let session_id = req
        .cookie("session_id")
        .map(|cookie| cookie.value().to_string());

    // Retrieve username from cookies
    let username = req
        .cookie("username")
        .map(|cookie| cookie.value().to_string());

    let template = AddCreditTemplate {
        session_id: session_id.clone(),
        username: username.unwrap_or_else(|| "Guest".to_string()),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

#[derive(Deserialize)]
pub struct AddCreditForm {
    pub transaction_type: String,
    pub amount: i64,
    pub source: String,
    pub ref_num: String,
    pub status: String,
    pub description: Option<String>,
}

pub async fn add_credit(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    form: web::Form<AddCreditForm>,
) -> impl Responder {
    let username = req
        .cookie("username")
        .map(|cookie| cookie.value().to_string());

    let mut conn = pool.get().expect("Couldn't get DB connection from pool");

    // Retrieve user_id using username
    let user_id: i32 = users::table
        .filter(users::username.eq(username.unwrap_or_else(|| "Guest".to_string())))
        .select(users::id)
        .first(&mut conn)
        .expect("Error loading user_id");

    // Create a new credit transaction entry
    let new_transaction = NewCreditTransaction {
        user_id: user_id,
        transaction_type: form.transaction_type.clone(),
        amount: form.amount,
        source: form.source.clone(),
        ref_num: form.ref_num.clone(),
        status: form.status.clone(),
        description: form.description.clone(),
    };

    // Start a new transaction
    let result = conn.transaction::<_, diesel::result::Error, _>(|conn| {
        // Insert the transaction into the database
        diesel::insert_into(credit_transactions::table)
            .values(&new_transaction)
            .execute(conn)?;

        // Update the user's credit in the users table
        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(users::credit.eq(users::credit + form.amount))
            .execute(conn)?;

        Ok(())
    });

    match result {
        Ok(_) => {
            HttpResponse::SeeOther()
                .append_header(("LOCATION", "/add_credit")) // Redirect to credit transactions page
                .finish()
        }
        Err(e) => {
            eprintln!("Error inserting credit transaction: {:?}", e);
            HttpResponse::InternalServerError().body("Error adding credit transaction")
        }
    }
}

// Define the model for inserting a new credit transaction
#[derive(Insertable)]
#[table_name = "credit_transactions"]
pub struct NewCreditTransaction {
    pub user_id: i32,
    pub transaction_type: String,
    pub amount: i64,
    pub source: String,
    pub ref_num: String,
    pub status: String,
    pub description: Option<String>,
}
