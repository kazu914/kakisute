use std::fs;

use crate::data_dir::DataDir;

pub fn edit(data_dir: &DataDir, file_name: &str) {
    let file_path = data_dir.join(file_name);
    scrawl::edit(file_path).unwrap();
}

pub fn show(data_dir: &DataDir, file_name: &str) {
    let file_path = data_dir.join(file_name);
    let contents = fs::read_to_string(file_path).unwrap();
    println!("{}", contents);
}