use base64;
use base64::Engine;
use rocket::http::Status;
use rocket::request::Outcome;
use rocket::response::status;
use rocket::{catch, catchers, delete, get, post, put, routes, Request};
use serde_json::{json, Value};

struct BasicAuth {
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    fn from_authorization_header(header: &str) -> Option<BasicAuth> {
        let split = header.splitn(2, ' ').collect::<Vec<&str>>();
        if split.len() != 2 {
            return None;
        }
        let auth_type = split[0].to_string();
        if auth_type != "Basic" {
            return None;
        }

        let auth_value = split[1].to_string();
        Self::from_base64_encoded(&auth_value)
    }

    fn from_base64_encoded(base64_encoded: &str) -> Option<BasicAuth> {
        let decoded = base64::engine::general_purpose::STANDARD.decode(base64_encoded).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;

        let parts: Vec<&str> = decoded_str.splitn(2, ':').collect();
        if parts.len() != 2 {
            return None;
        }
        let (username, password) = (parts[0].to_string(), parts[1].to_string());

        Some(BasicAuth { username, password })
    }
}

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header_value) = auth_header {
            if let Some(basic_auth) = BasicAuth::from_authorization_header(auth_header_value) {
                return Outcome::Success(basic_auth);
            }
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}

#[get("/rustaceans")]
fn get_rustaceans(auth: BasicAuth) -> Value {
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
        .launch()
        .await;
}
