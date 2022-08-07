mod edit_service;
mod list_service;
mod new_service;

use crate::{data_dir::DataDir, kakisute_list::KakisuteList};

pub struct App {
    data_dir: DataDir,
    kakisute_list: Option<KakisuteList>,
}

impl App {
    pub fn new(data_dir: Option<String>) -> Self {
        App {
            data_dir: DataDir::setup(data_dir),
            kakisute_list: None,
        }
    }
}
