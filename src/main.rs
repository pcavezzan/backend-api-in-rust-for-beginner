use rocket::{get, routes};
use serde_json::{json, Value};

#[get("/")]
async fn hello() -> Value {
    json!({ "message": "Hello, world!" })
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
    .mount("/", routes![hello])
    .launch()
    .await;
}
