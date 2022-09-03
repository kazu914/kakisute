mod edit_service;
mod inspect_service;
mod list_service;
mod new_service;
mod show_service;
mod ui;

use crate::{data_dir::DataDir, kakisute_list::KakisuteList};

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
}
