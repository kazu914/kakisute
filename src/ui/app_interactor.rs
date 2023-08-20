use crate::service::ServiceTrait;
use crate::ui::components::filtered_list::FilteredList;
use crate::ui::components::text_input::TextInput;
use crate::ui::display_data::Info;
use anyhow::Result;
use std::cell::RefCell;
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
    new_filename: TextInput,
    search_query: TextInput,
    exit: bool,
    service: &'a dyn ServiceTrait,
    filtered_list: FilteredList,
    cached_content: RefCell<HashMap<String, String>>,
    kakisute_name_list: Vec<String>,
}

impl<'a> AppInteractor<'a> {
    pub fn new(service: &'a dyn ServiceTrait) -> Self {
        let kakisute_name_list = service.get_kakisute_list();
        let filtered_list = FilteredList::new(kakisute_name_list.len());
        AppInteractor {
            mode: Mode::Normal,
            new_filename: TextInput::new(),
            search_query: TextInput::new(),
            exit: false,
            service,
            filtered_list,
            kakisute_name_list,
            cached_content: RefCell::new(HashMap::new()),
        }
    }

    fn load_all_kakisute(&self) {
        for i in 0..self.kakisute_name_list.len() {
            self.load_kakisute_content(i);
        }
    }

    fn get_current_text_input(&self) -> Result<&TextInput> {
        Ok(match self.mode {
            Mode::Insert => &self.new_filename,
            Mode::Search => &self.search_query,
            _ => return Err(anyhow::anyhow!("")),
        })
    }

    pub fn reload(&mut self) -> Result<()> {
        self.service.reload();
        self.kakisute_name_list = self.service.get_kakisute_list();
        self.filtered_list = FilteredList::new(self.kakisute_name_list.len());
        self.new_filename.clear();
        self.search_query.clear();
        self.mode = Mode::Normal;
        self.cached_content = RefCell::new(HashMap::new());
        Ok(())
    }

    pub fn filter(&mut self) -> Result<()> {
        self.load_all_kakisute();
        self.filtered_list.filter(
            &self.search_query.get(),
            self.kakisute_name_list
                .iter()
                .map(|s| s.to_owned() + self.cached_content.borrow().get(s).unwrap().as_str())
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

    pub fn clear_text_input(&self) {
        let text_input = self.get_current_text_input().unwrap();
        text_input.clear();
    }

    pub fn get_selected_kakisute_content(&self) -> Option<String> {
        let index = self.filtered_list.get_original_index().ok()?;
        self.load_kakisute_content(index)
    }

    fn load_kakisute_content(&self, index: usize) -> Option<String> {
        if let Some(content) = self.load_from_cache(index) {
            return Some(content);
        }

        let content = self.service.get_content_by_index(index).ok();
        let kakisute_name = self.kakisute_name_list.get(index)?;
        if let Some(content) = &content {
            self.cached_content
                .borrow_mut()
                .insert(kakisute_name.clone(), content.clone());
        }
        content
    }

    fn load_from_cache(&self, index: usize) -> Option<String> {
        let kakisute_name = self.kakisute_name_list.get(index)?;
        let binding = self.cached_content.borrow();
        binding.get(kakisute_name).cloned()
    }

    pub fn edit_kakisute(&self) -> Result<String> {
        self.filtered_list
            .get_original_index()
            .and_then(|index| self.service.edit_by_index(index))
    }

    pub fn create_new_kakisute_with_file_name(&self) -> Result<()> {
        self.service
            .create_kakisute(Some(&self.new_filename.get()))?;
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

    pub fn push_text_input(&self, c: char) {
        let text_input = self.get_current_text_input().unwrap();
        text_input.push(c);
    }

    pub fn pop_text_input(&self) {
        let text_input = self.get_current_text_input().unwrap();
        text_input.pop();
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
            new_filename: self.new_filename.get(),
            search_query: self.search_query.get(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AppInteractor, Mode, ServiceTrait};
    use crate::service::kakisute_list::KakisuteList;
    use anyhow::Result;

    impl ServiceMock {
        fn new(kakisute_list: KakisuteList) -> Self {
            ServiceMock { kakisute_list }
        }
    }

    struct ServiceMock {
        kakisute_list: KakisuteList,
    }

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

    #[test]
    fn test_empty_app_interactor() {
        let mut service = ServiceMock::new(KakisuteList::new());
        let app_interactor = AppInteractor::new(&mut service);
        assert!(!app_interactor.is_kakisute_selected());
        assert!(app_interactor.edit_kakisute().is_err());
        assert!(app_interactor.delete_kakisute().is_err());
        assert!(app_interactor.get_selected_kakisute_content().is_none());
        assert_eq!(app_interactor.mode, Mode::Normal);
        assert_eq!(app_interactor.new_filename.get(), "");
        assert!(!app_interactor.exit);
    }

    #[test]
    fn test_mode_switch() {
        let mut service = ServiceMock::new(KakisuteList::new());
        let mut app_interactor = AppInteractor::new(&mut service);
        app_interactor.enter_mode(Mode::Insert);
        assert_eq!(app_interactor.mode, Mode::Insert);
        app_interactor.enter_mode(Mode::Normal);
        assert_eq!(app_interactor.mode, Mode::Normal);
        app_interactor.enter_mode(Mode::DeleteConfirm);
        assert_eq!(app_interactor.mode, Mode::DeleteConfirm);
    }
}
