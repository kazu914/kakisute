use crate::kakisute_list::single_query::SingleQuery;

use super::App;

impl App {
    pub fn inspect(&self, query: SingleQuery) {
        let kakisute = self.kakisute_list.single_select(query);

        match kakisute {
            Some(kakisute) => {
                println!(
                    "{}",
                    self.data_dir.join(kakisute.file_name()).to_string_lossy()
                );
            }
            None => {
                println!("Can not find one matching the query");
            }
        }
    }
}
