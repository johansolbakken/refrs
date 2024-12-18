use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Reference {
    pub id: String,
    pub ris_path: String,
    pub attachments: Vec<String>,
}
