use anyhow::{Ok, Result};
use kakisute_file::KakisuteFile;
use std::fs::ReadDir;

use super::search_query::SingleQuery;
mod kakisute_file;

#[derive(Clone, Debug)]
pub struct KakisuteList {
    files: Vec<KakisuteFile>,
}

impl Default for KakisuteList {
    fn default() -> Self {
        Self::new()
    }
}

impl KakisuteList {
    pub fn new() -> Self {
        KakisuteList { files: vec![] }
    }

    pub fn from_dir(read_dir: ReadDir) -> Self {
        let mut kakisute_list = Self::new();

        for file in read_dir {
            let kakisute_file = KakisuteFile::from_path(&file.unwrap().path());
            if let Some(kakisute_file) = kakisute_file {
                kakisute_list.add(kakisute_file);
            }
        }

        kakisute_list.sort();

        kakisute_list
    }

    pub fn len(&self) -> usize {
        self.files.len()
    }

    pub fn is_empty(&self) -> bool {
        self.files.len() == 0
    }

    fn sort(&mut self) {
        self.files.sort();
    }

    fn add(&mut self, file: KakisuteFile) {
        self.files.push(file);
    }

    pub fn get_kakisute_file_name_list(&self) -> Vec<&str> {
        self.files
            .iter()
            .map(|kakisute| kakisute.file_name())
            .collect()
    }

    pub fn get_file_name_by_index(&'_ self, index: usize) -> Result<&'_ str> {
        Ok(self.files.get(index).unwrap().file_name())
    }

    pub fn get_matching_index(&self, query: SingleQuery) -> Option<usize> {
        let is_latest = query.is_latest;
        let file_name = query.file_name;
        match file_name {
            Some(file_name) => self.get_index_by_file_name(&file_name),
            None => {
                if is_latest {
                    Some(self.get_last_index())
                } else {
                    println!("File name or \"--latest\" flag is required.");
                    None
                }
            }
        }
    }

    fn get_index_by_file_name(&self, file_name: &str) -> Option<usize> {
        self.files
            .iter()
            .position(|file| file.file_name() == file_name)
    }

    fn get_last_index(&self) -> usize {
        self.files.len() - 1
    }
}
