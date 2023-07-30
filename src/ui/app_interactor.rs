use crate::service::kakisute_list::KakisuteList;
use crate::service::ServiceTrait;
use crate::ui::list_index::ListIndex;
use anyhow::Result;
use std::result::Result::Ok;

#[derive(Eq, PartialEq, Debug)]
pub enum Mode {
    Normal,
    Insert,
    DeleteConfirm,
    Search,
}

pub struct AppInteractor<'a> {
    selected_list_index: ListIndex,
    items: KakisuteList,
    mode: Mode,
    user_input: String,
    exit: bool,
    service: &'a mut dyn ServiceTrait,
}

impl<'a> AppInteractor<'a> {
    pub fn new(service: &'a mut dyn ServiceTrait) -> Self {
        let kakisute_list = service.get_kakisute_list();
        let index = ListIndex::new(kakisute_list.len());
        AppInteractor {
            selected_list_index: index,
            items: kakisute_list,
            mode: Mode::Normal,
            user_input: String::new(),
            exit: false,
            service,
        }
    }

    pub fn reload(&mut self) {
        self.service.reload();
        let kakisute_file_list = self.service.get_kakisute_list();
        let index = ListIndex::new(kakisute_file_list.len());
        self.selected_list_index = index;
        self.items = kakisute_file_list;
        self.mode = Mode::Normal;
    }

    pub fn filter(&mut self) {}

    pub fn enter_insert_mode(&mut self) {
        self.mode = Mode::Insert;
    }
    pub fn enter_normal_mode(&mut self) {
        self.mode = Mode::Normal;
    }
    pub fn enter_delete_mode(&mut self) {
        self.mode = Mode::DeleteConfirm;
    }
    pub fn enter_search_mode(&mut self) {
        self.mode = Mode::Search;
    }
    pub fn select_next(&mut self) {
        self.selected_list_index.increment();
    }
    pub fn select_previous(&mut self) {
        self.selected_list_index.decrement();
    }
    pub fn select_next_n(&mut self, n: u16) {
        self.selected_list_index.increment_n(n);
    }
    pub fn select_previous_n(&mut self, n: u16) {
        self.selected_list_index.decrement_n(n);
    }
    pub fn clear_user_input(&mut self) {
        self.user_input = String::new();
    }

    pub fn get_selected_kakisute_content(&self) -> Option<String> {
        let index = self.selected_list_index.get_index();
        if let Ok(index) = index {
            self.service.get_content_by_index(index).ok()
        } else {
            None
        }
    }

    pub fn edit_kakisute(&self) -> Result<()> {
        let index = self.selected_list_index.get_index()?;
        self.service.edit_by_index(index)?;
        Ok(())
    }

    pub fn create_new_kakisute_with_file_name(&self) -> Result<()> {
        self.service
            .create_kakisute(Some(self.user_input.clone()))?;
        Ok(())
    }

    pub fn create_new_kakisute(&self) -> Result<()> {
        self.service.create_kakisute(None)?;
        Ok(())
    }

    pub fn delete_kakisute(&self) -> Result<()> {
        let index = self.selected_list_index.get_index()?;
        self.service.delete_by_index(index)?;
        Ok(())
    }

    pub fn is_kakisute_selected(&self) -> bool {
        self.selected_list_index.is_some()
    }

    pub fn get_selected_index(&self) -> Option<usize> {
        self.selected_list_index.get_index().ok()
    }

    pub fn get_kakisute_file_name_list(&self) -> Vec<&str> {
        self.items.get_kakisute_file_name_list()
    }

    pub fn get_mode(&self) -> &Mode {
        &self.mode
    }

    pub fn get_user_input(&self) -> &str {
        &self.user_input
    }

    pub fn push_user_input(&mut self, c: char) {
        self.user_input.push(c)
    }

    pub fn pop_user_input(&mut self) {
        self.user_input.pop();
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
struct ServiceMock {
    kakisute_list: KakisuteList,
}

#[cfg(test)]
impl ServiceMock {
    fn new(kakisute_list: KakisuteList) -> Self {
        ServiceMock { kakisute_list }
    }
}

#[cfg(test)]
impl ServiceTrait for ServiceMock {
    fn create_kakisute(&self, _: Option<String>) -> Result<String> {
        Ok("Ok".to_string())
    }
    fn edit_by_index(&self, _: usize) -> Result<&str> {
        Ok("ok")
    }
    fn delete_by_index(&self, _: usize) -> Result<&str> {
        Ok("ok")
    }
    fn get_content_by_index(&self, _: usize) -> Result<String> {
        Ok("Ok".to_string())
    }

    fn reload(&mut self) {}

    fn get_kakisute_list(&self) -> KakisuteList {
        self.kakisute_list.clone()
    }
}

#[cfg(test)]
speculate! {
    describe "empty app_interactor" {
        before {
            let mut service = ServiceMock::new(KakisuteList::new());
            let app_interactor = AppInteractor::new(&mut service);
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

        it "should start with normal mode" {
            assert_eq!(app_interactor.mode, Mode::Normal)
        }

        it "should start with empty user input" {
            assert_eq!(app_interactor.user_input, "")
        }

        it "should start with exit false" {
            assert!(!app_interactor.exit)
        }
    }

    describe "app_interactor mode function" {
        before {
            let mut service = ServiceMock::new(KakisuteList::new());
            let mut app_interactor = AppInteractor::new(&mut service);
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
