use crate::data_dir::DataDir;

pub fn edit(data_dir: &DataDir, file_name: &str) {
    let file_path = data_dir.join(file_name);
    let _output = scrawl::edit(file_path).unwrap();
}
