use std::path::PathBuf;

use chrono::{DateTime, Local};
pub struct KakisuteFile {
    file_name: String,
}

impl KakisuteFile {
    pub fn new(filename: Option<String>) -> Self {
        let file_name = KakisuteFile::generate_file_name(Local::now(), filename);
        KakisuteFile { file_name }
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn from_path(path: &PathBuf) -> Option<Self> {
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
            Some(KakisuteFile { file_name })
        } else {
            None
        }
    }

    fn generate_file_name(date: DateTime<Local>, filename: Option<String>) -> String {
        let prefix = date.format("%Y_%m_%d_%H_%M_%S").to_string();
        if let Some(filename) = filename {
            prefix + "_" + &filename
        } else {
            prefix + ".txt"
        }
    }
}

#[cfg(test)]
extern crate speculate;
#[cfg(test)]
use speculate::speculate;

#[cfg(test)]
speculate! {
    use chrono::TimeZone;
    describe "generate_file_name" {

        before {
            let date = Local.ymd(2022,1,10).and_hms(16,30,15);;
        }

        it "joins datetime and given filename" {
            let file_name = KakisuteFile::generate_file_name(date,Some("test.sql".to_string()));
            assert_eq!(file_name,"2022_01_10_16_30_15_test.sql")
        }

        it "use only datetime if filename is not given" {
            let file_name = KakisuteFile::generate_file_name(date,None);
            assert_eq!(file_name,"2022_01_10_16_30_15.txt")
        }
    }
}
