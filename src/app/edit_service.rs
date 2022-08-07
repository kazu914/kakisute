use crate::{editor, kakisute_list::KakisuteList};

use super::App;

impl App {
    pub fn edit(&self, is_latest: bool, file_name: Option<String>) {
        let kakisute_list = KakisuteList::from_dir(self.data_dir.read_dir());
        match file_name {
            Some(file_name) => {
                self.edit_by_file_name(kakisute_list, file_name);
            }
            None => {
                if is_latest {
                    self.edit_latest(kakisute_list);
                } else {
                    println!("edit expect a file name or \"--latest\" flag")
                }
            }
        }
    }

    fn edit_by_file_name(&self, kakisute_list: KakisuteList, file_name: String) {
        let kakisute = kakisute_list.get_by_file_name(&file_name);
        match kakisute {
            Some(kakisute) => {
                editor::edit(&self.data_dir, &kakisute.file_name());
            }
            None => {
                println!("Can not find: {}", file_name);
            }
        }
    }
    fn edit_latest(&self, kakisute_list: KakisuteList) {
        editor::edit(&self.data_dir, &kakisute_list.get_latest().file_name());
    }
}
