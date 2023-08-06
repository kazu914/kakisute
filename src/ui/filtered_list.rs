use anyhow::Result;
use grep::regex::RegexMatcher;
use grep::searcher::sinks::UTF8;
use grep::searcher::Searcher;

use crate::service::ServiceTrait;
use crate::ui::list_index::ListIndex;

pub struct FilteredList {
    list_index: ListIndex,
    kakisute_name_list: Vec<String>,
    filtered_indexes: Vec<usize>,
}

impl FilteredList {
    pub fn new(service: &dyn ServiceTrait) -> Self {
        let kakisute_list = service.get_kakisute_list();
        let filtered_indexes: Vec<usize> = (0..kakisute_list.len()).collect();
        let index = ListIndex::new(filtered_indexes.len());
        FilteredList {
            list_index: index,
            kakisute_name_list: kakisute_list,
            filtered_indexes,
        }
    }

    pub fn select_next(&mut self) {
        self.list_index.increment();
    }

    pub fn select_previous(&mut self) {
        self.list_index.decrement();
    }

    pub fn select_next_n(&mut self, n: u16) {
        self.list_index.increment_n(n);
    }

    pub fn select_previous_n(&mut self, n: u16) {
        self.list_index.decrement_n(n);
    }

    pub fn get_index(&self) -> Result<usize> {
        self.list_index.get_index()
    }

    pub fn is_some(&self) -> bool {
        self.list_index.is_some()
    }

    pub fn get_kakisute_file_name_list(&self) -> Vec<&str> {
        self.filtered_indexes
            .iter()
            .filter_map(|&index| self.kakisute_name_list.get(index))
            .map(|s| s.as_str())
            .collect()
    }

    pub fn filter(&mut self, user_input: &str) -> Result<()> {
        let matcher = RegexMatcher::new(user_input)?;
        let mut matches: Vec<u64> = vec![];
        Searcher::new().search_slice(
            &matcher,
            self.kakisute_name_list.join("\n").as_bytes(),
            UTF8(|lnum, _| {
                matches.push(lnum - 1);
                Ok(true)
            }),
        )?;
        self.filtered_indexes = matches.iter().map(|x| *x as usize).collect();
        self.list_index = ListIndex::new(self.filtered_indexes.len());
        Ok(())
    }
}

#[cfg(test)]
use crate::service::kakisute_list::KakisuteList;
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
    describe "empty filtered_list" {
        before {
            let mut service = ServiceMock::new(KakisuteList::new());
            let filtered_list = FilteredList::new(&mut service);
        }

        it "index should be None" {
            assert!(filtered_list.list_index.is_none())
        }

        it "should start with empty items" {
            assert_eq!(filtered_list.kakisute_name_list.len(), 0)
        }
    }
}
