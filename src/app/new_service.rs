use crate::{editor, kakisute_file::KakisuteFile};

use super::App;

impl App {
    pub fn create_kakisute(&self, file_name: Option<String>) {
        let kakisute_file = KakisuteFile::new(file_name);
        editor::edit(&self.data_dir, &kakisute_file.file_name());
    }
}
