use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub account: String,
    pub body: String,
    pub timestamp: i64,
    pub synced: bool,
}
