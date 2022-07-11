use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FriendsResponse {
    pub success: bool,
    pub records: Vec<Friend>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Friend {
    #[serde(rename = "uuidSender")]
    pub uuid_sender: String,
    #[serde(rename = "uuidReceiver")]
    pub uuid_receiver: String,
    pub started: i64,
}
