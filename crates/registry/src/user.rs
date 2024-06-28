use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserList {
    pub users: Vec<User>,
}

impl From<Vec<User>> for UserList {
    fn from(users: Vec<User>) -> Self {
        Self { users }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRequest {
    pub users: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserResponse {
    pub ok: bool,
    pub msg: String,
}

impl From<&str> for UserResponse {
    fn from(msg: &str) -> Self {
        Self {
            ok: true,
            msg: msg.to_string(),
        }
    }
}
