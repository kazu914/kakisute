use crate::data_dir::DataDir;
use crate::editor;
use crate::kakisute_file::KakisuteFile;

pub struct NewService<'a> {
    data_dir: &'a DataDir,
}

impl<'a> NewService<'a> {
    pub fn new(data_dir: &'a DataDir) -> Self {
        NewService { data_dir }
    }

    pub fn create(&self, file_name: Option<String>) {
        let kakisute_file = KakisuteFile::new(file_name);
        editor::edit(self.data_dir, &kakisute_file.file_name());
    }
}
