use crate::editor;

use super::App;

impl App {
    pub fn edit(&self, is_latest: bool, file_name: Option<String>) {
        let kakisute = self.kakisute_list.single_select(is_latest, file_name);

        match kakisute {
            Some(kakisute) => {
                editor::edit(&self.data_dir, &kakisute.file_name());
            }
            None => {
                println!("Can not find one matching the query");
            }
        }
    }
}
