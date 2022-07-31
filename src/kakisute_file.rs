use chrono::{DateTime, Local};
pub struct KakisuteFile {
    pub base_name: String,
}

impl KakisuteFile {
    pub fn new(filename: Option<String>) -> Self {
        let base_name = KakisuteFile::generate_base_name(Local::now(), filename);
        KakisuteFile { base_name }
    }

    fn generate_base_name(date: DateTime<Local>, filename: Option<String>) -> String {
        let prefix = date.format("%Y_%m_%d_%H_%M_%S").to_string();
        if let Some(filename) = filename {
            prefix + "_" + &filename
        } else {
            prefix + ".txt"
        }
    }
}
