mod auth;
mod model;
mod repositories;
mod schema;

use crate::auth::BasicAuth;
use crate::model::{NewRustacean, Rustacean};
use crate::repositories::RustaceanRepositories;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{catch, catchers, delete, get, post, put, routes};
use rocket_sync_db_pools::database;
use serde_json::{json, Value};

#[database("sqlite_db")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Value {
    // json!([{ "id": 1, "name": "John Doe" }, { "id": 2, "name": "Jane Doe"}])
    db.run(|c| {
        let rustaceans =
            RustaceanRepositories::find_all(c, 1000).expect("Error loading rustaceans");
        json!(rustaceans)
    })
    .await
}

#[get("/rustaceans/<id>")]
async fn get_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> Value {
    db.run(move |c| {
        let rustacean = RustaceanRepositories::find(c, id).expect("Error loading rustaceans");
        json!(rustacean)
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustaceans>")]
async fn create_rustacean(
    _auth: BasicAuth,
    db: DbConn,
    new_rustaceans: Json<NewRustacean>,
) -> Value {
    db.run(|c| {
        let result = RustaceanRepositories::create(c, new_rustaceans.into_inner())
            .expect("Error saving new rustacean");
        json!(result)
    })
    .await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(
    id: i32,
    _auth: BasicAuth,
    db: DbConn,
    rustacean: Json<Rustacean>,
) -> Value {
    db.run(move |c| {
        let rustacean_inner = rustacean.into_inner();
        let result = RustaceanRepositories::save(c, id, rustacean_inner)
            .expect("Error updating rustacean");
        json!(result)
    })
    .await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> status::NoContent {
    db.run(move |c| {
        RustaceanRepositories::delete(c, id).expect("Error deleting rustacean");
        status::NoContent
    })
    .await
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
        .register(
            "/",
            catchers![not_found, unauthorized, unprocessable_entity],
        )
        .attach(DbConn::fairing())
        .launch()
        .await;
}
