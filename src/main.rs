mod auth;
mod model;
mod repositories;
mod schema;

use crate::auth::BasicAuth;
use crate::model::{NewRustacean, Rustacean};
use crate::repositories::RustaceanRepositories;
use diesel::NotFound;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::response::status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{catch, catchers, delete, get, post, put, routes, Build, Rocket};
use rocket_sync_db_pools::database;
use serde_json::{json, Value};

#[database("sqlite_db")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceanRepositories::find_all(c, 1000)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[get("/rustaceans/<id>")]
async fn get_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepositories::find(c, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|e| match e {
                NotFound => Custom(Status::NotFound, json!(e.to_string())),
                _ => Custom(Status::InternalServerError, json!(e.to_string())),
            })
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustaceans>")]
async fn create_rustacean(
    _auth: BasicAuth,
    db: DbConn,
    new_rustaceans: Json<NewRustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceanRepositories::create(c, new_rustaceans.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(
    id: i32,
    _auth: BasicAuth,
    db: DbConn,
    rustacean: Json<Rustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        let rustacean_inner = rustacean.into_inner();
        RustaceanRepositories::save(c, id, rustacean_inner)
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(
    id: i32,
    _auth: BasicAuth,
    db: DbConn,
) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepositories::find(c, id)
            .map(|_| RustaceanRepositories::delete(c, id))
            .map(|_| status::NoContent)
            .map_err(|e| match e {
                NotFound => Custom(Status::NotFound, json!(e.to_string())),
                _ => Custom(Status::InternalServerError, json!(e.to_string())),
            })
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

async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
    DbConn::get_one(&rocket)
        .await
        .expect("Unable to retrieve database connection")
        .run(|c| {
            c.run_pending_migrations(MIGRATIONS)
                .expect("Migrations failed");
        })
        .await;
    rocket
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
        .attach(AdHoc::on_ignite("Diesel migrations", run_db_migrations))
        .launch()
        .await;
}
