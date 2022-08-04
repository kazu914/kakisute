use crate::data_dir::DataDir;
use crate::kakisute_list::KakisuteList;

pub struct ListService<'a> {
    data_dir: &'a DataDir,
}

impl<'a> ListService<'a> {
    pub fn new(data_dir: &'a DataDir) -> Self {
        ListService { data_dir }
    }

    pub fn list(&self) {
        let kakisute_list = KakisuteList::from_dir(self.data_dir.read_dir());
        kakisute_list.print_list();
    }
}
