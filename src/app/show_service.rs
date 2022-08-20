use crate::{kakisute_list::single_query::SingleQuery, operation};

use super::App;

impl App {
    pub fn show(&self, query: SingleQuery) {
        let kakisute = self.kakisute_list.single_select(query);

        match kakisute {
            Some(kakisute) => {
                operation::show(&self.data_dir, kakisute.file_name());
            }
            None => {
                println!("Can not find one matching the query");
            }
        }
    }
}
