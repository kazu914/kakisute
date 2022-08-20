use std::process::Command;

use crate::data_dir::DataDir;

pub fn edit(data_dir: &DataDir, file_name: &str) {
    let file_path = data_dir.join(file_name);
    scrawl::edit(file_path).unwrap();
}

pub fn show(data_dir: &DataDir, file_name: &str) {
    let file_path = data_dir.join(file_name);
    Command::new("cat")
        .arg(file_path)
        .spawn()
        .expect("Not found cat command");
}
