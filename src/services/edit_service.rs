use crate::data_dir::DataDir;
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
        let kakisute_list = KakisuteList::from_dir(self.data_dir.read_dir());
        kakisute_list.print_list();
    }

    fn edit_latest(&self, kakisute_list: KakisuteList) {
        let file_path = self.data_dir.join(&kakisute_list.get_latest().file_name());
        let _utput = scrawl::edit(file_path).unwrap();
    }
    fn edit_by_filename(&self, kakisute_list: KakisuteList, filename: String) {
        let kakisute = kakisute_list.get_by_filename(&filename);
        match kakisute {
            Some(kakiste) => {
                let file_path = self.data_dir.join(kakiste.file_name());
                let _utput = scrawl::edit(file_path).unwrap();
            }
            None => {
                println!("Can not find: {}", filename);
            }
        }
    }
}
