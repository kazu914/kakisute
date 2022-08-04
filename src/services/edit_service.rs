use crate::data_dir::DataDir;
use crate::editor;
use crate::kakisute_list::KakisuteList;

pub struct EditService<'a> {
    data_dir: &'a DataDir,
}

impl<'a> EditService<'a> {
    pub fn new(data_dir: &'a DataDir) -> Self {
        EditService { data_dir }
    }

    pub fn edit(&self, is_latest: bool, filename: Option<String>) {
        let kakisute_list = KakisuteList::from_dir(self.data_dir.read_dir());
        match filename {
            Some(filename) => {
                self.edit_by_filename(kakisute_list, filename);
            }
            None => {
                if is_latest {
                    self.edit_latest(kakisute_list);
                } else {
                    println!("edit expect a file name or \"--latest\" flag")
                }
            }
        }
    }

    fn edit_by_filename(&self, kakisute_list: KakisuteList, filename: String) {
        let kakisute = kakisute_list.get_by_filename(&filename);
        match kakisute {
            Some(kakisute) => {
                editor::edit(&self.data_dir, &kakisute.file_name());
            }
            None => {
                println!("Can not find: {}", filename);
            }
        }
    }
    fn edit_latest(&self, kakisute_list: KakisuteList) {
        editor::edit(&self.data_dir, &kakisute_list.get_latest().file_name());
    }
}
