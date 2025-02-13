mod auth;
mod schema;
mod model;

use diesel::prelude::*;
use rocket::response::status;
use rocket::{catch, catchers, delete, get, post, put, routes};
use rocket::serde::json::Json;
use rocket_sync_db_pools::database;
use serde_json::{json, Value};
use crate::auth::BasicAuth;
use crate::model::{NewRustacean, Rustacean};
use crate::schema::rustaceans;

#[database("sqlite_db")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Value {
    // json!([{ "id": 1, "name": "John Doe" }, { "id": 2, "name": "Jane Doe"}])
    db.run(|c|{
        let rustaceans = rustaceans::table
            .order(rustaceans::id.desc())
            .limit(1000)
            .load::<Rustacean>(c)
            .expect("Error loading rustaceans");
        json!(rustaceans)
    }).await
}

#[get("/rustaceans/<id>")]
async fn get_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> Value {
    db.run(move |c|{
        let rustacean = rustaceans::table
            .find(id)
            .get_result::<Rustacean>(c)
        .expect("Error loading rustaceans");
        json!(rustacean)
    }).await
}

#[post("/rustaceans", format = "json", data = "<new_rustaceans>")]
async fn create_rustacean(_auth: BasicAuth, db: DbConn, new_rustaceans: Json<NewRustacean>) -> Value {
    db.run(|c|{
        let result = diesel::insert_into(rustaceans::table)
        .values(&new_rustaceans.into_inner())
        .execute(c)
        .expect("Error saving new rustacean");
        json!(result)
    }).await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(id: i32, _auth: BasicAuth, db: DbConn, rustacean: Json<Rustacean>) -> Value {
    db.run(move |c|{
        let rustacean_inner = rustacean.into_inner();
        let result = diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::name.eq(rustacean_inner.name),
                rustaceans::email.eq(rustacean_inner.email),
            ))
            .execute(c)
            .expect("Error updating rustacean");
        json!(result)
    }).await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> status::NoContent {
    db.run(move |c|{
        diesel::delete(rustaceans::table.find(id))
            .execute(c)
            .expect("Error deleting rustacean");
        status::NoContent
    }).await
}

#[catch(404)]
fn not_found() -> Value {
    json!({ "message": "Not Found" })
}

#[catch(401)]
fn unauthorized() -> Value {
    json!({ "message": "Unauthorized" })
}

#[catch(422)]
fn unprocessable_entity() -> Value {
    json!({
        "message": "Unprocessable Entity",
        "details": "The request was well-formed but was unable to be followed due to semantic errors."
    })
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                get_rustaceans,
                get_rustacean,
                create_rustacean,
                update_rustacean,
                delete_rustacean
            ],
        )
        .register("/", catchers![not_found, unauthorized, unprocessable_entity])
        .attach(DbConn::fairing())
        .launch()
        .await;
}
