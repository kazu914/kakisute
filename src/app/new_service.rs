use crate::{kakisute_file::KakisuteFile, operation};

use super::App;

impl App {
    pub fn create_kakisute(&self, file_name: Option<String>) {
        let kakisute_file = KakisuteFile::new(file_name);
        operation::edit(&self.data_dir, kakisute_file.file_name());
    }
}
