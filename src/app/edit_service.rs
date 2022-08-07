use crate::editor;

use super::App;

impl App {
    pub fn edit(&self, is_latest: bool, file_name: Option<String>) {
        match file_name {
            Some(file_name) => {
                self.edit_by_file_name(file_name);
            }
            None => {
                if is_latest {
                    self.edit_latest();
                } else {
                    println!("edit expect a file name or \"--latest\" flag")
                }
            }
        }
    }

    fn edit_by_file_name(&self, file_name: String) {
        let kakisute = self.kakisute_list.get_by_file_name(&file_name);
        match kakisute {
            Some(kakisute) => {
                editor::edit(&self.data_dir, &kakisute.file_name());
            }
            None => {
                println!("Can not find: {}", file_name);
            }
        }
    }
    fn edit_latest(&self) {
        editor::edit(&self.data_dir, &self.kakisute_list.get_latest().file_name());
    }
}
