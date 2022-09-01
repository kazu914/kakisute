use std::fs;

use crate::data_dir::DataDir;

pub fn edit(data_dir: &DataDir, file_name: &str) {
    let file_path = data_dir.join(file_name);
    scrawl::edit(file_path).unwrap();
}

pub fn show(data_dir: &DataDir, file_name: &str) {
    let content = get_content(data_dir, file_name);
    match content {
        Some(content) => {
            println!("{}", content);
        }
        None => {}
    }
}

pub fn get_content(data_dir: &DataDir, file_name: &str) -> Option<String> {
    let file_path = data_dir.join(file_name);
    fs::read_to_string(file_path).ok()
}
