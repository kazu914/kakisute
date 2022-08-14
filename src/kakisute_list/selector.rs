use crate::kakisute_file::KakisuteFile;

use super::KakisuteList;

impl KakisuteList {
    pub fn single_select(
        &self,
        is_latest: bool,
        file_name: Option<String>,
    ) -> Option<&KakisuteFile> {
        match file_name {
            Some(file_name) => self.get_by_file_name(&file_name),
            None => {
                if is_latest {
                    self.get_latest()
                } else {
                    println!("edit expect a file name or \"--latest\" flag");
                    None
                }
            }
        }
    }
    fn get_latest(&self) -> Option<&KakisuteFile> {
        self.files.iter().max()
    }

    fn get_by_file_name(&self, file_name: &str) -> Option<&KakisuteFile> {
        self.files.iter().find(|file| file.file_name() == file_name)
    }
}
