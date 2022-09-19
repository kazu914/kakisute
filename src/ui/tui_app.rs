use crate::app::App;
use crate::kakisute_file::KakisuteFile;
use anyhow::{Context, Ok, Result};

#[derive(Eq, PartialEq, Debug)]
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

    pub fn edit_kakisute(&self, app: &App) -> Result<()> {
        let index = self
            .selected_list_index
            .context("Received edit without selecting kakisute index")?;
        app.edit_by_index(index)?;
        Ok(())
    }

    pub fn create_new_kakisute_with_file_name(&self, app: &App) -> Result<()> {
        app.create_kakisute(Some(self.new_file_name.clone()))?;
        Ok(())
    }

    pub fn create_new_kakisute(&self, app: &App) -> Result<()> {
        app.create_kakisute(None)?;
        Ok(())
    }
}

#[cfg(test)]
use speculate::speculate;

#[cfg(test)]
speculate! {
    describe "empty tui" {
        before {
            let kakisute_file_list: Vec<KakisuteFile> = vec![];
            let tui = Tui::new(kakisute_file_list);
        }

        it "index should be None" {
            assert_eq!(tui.selected_list_index, None)
        }

        it "should start with empty items" {
            assert_eq!(tui.items.len(), 0)
        }

        it "return error if edit is called" {
            let app = App::new(None);
            let res = tui.edit_kakisute(&app).is_err();
            assert!(res)

        }
    }


    describe "tui with multiple kakisute" {
        before {
            let kakisute_file_list: Vec<KakisuteFile> = vec![
                KakisuteFile::new(Some("file1".to_string())),
                KakisuteFile::new(Some("file2".to_string())),
                KakisuteFile::new(Some("file3".to_string())),
            ];
            let tui = Tui::new(kakisute_file_list);
        }

        it "index should be Some(0)" {
            assert_eq!(tui.selected_list_index, Some(0))
        }

        it "should start with normal mode" {
            assert_eq!(tui.mode, Mode::Normal)
        }

        it "should start with given items" {
            assert_eq!(tui.items.len(), 3);
            assert!(tui.items[0].file_name().contains("file1"));
            assert!(tui.items[1].file_name().contains("file2"));
            assert!(tui.items[2].file_name().contains("file3"));
        }

        it "should start with empty file name" {
            assert_eq!(tui.new_file_name, "")
        }

        it "should start with exit false" {
            assert!(!tui.exit)
        }
    }

    describe "tui mode function" {
        before {
            let kakisute_file_list: Vec<KakisuteFile> = vec![
                KakisuteFile::new(Some("file1".to_string())),
                KakisuteFile::new(Some("file2".to_string())),
                KakisuteFile::new(Some("file3".to_string())),
            ];
            let mut tui = Tui::new(kakisute_file_list);
        }

        it "enter_insert_mode should make mode insert" {
            tui.enter_insert_mode();
            assert_eq!(tui.mode, Mode::Insert)
        }

        it "enter_normal_mode should make mode insert" {
            tui.enter_insert_mode();
            tui.enter_normal_mode();
            assert_eq!(tui.mode, Mode::Normal)
        }

        it "enter_delete_mode should make mode delete" {
            tui.enter_delete_mode();
            assert_eq!(tui.mode, Mode::DeleteConfirm)
        }
    }

    describe "index function" {
        before {
            let kakisute_file_list: Vec<KakisuteFile> = vec![
                KakisuteFile::new(Some("file1".to_string())),
                KakisuteFile::new(Some("file2".to_string())),
                KakisuteFile::new(Some("file3".to_string())),
            ];
            let mut tui = Tui::new(kakisute_file_list);
        }

        it "select_next shold increment index" {
            tui.select_next();
            assert_eq!(tui.selected_list_index,Some(1))
        }

        it "select_next shold return 0 when index was max" {
            tui.select_next();
            tui.select_next();
            tui.select_next();
            assert_eq!(tui.selected_list_index,Some(0))
        }

        it "select_previous shold return max when index was 0" {
            tui.select_previous();
            assert_eq!(tui.selected_list_index,Some(2))
        }

        it "select_previous shold decrease index" {
            tui.select_previous();
            tui.select_previous();
            assert_eq!(tui.selected_list_index,Some(1))
        }
    }
}
