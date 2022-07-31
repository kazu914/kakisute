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

#[cfg(test)]
extern crate speculate;
#[cfg(test)]
use speculate::speculate;

#[cfg(test)]
speculate! {
    use chrono::TimeZone;
    describe "generate_base_name" {

        before {
            let date = Local.ymd(2022,1,10).and_hms(16,30,15);;
        }

        it "joins datetime and given filename" {
            let base_name = KakisuteFile::generate_base_name(date,Some("test.sql".to_string()));
            assert_eq!(base_name,"2022_01_10_16_30_15_test.sql")
        }

        it "use only datetime if filename is not given" {
            let base_name = KakisuteFile::generate_base_name(date,None);
            assert_eq!(base_name,"2022_01_10_16_30_15.txt")
        }
    }
}
