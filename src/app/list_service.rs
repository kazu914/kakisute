use crate::kakisute_list::KakisuteList;

use super::App;

impl App {
    pub fn list(&self) {
        let kakisute_list = KakisuteList::from_dir(self.data_dir.read_dir());
        kakisute_list.print_list();
    }
}
