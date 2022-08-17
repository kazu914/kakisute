use crate::{editor, kakisute_list::single_query::SingleQuery};

use super::App;

impl App {
    pub fn edit(&self, query: SingleQuery) {
        let kakisute = self.kakisute_list.single_select(query);

        match kakisute {
            Some(kakisute) => {
                editor::edit(&self.data_dir, kakisute.file_name());
            }
            None => {
                println!("Can not find one matching the query");
            }
        }
    }
}
