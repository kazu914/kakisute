use crate::service::kakisute_file::KakisuteFile;
use std::fs::ReadDir;
mod selector;
pub mod single_query;

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

    pub fn get(&self, index: usize) -> Option<&KakisuteFile> {
        self.files.get(index)
    }

    pub fn get_kakisute_file_name_list(&self) -> Vec<&str> {
        self.files
            .iter()
            .map(|kakisute| kakisute.file_name())
            .collect()
    }
}
