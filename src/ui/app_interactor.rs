use crate::service::ServiceTrait;
use crate::ui::filtered_list::FilteredList;
use anyhow::Result;
use std::collections::HashMap;
use std::result::Result::Ok;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Mode {
    Normal,
    Insert,
    DeleteConfirm,
    Search,
}

pub struct AppInteractor<'a> {
    mode: Mode,
    user_input: String,
    exit: bool,
    service: &'a dyn ServiceTrait,
    filtered_list: FilteredList,
    cached_content: HashMap<String, String>,
    kakisute_name_list: Vec<String>,
}

impl<'a> AppInteractor<'a> {
    pub fn new(service: &'a dyn ServiceTrait) -> Self {
        let kakisute_name_list = service.get_kakisute_list();
        let filtered_list = FilteredList::new(kakisute_name_list.len());
        AppInteractor {
            mode: Mode::Normal,
            user_input: String::new(),
            exit: false,
            service,
            filtered_list,
            kakisute_name_list,
            cached_content: HashMap::new(),
        }
    }

    fn load_all_kakisute(&mut self) {
        for i in 0..self.kakisute_name_list.len() {
            self.load_kakisute_content(i);
        }
    }

    pub fn reload(&mut self) -> Result<()> {
        self.service.reload();
        self.kakisute_name_list = self.service.get_kakisute_list();
        self.filtered_list = FilteredList::new(self.kakisute_name_list.len());
        self.mode = Mode::Normal;
        self.cached_content = HashMap::new();
        Ok(())
    }

    pub fn filter(&mut self) -> Result<()> {
        self.load_all_kakisute();
        self.filtered_list.filter(
            &self.user_input,
            self.kakisute_name_list
                .iter()
                .map(|s| s.to_owned() + self.cached_content.get(s).unwrap().as_str())
                .collect::<Vec<String>>(),
        )
    }

    pub fn enter_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn select_next(&mut self) {
        self.filtered_list.select_next();
    }
    pub fn select_previous(&mut self) {
        self.filtered_list.select_previous();
    }
    pub fn select_next_n(&mut self, n: u16) {
        self.filtered_list.select_next_n(n);
    }
    pub fn select_previous_n(&mut self, n: u16) {
        self.filtered_list.select_previous_n(n);
    }

    pub fn clear_user_input(&mut self) {
        self.user_input.clear();
    }

    pub fn get_selected_kakisute_content(&mut self) -> Option<String> {
        let index = self.filtered_list.get_original_index().ok()?;
        self.load_kakisute_content(index)
    }

    fn load_kakisute_content(&mut self, index: usize) -> Option<String> {
        // cached_content にあればそれを返す
        let kakisute_name = self.kakisute_name_list.get(index)?;
        let cached_content = self.cached_content.get(kakisute_name);
        if let Some(content) = cached_content {
            return Some(content.clone());
        }

        // なければ service から取得して返す
        let content = self.service.get_content_by_index(index).ok();
        if let Some(content) = &content {
            self.cached_content
                .insert(kakisute_name.clone(), content.clone());
        }
        content
    }

    pub fn edit_kakisute(&self) -> Result<String> {
        self.filtered_list
            .get_original_index()
            .and_then(|index| self.service.edit_by_index(index))
    }

    pub fn create_new_kakisute_with_file_name(&self) -> Result<()> {
        self.service.create_kakisute(Some(&self.user_input))?;
        Ok(())
    }

    pub fn create_new_kakisute(&self) -> Result<()> {
        self.service.create_kakisute(None)?;
        Ok(())
    }

    pub fn delete_kakisute(&self) -> Result<String> {
        self.filtered_list
            .get_original_index()
            .and_then(|index| self.service.delete_by_index(index))
    }

    pub fn is_kakisute_selected(&self) -> bool {
        self.filtered_list.is_some()
    }

    pub fn get_mode(&self) -> &Mode {
        &self.mode
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

    pub fn generate_info(&mut self) -> Info {
        let content = self.get_selected_kakisute_content();
        let kakisute_name_list = self.filtered_list.get_kakisute_file_name_list(
            self.kakisute_name_list.iter().map(|s| s.as_str()).collect(),
        );
        Info {
            index: self.filtered_list.get_index().ok(),
            mode: self.mode,
            kakisute_list: kakisute_name_list,
            content,
            user_input: &self.user_input,
        }
    }
}

#[cfg(test)]
use speculate::speculate;

#[cfg(test)]
use crate::service::kakisute_list::KakisuteList;

use super::display_data::Info;

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
    fn create_kakisute(&self, _: Option<&str>) -> Result<String> {
        Ok("Ok".to_string())
    }
    fn edit_by_index(&self, _: usize) -> Result<String> {
        Ok("ok".to_string())
    }
    fn delete_by_index(&self, _: usize) -> Result<String> {
        Ok("ok".to_string())
    }
    fn get_content_by_index(&self, _: usize) -> Result<String> {
        Ok("Ok".to_string())
    }

    fn reload(&self) {}

    fn get_kakisute_list(&self) -> Vec<String> {
        self.kakisute_list.get_kakisute_file_name_list()
    }
}

#[cfg(test)]
speculate! {
    describe "empty app_interactor" {
        before {
            let mut service = ServiceMock::new(KakisuteList::new());
            let mut app_interactor = AppInteractor::new(&mut service);
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
            app_interactor.enter_mode(Mode::Insert);
            assert_eq!(app_interactor.mode, Mode::Insert)
        }

        it "enter_normal_mode should make mode insert" {
            app_interactor.enter_mode(Mode::Insert);
            app_interactor.enter_mode(Mode::Normal);
            assert_eq!(app_interactor.mode, Mode::Normal)
        }

        it "enter_delete_mode should make mode delete" {
            app_interactor.enter_mode(Mode::DeleteConfirm);
            assert_eq!(app_interactor.mode, Mode::DeleteConfirm)
        }
    }
}
