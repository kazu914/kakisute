use std::process::Command;

use crate::kakisute_list::single_query::SingleQuery;

use super::App;

impl App {
    pub fn show(&self, query: SingleQuery) {
        let kakisute = self.kakisute_list.single_select(query);

        match kakisute {
            Some(kakisute) => {
                let path = self
                    .data_dir
                    .join(kakisute.file_name())
                    .into_os_string()
                    .into_string()
                    .unwrap();
                Command::new("cat")
                    .arg(path)
                    .spawn()
                    .expect("Not found cat command");
            }
            None => {
                println!("Can not find one matching the query");
            }
        }
    }
}
