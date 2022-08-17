use crate::kakisute_file::KakisuteFile;

use super::{single_query::SingleQuery, KakisuteList};

impl KakisuteList {
    pub fn single_select(&self, query: SingleQuery) -> Option<&KakisuteFile> {
        let is_latest = query.is_latest;
        let file_name = query.file_name;
        match file_name {
            Some(file_name) => self.get_by_file_name(&file_name),
            None => {
                if is_latest {
                    self.get_latest()
                } else {
                    println!("edit expect a file name or \"--latest\" flag");
                    None
                }
            }
        }
    }
    fn get_latest(&self) -> Option<&KakisuteFile> {
        self.files.iter().max()
    }

    fn get_by_file_name(&self, file_name: &str) -> Option<&KakisuteFile> {
        self.files.iter().find(|file| file.file_name() == file_name)
    }
}
