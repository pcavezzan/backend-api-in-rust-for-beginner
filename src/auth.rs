use base64::Engine;
use rocket::http::Status;
use rocket::Request;
use rocket::request::Outcome;

pub struct BasicAuth {
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
                if basic_auth.username == "foo" && basic_auth.password == "bar" {
                    return Outcome::Success(basic_auth);
                }
            }
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}
