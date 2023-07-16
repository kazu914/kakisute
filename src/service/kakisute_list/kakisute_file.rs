pub use crate::datetime_helper::string_to_datetime;
use std::path::Path;

use chrono::{DateTime, Local};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct KakisuteFile {
    file_name: String,
    created_at: Option<DateTime<Local>>,
}

impl KakisuteFile {
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn from_path(path: &Path) -> Option<Self> {
        if !path.is_file() {
            return None;
        };

        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        let created_at = string_to_datetime(&file_name);

        created_at.single().map(|created_at| KakisuteFile {
            file_name,
            created_at: Some(created_at),
        })
    }
}
