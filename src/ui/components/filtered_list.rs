use anyhow::{anyhow, Result};
use grep::matcher::LineTerminator;
use grep::regex::RegexMatcher;
use grep::searcher::sinks::UTF8;
use grep::searcher::SearcherBuilder;

use crate::ui::components::list_index::ListIndex;

pub struct FilteredList {
    list_index: ListIndex,
    filtered_indexes: Vec<usize>,
}

impl FilteredList {
    pub fn new(list_size: usize) -> Self {
        let filtered_indexes: Vec<usize> = (0..list_size).collect();
        let index = ListIndex::new(filtered_indexes.len());
        FilteredList {
            list_index: index,
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

    pub fn get_original_index(&self) -> Result<usize> {
        if self.list_index.is_none() {
            return Err(anyhow!(
                "Received get_original_index while ListIndex is None"
            ));
        }
        Ok(self.filtered_indexes[self.list_index.get_index()?])
    }

    pub fn is_some(&self) -> bool {
        self.list_index.is_some()
    }

    pub fn get_kakisute_file_name_list<'a>(
        &'a self,
        kakisute_name_list: Vec<&'a str>,
    ) -> Vec<&str> {
        self.filtered_indexes
            .iter()
            .filter_map(|&index| kakisute_name_list.get(index).copied())
            .collect()
    }

    pub fn filter(&mut self, user_input: &str, kakisute_content_list: Vec<String>) -> Result<()> {
        let matcher = RegexMatcher::new(user_input);
        if matcher.is_err() {
            return Ok(());
        }
        let mut matches: Vec<u64> = vec![];
        let mut searcher = SearcherBuilder::new()
            .line_terminator(LineTerminator::byte(b'\0'))
            .build();
        searcher.search_slice(
            &matcher?,
            kakisute_content_list.join("\0").as_bytes(),
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
use speculate::speculate;

#[cfg(test)]
speculate! {
    describe "empty filtered_list" {
        before {
            let filtered_list = FilteredList::new(0);
        }

        it "index should be None" {
            assert!(filtered_list.list_index.is_none())
        }

        it "should start with empty items" {
            assert_eq!(filtered_list.filtered_indexes.len(), 0)
        }
    }
}
