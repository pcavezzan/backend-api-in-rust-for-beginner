mod auth;

use rocket::response::status;
use rocket::{catch, catchers, delete, get, post, put, routes};
use rocket_sync_db_pools::database;
use serde_json::{json, Value};
use crate::auth::BasicAuth;

#[database("sqlite_db")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Value {
    json!([{ "id": 1, "name": "John Doe" }, { "id": 2, "name": "Jane Doe"}])
}

#[get("/rustaceans/<id>")]
fn get_rustacean(id: u32, _auth: BasicAuth) -> Value {
    json!({ "id": id, "name": "John Doe", "email": "john@doe.com" })
}

#[post("/rustaceans", format = "json")]
fn create_rustacean(_auth: BasicAuth) -> Value {
    json!({ "id": 3, "name": "John Doe", "email": "john@doe.com" })
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: u32, _auth: BasicAuth) -> Value {
    json!({ "id": id, "name": "John Doe", "email": "john@doe.com" })
}

#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: u32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
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
