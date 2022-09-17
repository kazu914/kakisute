use crate::app::App;
use crate::kakisute_file::KakisuteFile;

pub enum Mode {
    Normal,
    Insert,
    DeleteConfirm,
}

pub struct Tui {
    pub selected_list_index: Option<usize>,
    pub items: Vec<KakisuteFile>,
    pub mode: Mode,
    pub new_file_name: String,
    pub exit: bool,
}

impl Default for Tui {
    fn default() -> Self {
        Tui {
            selected_list_index: Some(0),
            items: vec![],
            mode: Mode::Normal,
            new_file_name: String::new(),
            exit: false,
        }
    }
}

impl Tui {
    pub fn new(kakisute_file_list: Vec<KakisuteFile>) -> Self {
        let index = if kakisute_file_list.is_empty() {
            None
        } else {
            Some(0)
        };
        Tui {
            selected_list_index: index,
            items: kakisute_file_list,
            mode: Mode::Normal,
            new_file_name: String::new(),
            exit: false,
        }
    }

    pub fn reload(&mut self, kakisute_file_list: Vec<KakisuteFile>) {
        let index = if kakisute_file_list.is_empty() {
            None
        } else {
            Some(0)
        };

        self.selected_list_index = index;
        self.items = kakisute_file_list;
        self.mode = Mode::Normal;
    }
    pub fn enter_insert_mode(&mut self) {
        self.mode = Mode::Insert;
    }
    pub fn enter_normal_mode(&mut self) {
        self.mode = Mode::Normal;
    }
    pub fn enter_delete_mode(&mut self) {
        self.mode = Mode::DeleteConfirm;
    }
    pub fn select_next(&mut self) {
        match self.selected_list_index {
            Some(n) => {
                if n != self.items.len() - 1 {
                    self.selected_list_index = Some(n + 1);
                } else {
                    self.selected_list_index = Some(0);
                }
            }
            None => {}
        }
    }
    pub fn select_previous(&mut self) {
        match self.selected_list_index {
            Some(n) => {
                if n > 0 {
                    self.selected_list_index = Some(n - 1);
                } else {
                    self.selected_list_index = Some(self.items.len() - 1);
                }
            }
            None => {}
        }
    }
    pub fn clear_file_name(&mut self) {
        self.new_file_name = String::new();
    }

    pub fn get_selected_kakisute_content(&self, app: &App) -> Option<String> {
        match self.selected_list_index {
            Some(index) => app.get_contetent_by_index(index).ok(),
            None => None,
        }
    }
}
