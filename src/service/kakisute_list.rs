use kakisute_file::KakisuteFile;
use std::rc::Rc;
use std::{cell::RefCell, fs::ReadDir};

use super::search_query::SingleQuery;
mod kakisute_file;

#[derive(Clone, Debug)]
pub struct KakisuteList {
    ref_files: Rc<RefCell<Vec<KakisuteFile>>>,
}

impl Default for KakisuteList {
    fn default() -> Self {
        Self::new()
    }
}

impl KakisuteList {
    pub fn new() -> Self {
        KakisuteList {
            ref_files: Rc::new(RefCell::new(vec![])),
        }
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

    pub fn reload(&self, read_dir: ReadDir) {
        let kakisute_list = Self::from_dir(read_dir);
        *self.borrow_mut() = kakisute_list.ref_files.borrow().clone();
    }

    pub fn len(&self) -> usize {
        self.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn sort(&self) {
        self.borrow_mut().sort();
    }

    fn add(&mut self, file: KakisuteFile) {
        self.borrow_mut().push(file);
    }

    pub fn get_kakisute_file_name_list(&self) -> Vec<String> {
        self.borrow()
            .iter()
            .map(|kakisute| kakisute.file_name().to_string())
            .collect()
    }

    fn borrow_mut(&self) -> std::cell::RefMut<'_, Vec<KakisuteFile>> {
        self.ref_files.borrow_mut()
    }

    fn borrow(&self) -> std::cell::Ref<'_, Vec<KakisuteFile>> {
        self.ref_files.borrow()
    }

    pub fn get_file_name_by_index(&self, index: usize) -> Option<String> {
        self.borrow()
            .get(index)
            .map(|kakisute| kakisute.file_name().to_string())
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
        self.borrow_mut()
            .iter()
            .position(|file| file.file_name() == file_name)
    }

    fn get_last_index(&self) -> usize {
        self.borrow_mut().len() - 1
    }
}
