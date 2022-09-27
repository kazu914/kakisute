use crate::app::AppTrait;
use crate::kakisute_file::KakisuteFile;
use crate::ui::list_index::ListIndex;
use anyhow::Result;
use std::result::Result::Ok;

#[derive(Eq, PartialEq, Debug)]
pub enum Mode {
    Normal,
    Insert,
    DeleteConfirm,
}

pub struct AppInteractor<'a> {
    selected_list_index: ListIndex,
    items: Vec<KakisuteFile>,
    mode: Mode,
    new_file_name: String,
    exit: bool,
    app: &'a mut dyn AppTrait,
}

impl<'a> AppInteractor<'a> {
    pub fn new(app: &'a mut dyn AppTrait) -> Self {
        let kakisute_file_list = app.get_kakisute_list();
        let index = ListIndex::new(kakisute_file_list.len());
        AppInteractor {
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
        let index = ListIndex::new(kakisute_file_list.len());
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
        self.selected_list_index.increment();
    }
    pub fn select_previous(&mut self) {
        self.selected_list_index.decrement();
    }
    pub fn clear_file_name(&mut self) {
        self.new_file_name = String::new();
    }

    pub fn get_selected_kakisute_content(&self) -> Option<String> {
        let index = self.selected_list_index.get_index();
        if let Ok(index) = index {
            self.app.get_contetent_by_index(index).ok()
        } else {
            None
        }
    }

    pub fn edit_kakisute(&self) -> Result<()> {
        let index = self.selected_list_index.get_index()?;
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
        let index = self.selected_list_index.get_index()?;
        self.app.delete_by_index(index)?;
        Ok(())
    }

    pub fn is_kakisute_selected(&self) -> bool {
        self.selected_list_index.is_some()
    }

    pub fn get_selected_index(&self) -> Option<usize> {
        self.selected_list_index.get_index().ok()
    }

    pub fn get_kakisute_list(&self) -> &[KakisuteFile] {
        &self.items
    }

    pub fn get_mode(&self) -> &Mode {
        &self.mode
    }

    pub fn get_file_name(&self) -> &str {
        &self.new_file_name
    }

    pub fn push_to_file_name(&mut self, c: char) {
        self.new_file_name.push(c)
    }

    pub fn pop_from_file_name(&mut self) {
        self.new_file_name.pop();
    }

    pub fn exit(&mut self) {
        self.exit = true
    }

    pub fn is_exited(&self) -> bool {
        self.exit
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
    describe "empty app_interactor" {
        before {
            let kakisute_list: Vec<KakisuteFile> = vec![];
            let mut app = AppMock::new(kakisute_list);
            let app_interactor = AppInteractor::new(&mut app);
        }

        it "index should be None" {
            assert!(app_interactor.selected_list_index.is_none())
        }

        it "should start with empty items" {
            assert_eq!(app_interactor.items.len(), 0)
        }

        it "return error if edit is called" {
            let res = app_interactor.edit_kakisute().is_err();
            assert!(res)
        }

        it "return error if delete is called" {
            let res = app_interactor.delete_kakisute().is_err();
            assert!(res)
        }

        it "return none if get selected content is called" {
            let res = app_interactor.get_selected_kakisute_content();
            assert!(res.is_none())
        }

        it "return false if is_kakisute_selected is called" {
            let res = app_interactor.is_kakisute_selected();
            assert!(!res)
        }
    }


    describe "app_interactor with multiple kakisute" {
        before {
            let kakisute_list: Vec<KakisuteFile> = vec![
                KakisuteFile::new(Some("file1".to_string())),
                KakisuteFile::new(Some("file2".to_string())),
                KakisuteFile::new(Some("file3".to_string())),
            ];
            let mut app = AppMock::new(kakisute_list);
            let app_interactor = AppInteractor::new(&mut app);
        }

        it "index should be Some(0)" {
            assert_eq!(app_interactor.selected_list_index.get_index().unwrap(), 0)
        }

        it "should start with normal mode" {
            assert_eq!(app_interactor.mode, Mode::Normal)
        }

        it "should start with given items" {
            assert_eq!(app_interactor.items.len(), 3);
            assert!(app_interactor.items[0].file_name().contains("file1"));
            assert!(app_interactor.items[1].file_name().contains("file2"));
            assert!(app_interactor.items[2].file_name().contains("file3"));
        }

        it "should start with empty file name" {
            assert_eq!(app_interactor.new_file_name, "")
        }

        it "should start with exit false" {
            assert!(!app_interactor.exit)
        }

        it "return no error if edit is called" {
            let res = app_interactor.edit_kakisute().is_ok();
            assert!(res)

        }

        it "return no error if delete is called" {
            let res = app_interactor.delete_kakisute().is_ok();
            assert!(res)
        }

        it "return some if get selected content is called" {
            let res = app_interactor.get_selected_kakisute_content();
            assert!(res.is_some())
        }

        it "return true if is_kakisute_selected is called" {
            let res = app_interactor.is_kakisute_selected();
            assert!(res)
        }
    }

    describe "app_interactor mode function" {
        before {
            let kakisute_list: Vec<KakisuteFile> = vec![
                KakisuteFile::new(Some("file1".to_string())),
                KakisuteFile::new(Some("file2".to_string())),
                KakisuteFile::new(Some("file3".to_string())),
            ];
            let mut app = AppMock::new(kakisute_list);
            let mut app_interactor = AppInteractor::new(&mut app);
        }

        it "enter_insert_mode should make mode insert" {
            app_interactor.enter_insert_mode();
            assert_eq!(app_interactor.mode, Mode::Insert)
        }

        it "enter_normal_mode should make mode insert" {
            app_interactor.enter_insert_mode();
            app_interactor.enter_normal_mode();
            assert_eq!(app_interactor.mode, Mode::Normal)
        }

        it "enter_delete_mode should make mode delete" {
            app_interactor.enter_delete_mode();
            assert_eq!(app_interactor.mode, Mode::DeleteConfirm)
        }
    }
}
