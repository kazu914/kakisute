use crate::data_dir::DataDir;
use crate::kakisute_file::KakisuteFile;

pub struct NewService<'a> {
    data_dir: &'a DataDir,
}

impl<'a> NewService<'a> {
    pub fn new(data_dir: &'a DataDir) -> Self {
        NewService { data_dir }
    }

    pub fn create(&self, filename: Option<String>) {
        let kakisute_file = KakisuteFile::new(filename);
        let file_path = self.data_dir.join(&kakisute_file.file_name());
        let _utput = scrawl::edit(file_path).unwrap();
    }
}
