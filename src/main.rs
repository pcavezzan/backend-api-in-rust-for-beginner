use rocket::{get, routes};

#[get("/")]
async fn hello() -> &'static str {
    "Hello, world!\n"
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
    .mount("/", routes![hello])
    .launch()
    .await;
}
