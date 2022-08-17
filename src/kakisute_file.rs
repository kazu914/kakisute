use std::path::PathBuf;

use chrono::{DateTime, Local, LocalResult, NaiveDateTime, TimeZone};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct KakisuteFile {
    file_name: String,
    created_at: Option<DateTime<Local>>,
}

const DATE_FORMAT: &str = "%Y_%m_%d_%H_%M_%S";
const DATE_FORMAT_LENGTH: usize = 19;

impl KakisuteFile {
    pub fn new(file_name: Option<String>) -> Self {
        let created_at = Local::now();
        let file_name = KakisuteFile::generate_file_name(created_at, file_name);
        KakisuteFile {
            file_name,
            created_at: None,
        }
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn from_path(path: &PathBuf) -> Option<Self> {
        if !path.is_file() {
            return None;
        };

        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        let created_at = KakisuteFile::get_created_at_from_file_name(&file_name);

        created_at.single().map(|created_at| KakisuteFile {
                file_name,
                created_at: Some(created_at),
            })
    }

    fn generate_file_name(date: DateTime<Local>, file_name: Option<String>) -> String {
        let prefix = date.format(DATE_FORMAT).to_string();
        if let Some(file_name) = file_name {
            prefix + "_" + &file_name
        } else {
            prefix + ".txt"
        }
    }

    fn get_created_at_from_file_name(file_name: &str) -> LocalResult<DateTime<Local>> {
        if file_name.chars().count() < DATE_FORMAT_LENGTH {
            return LocalResult::None;
        };

        let prefix_date = &file_name[0..DATE_FORMAT_LENGTH];

        let created_at = NaiveDateTime::parse_from_str(prefix_date, DATE_FORMAT);
        if let Ok(created_at) = created_at {
            Local.from_local_datetime(&created_at)
        } else {
            LocalResult::None
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

        it "joins datetime and given file_name" {
            let file_name = KakisuteFile::generate_file_name(date,Some("test.sql".to_string()));
            assert_eq!(file_name,"2022_01_10_16_30_15_test.sql")
        }

        it "use only datetime if file_name is not given" {
            let file_name = KakisuteFile::generate_file_name(date,None);
            assert_eq!(file_name,"2022_01_10_16_30_15.txt")
        }
    }

    describe "get_created_at_from_file_name" {
        it "return local date when format is correct" {
            let date = Local.ymd(2022,1,10).and_hms(16,30,15);;
            let file_name = "2022_01_10_16_30_15_test.sql";
            let created_at = KakisuteFile::get_created_at_from_file_name(file_name);
            assert_eq!(created_at.single(),Some(date));
        }

        it "return none when file_name is shorter than fomrat" {
            let file_name = "test.sql";
            let created_at = KakisuteFile::get_created_at_from_file_name(file_name);
            assert_eq!(created_at.single(),None);
        }

        it "return local date when format is wrong" {
            let file_name = "2022_01_10_16_30_test.sql";
            let created_at = KakisuteFile::get_created_at_from_file_name(file_name);
            assert_eq!(created_at.single(),None);
        }

        it "return local date when date time is invalid" {
            let file_name = "2022_13_10_16_30_test.sql";
            let created_at = KakisuteFile::get_created_at_from_file_name(file_name);
            assert_eq!(created_at.single(),None);
        }
    }
}
