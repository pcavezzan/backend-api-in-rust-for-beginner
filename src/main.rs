use rocket::response::status;
use rocket::{delete, get, post, put, routes};
use serde_json::{json, Value};
#[get("/rustaceans")]
fn get_rustaceans() -> Value {
    json!([{ "id": 1, "name": "John Doe" }, { "id": 2, "name": "Jane Doe"}])
}

#[get("/rustaceans/<id>")]
fn get_rustacean(id: u32) -> Value {
    json!({ "id": id, "name": "John Doe", "email": "john@doe.com" })
}

#[post("/rustaceans", format = "json")]
fn create_rustacean() -> Value {
    json!({ "id": 3, "name": "John Doe", "email": "john@doe.com" })
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: u32) -> Value {
    json!({ "id": id, "name": "John Doe", "email": "john@doe.com" })
}

#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: u32) -> status::NoContent {
    status::NoContent
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
        .launch()
        .await;
}
