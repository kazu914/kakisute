use std::usize;

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
                    println!("File name or \"--latest\" flag is required.");
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

    pub fn get_matching_index(&self, query: SingleQuery) -> Option<usize> {
        let is_latest = query.is_latest;
        let file_name = query.file_name;
        match file_name {
            Some(file_name) => self.get_index_by_file_name(&file_name),
            None => {
                if is_latest {
                    Some(self.get_latest_index())
                } else {
                    println!("File name or \"--latest\" flag is required.");
                    None
                }
            }
        }
    }

    fn get_latest_index(&self) -> usize {
        self.files.len()
    }

    fn get_index_by_file_name(&self, file_name: &str) -> Option<usize> {
        self.files
            .iter()
            .position(|file| file.file_name() == file_name)
    }
}
