use chrono::DateTime;
use chrono::Local;
use std::process;

use anyhow::{Ok, Result};

use crate::datetime_helper::datetime_to_string;
use crate::domain::kakisute::Kakisute;

use self::interface::IRepository;
use self::kakisute_list::KakisuteList;
use self::search_query::SingleQuery;

pub struct Service<'a> {
    kakisute_list: KakisuteList,
    repository: &'a dyn IRepository,
}

pub mod interface;
pub mod kakisute_list;
pub mod search_query;

impl<'a> Service<'a> {
    pub fn new(repository: &'a dyn IRepository) -> Self {
        let kakisute_list = KakisuteList::from_dir(repository.read_dir());
        Service {
            kakisute_list,
            repository,
        }
    }

    pub fn delete_by_single_query(&self, query: SingleQuery) -> Result<&str> {
        let index = self.get_index_by_single_query(query);
        self.delete_by_index(index)
    }

    pub fn get_content_by_single_query(&self, query: SingleQuery) -> Result<String> {
        let index = self.get_index_by_single_query(query);
        self.get_contetent_by_index(index)
    }

    pub fn edit_by_single_query(&self, query: SingleQuery) -> Result<&str> {
        let index = self.get_index_by_single_query(query);
        self.edit_by_index(index)
    }

    pub fn inspect_by_query(&self, query: SingleQuery) -> Result<String> {
        let index = self.get_index_by_single_query(query);
        self.inspect_by_index(index)
    }

    fn inspect_by_index(&self, index: usize) -> Result<String> {
        let file_name = self.kakisute_list.get_file_name_by_index(index)?;
        self.repository.get_path(file_name)
    }

    fn get_index_by_single_query(&self, query: SingleQuery) -> usize {
        self.kakisute_list
            .get_matching_index(query)
            .unwrap_or_else(|| {
                eprintln!("Can not find one matching the query");
                process::exit(1)
            })
    }

    fn get_kakisute(&self, index: usize) -> Result<Kakisute> {
        let file_name = self.kakisute_list.get_file_name_by_index(index)?;
        let content = self.repository.get_content(file_name)?;
        Ok(Kakisute::new(content))
    }

    fn generate_file_name(date: DateTime<Local>, file_name: Option<String>) -> String {
        let prefix = datetime_to_string(date);
        if let Some(file_name) = file_name {
            prefix + "_" + &file_name
        } else {
            prefix + ".txt"
        }
    }
}

impl ServiceTrait for Service<'_> {
    fn create_kakisute(&self, file_name: Option<String>) -> Result<String> {
        let created_at = Local::now();
        let file_name = Service::generate_file_name(created_at, file_name);
        self.repository.edit(&file_name)?;
        Ok(file_name)
    }

    fn edit_by_index(&self, index: usize) -> Result<&str> {
        let file_name = self.kakisute_list.get_file_name_by_index(index)?;
        self.repository.edit(file_name)?;
        Ok(file_name)
    }

    fn delete_by_index(&self, index: usize) -> Result<&str> {
        let file_name = self.kakisute_list.get_file_name_by_index(index)?;
        self.repository.delete(file_name)?;
        Ok(file_name)
    }

    fn get_contetent_by_index(&self, index: usize) -> Result<String> {
        let kakisute = self.get_kakisute(index)?;
        Ok(kakisute.content())
    }
    fn reload(&mut self) {
        self.kakisute_list = KakisuteList::from_dir(self.repository.read_dir());
    }

    fn get_kakisute_list(&self) -> KakisuteList {
        self.kakisute_list.clone()
    }
}

pub trait ServiceTrait {
    fn create_kakisute(&self, file_name: Option<String>) -> Result<String>;
    fn edit_by_index(&self, index: usize) -> Result<&str>;
    fn delete_by_index(&self, index: usize) -> Result<&str>;
    fn get_contetent_by_index(&self, index: usize) -> Result<String>;
    fn reload(&mut self);
    fn get_kakisute_list(&self) -> KakisuteList;
}

#[cfg(test)]
extern crate speculate;
#[cfg(test)]
use speculate::speculate;

#[cfg(test)]
speculate! {
    use chrono::TimeZone;
    describe "generate_file_name" {

        before {
            let date = Local.ymd(2022,1,10).and_hms(16,30,15);;
        }

        it "joins datetime and given file_name" {
            let file_name = Service::generate_file_name(date,Some("test.sql".to_string()));
            assert_eq!(file_name,"2022_01_10_16_30_15_test.sql")
        }

        it "use only datetime if file_name is not given" {
            let file_name = Service::generate_file_name(date,None);
            assert_eq!(file_name,"2022_01_10_16_30_15.txt")
        }
    }
}
