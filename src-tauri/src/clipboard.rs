use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClipboardContent {
    pub content: String,
    pub clipboard_type: ClipboardType,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClipboardType {
    Text,
    Image,
}

impl ClipboardType {
    pub fn as_str(&self) -> String {
        match self {
            &ClipboardType::Image => "Image".to_owned(),
            &ClipboardType::Text => "Text".to_owned(),
        }
    }
}

impl From<ClipboardContent> for serde_json::Value {
    fn from(content: ClipboardContent) -> serde_json::Value {
        serde_json::to_value(content).unwrap()
    }
}

impl PartialEq for ClipboardType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
