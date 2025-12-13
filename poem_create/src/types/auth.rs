use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct DefaultMessage {
    pub message: String
}

#[derive(Deserialize, Serialize)]
pub struct UserLoginPayload {
    pub user_id: String,
    pub password: String
}