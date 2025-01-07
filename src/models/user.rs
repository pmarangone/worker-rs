use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserModel {
    pub name: String,
    pub surname: String,
    pub description: String,
    pub age: i32,
}
