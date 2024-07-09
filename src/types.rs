use serde::{Serialize, Deserialize};

use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub fname: String,
    pub lname: String,
    pub email: String
}

impl User {
    pub fn new(fname: &str, lname: &str, email: &str) -> User {
        let id = Uuid::new_v4();
        User { id: id.to_string(), fname: fname.to_string(), lname: lname.to_string(), email: email.to_string() }
    }

}