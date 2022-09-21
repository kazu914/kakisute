use crate::app::AppTrait;
use crate::kakisute_file::KakisuteFile;
use anyhow::{Context, Ok, Result};

#[derive(Eq, PartialEq, Debug)]
pub enum Mode {
    Normal,
    Insert,
    DeleteConfirm,
}

pub struct Tui<'a> {
    pub selected_list_index: Option<usize>,
    pub items: Vec<KakisuteFile>,
    pub mode: Mode,
    pub new_file_name: String,
    pub exit: bool,
    pub app: &'a mut dyn AppTrait,
}

impl<'a> Tui<'a> {
    pub fn new(app: &'a mut dyn AppTrait) -> Self {
        let kakisute_file_list = app.get_kakisute_list();
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
            app,
        }
    }

    pub fn reload(&mut self) {
        self.app.reload();
        let kakisute_file_list = self.app.get_kakisute_list();
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

    pub fn get_selected_kakisute_content(&self) -> Option<String> {
        match self.selected_list_index {
            Some(index) => self.app.get_contetent_by_index(index).ok(),
            None => None,
        }
    }

    pub fn edit_kakisute(&self) -> Result<()> {
        let index = self
            .selected_list_index
            .context("Received edit without selecting kakisute index")?;
        self.app.edit_by_index(index)?;
        Ok(())
    }

    pub fn create_new_kakisute_with_file_name(&self) -> Result<()> {
        self.app.create_kakisute(Some(self.new_file_name.clone()))?;
        Ok(())
    }

    pub fn create_new_kakisute(&self) -> Result<()> {
        self.app.create_kakisute(None)?;
        Ok(())
    }

    pub fn delete_kakisute(&self) -> Result<()> {
        let index = self
            .selected_list_index
            .context("Received edit without selecting kakisute index")?;
        self.app.delete_by_index(index)?;
        Ok(())
    }
}

#[cfg(test)]
use speculate::speculate;
#[cfg(test)]
struct AppMock {
    kakisute_list: Vec<KakisuteFile>,
}

#[cfg(test)]
impl AppMock {
    fn new(kakisute_list: Vec<KakisuteFile>) -> Self {
        AppMock { kakisute_list }
    }
}

#[cfg(test)]
impl AppTrait for AppMock {
    fn create_kakisute(&self, _: Option<String>) -> Result<String> {
        Ok("Ok".to_string())
    }
    fn get_kakisute_by_index(&self, index: usize) -> Result<&KakisuteFile> {
        Ok(self.kakisute_list.get(index).unwrap())
    }
    fn edit_by_index(&self, _: usize) -> Result<&str> {
        Ok("ok")
    }
    fn delete_by_index(&self, _: usize) -> Result<&str> {
        Ok("ok")
    }
    fn get_contetent_by_index(&self, _: usize) -> Result<String> {
        Ok("Ok".to_string())
    }

    fn reload(&mut self) {}

    fn get_kakisute_list(&self) -> Vec<KakisuteFile> {
        self.kakisute_list.clone()
    }
}

#[cfg(test)]
speculate! {
    describe "empty tui" {
        before {
            let kakisute_list: Vec<KakisuteFile> = vec![];
            let mut app = AppMock::new(kakisute_list);
            let tui = Tui::new(&mut app);
        }

        it "index should be None" {
            assert_eq!(tui.selected_list_index, None)
        }

        it "should start with empty items" {
            assert_eq!(tui.items.len(), 0)
        }

        it "return error if edit is called" {
            let res = tui.edit_kakisute().is_err();
            assert!(res)

        }
    }


    describe "tui with multiple kakisute" {
        before {
            let kakisute_list: Vec<KakisuteFile> = vec![
                KakisuteFile::new(Some("file1".to_string())),
                KakisuteFile::new(Some("file2".to_string())),
                KakisuteFile::new(Some("file3".to_string())),
            ];
            let mut app = AppMock::new(kakisute_list);
            let tui = Tui::new(&mut app);
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
            let kakisute_list: Vec<KakisuteFile> = vec![
                KakisuteFile::new(Some("file1".to_string())),
                KakisuteFile::new(Some("file2".to_string())),
                KakisuteFile::new(Some("file3".to_string())),
            ];
            let mut app = AppMock::new(kakisute_list);
            let mut tui = Tui::new(&mut app);
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
            let kakisute_list: Vec<KakisuteFile> = vec![
                KakisuteFile::new(Some("file1".to_string())),
                KakisuteFile::new(Some("file2".to_string())),
                KakisuteFile::new(Some("file3".to_string())),
            ];
            let mut app = AppMock::new(kakisute_list);
            let mut tui = Tui::new(&mut app);
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
