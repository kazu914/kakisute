use crate::kakisute_file::KakisuteFile;

use super::KakisuteList;

impl KakisuteList {
    pub fn get_latest(&self) -> &KakisuteFile {
        self.files.iter().max().unwrap()
    }

    pub fn get_by_file_name(&self, file_name: &str) -> Option<&KakisuteFile> {
        self.files.iter().find(|file| file.file_name() == file_name)
    }
}
