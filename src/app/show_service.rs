use std::process::Command;

use super::App;
use crate::kakisute_file::KakisuteFile;

impl App {
    pub fn show(&self, is_latest: bool, file_name: Option<String>) {
        match file_name {
            Some(file_name) => {
                self.show_by_file_name(file_name);
            }
            None => {
                if is_latest {
                    self.show_latest();
                } else {
                    println!("edit expect a file name or \"--latest\" flag")
                }
            }
        }
    }

    fn show_by_file_name(&self, file_name: String) {
        let kakisute = self.kakisute_list.get_by_file_name(&file_name);
        match kakisute {
            Some(kakisute) => self.show_path(kakisute),
            None => {
                println!("Can not find: {}", file_name);
            }
        }
    }
    fn show_latest(&self) {
        let kakisute = &self.kakisute_list.get_latest();
        self.show_path(kakisute)
    }

    fn show_path(&self, kakisute: &KakisuteFile) {
        let path = self
            .data_dir
            .join(kakisute.file_name())
            .into_os_string()
            .into_string()
            .unwrap();
        Command::new("cat").arg(path).spawn().expect("Not found cat command");
    }
}
