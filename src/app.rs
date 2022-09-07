mod delete_service;
mod edit_service;
mod inspect_service;
mod list_service;
mod new_service;
mod show_service;

use crate::{
    data_dir::DataDir, kakisute_file::KakisuteFile, kakisute_list::KakisuteList, operation,
};

pub struct App {
    data_dir: DataDir,
    kakisute_list: KakisuteList,
}

impl App {
    pub fn new(data_dir_arg: Option<String>) -> Self {
        let data_dir = DataDir::setup(data_dir_arg);
        let kakisute_list = KakisuteList::from_dir(data_dir.read_dir());
        App {
            data_dir,
            kakisute_list,
        }
    }

    pub fn reload(&mut self) {
        self.kakisute_list = KakisuteList::from_dir(self.data_dir.read_dir());
    }

    pub fn get_kakisute_list(&self) -> Vec<KakisuteFile> {
        self.kakisute_list.get_list()
    }

    pub fn get_kakisute(&self, index: usize) -> Option<&KakisuteFile> {
        self.kakisute_list.get(index)
    }

    pub fn edit_by_index(&self, index: usize) {
        let kakisute = self.get_kakisute(index);
        match kakisute {
            Some(kakisute) => {
                operation::edit(&self.data_dir, kakisute.file_name());
            }
            None => {}
        }
    }

    pub fn delete_by_index(&self, index: usize) {
        let kakisute = self.get_kakisute(index);
        match kakisute {
            Some(kakisute) => {
                operation::delete(&self.data_dir, kakisute.file_name());
            }
            None => {}
        }
    }
    pub fn get_kakisute_contetent(&self, index: usize) -> Option<String> {
        let kakisute = self.get_kakisute(index);
        match kakisute {
            Some(kakisute) => operation::get_content(&self.data_dir, kakisute.file_name()),
            None => None,
        }
    }
}
