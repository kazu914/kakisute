use std::process::Command;

use super::App;

impl App {
    pub fn show(&self, is_latest: bool, file_name: Option<String>) {
        let kakisute = self.kakisute_list.single_select(is_latest, file_name);

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
