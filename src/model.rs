use diesel::Queryable;
use serde::Serialize;

#[derive(Serialize, Queryable, PartialEq, Debug)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String
}