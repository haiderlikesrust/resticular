use serde_derive::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PageData {
    pub title: String,
}