use chrono::DateTime;
use chrono::Local;
use grep::cli;
use grep::printer::{ColorSpecs, StandardBuilder};
use grep::regex::RegexMatcher;
use std::process;

use anyhow::{anyhow, Ok, Result};
use core::result::Result::Ok as CoreOk;
use grep::searcher::Searcher;
use termcolor::ColorChoice;
use walkdir::WalkDir;

use crate::datetime_helper::datetime_to_string;
use crate::domain::kakisute::Kakisute;

use self::interface::IRepository;
use self::kakisute_list::KakisuteList;
use self::search_query::SingleQuery;

pub struct Service<'a> {
    kakisute_list: &'a KakisuteList,
    repository: &'a dyn IRepository,
}

pub mod interface;
pub mod kakisute_list;
pub mod search_query;

impl<'a> Service<'a> {
    pub fn new(repository: &'a dyn IRepository, kakisute_list: &'a KakisuteList) -> Self {
        Service {
            kakisute_list,
            repository,
        }
    }

    pub fn delete_by_single_query(&self, query: SingleQuery) -> Result<String> {
        let index = self.get_index_by_single_query(query);
        self.delete_by_index(index)
    }

    pub fn get_content_by_single_query(&self, query: SingleQuery) -> Result<String> {
        let index = self.get_index_by_single_query(query);
        self.get_content_by_index(index)
    }

    pub fn edit_by_single_query(&self, query: SingleQuery) -> Result<String> {
        let index = self.get_index_by_single_query(query);
        self.edit_by_index(index)
    }

    pub fn inspect_by_query(&self, query: SingleQuery) -> Result<String> {
        let index = self.get_index_by_single_query(query);
        self.inspect_by_index(index)
    }

    fn inspect_by_index(&self, index: usize) -> Result<String> {
        if let Some(file_name) = self.kakisute_list.get_file_name_by_index(index) {
            let path = self.repository.get_path(&file_name)?;
            Ok(path)
        } else {
            Err(anyhow!("File not found"))
        }
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
        if let Some(file_name) = self.kakisute_list.get_file_name_by_index(index) {
            let content = self.repository.get_content(&file_name)?;
            Ok(Kakisute::new(content))
        } else {
            Err(anyhow!("File not found"))
        }
    }

    fn generate_file_name(date: DateTime<Local>, file_name: Option<String>) -> String {
        let prefix = datetime_to_string(date);
        if let Some(file_name) = file_name {
            prefix + "_" + &file_name
        } else {
            prefix + ".txt"
        }
    }
    pub fn search_cli(&self, word: &str) -> Result<()> {
        let matcher = RegexMatcher::new(word)?;
        let mut searcher = Searcher::new();
        let mut printer = StandardBuilder::new()
            .color_specs(ColorSpecs::default_with_color())
            .build(cli::stdout(if cli::is_tty_stdout() {
                ColorChoice::Auto
            } else {
                ColorChoice::Never
            }));

        for file_name in self.kakisute_list.get_kakisute_file_name_list() {
            let path = self.repository.get_path(&file_name)?;
            for result in WalkDir::new(path) {
                let dent = match result {
                    CoreOk(dent) => dent,
                    Err(err) => {
                        eprintln!("{}", err);
                        continue;
                    }
                };
                if !dent.file_type().is_file() {
                    continue;
                }
                let result = searcher.search_path(
                    &matcher,
                    dent.path(),
                    printer.sink_with_path(&matcher, dent.path()),
                );
                if let Err(err) = result {
                    eprintln!("{}: {}", dent.path().display(), err);
                }
            }
        }

        Ok(())
    }
}

impl ServiceTrait for Service<'_> {
    fn create_kakisute(&self, file_name: Option<String>) -> Result<String> {
        let created_at = Local::now();
        let file_name = Service::generate_file_name(created_at, file_name);
        self.repository.edit(&file_name)?;
        Ok(file_name)
    }

    fn edit_by_index(&self, index: usize) -> Result<String> {
        if let Some(file_name) = self.kakisute_list.get_file_name_by_index(index) {
            self.repository.edit(&file_name)?;
            Ok(file_name)
        } else {
            Err(anyhow!("File not found"))
        }
    }

    fn delete_by_index(&self, index: usize) -> Result<String> {
        if let Some(file_name) = self.kakisute_list.get_file_name_by_index(index) {
            self.repository.delete(&file_name)?;
            Ok(file_name)
        } else {
            Err(anyhow!("File not found"))
        }
    }

    fn get_content_by_index(&self, index: usize) -> Result<String> {
        let kakisute = self.get_kakisute(index)?;
        Ok(kakisute.content())
    }
    fn reload(&self) {
        self.kakisute_list.reload(self.repository.read_dir());
    }

    fn get_kakisute_list(&self) -> Vec<String> {
        self.kakisute_list.get_kakisute_file_name_list()
    }
}

pub trait ServiceTrait {
    fn create_kakisute(&self, file_name: Option<String>) -> Result<String>;
    fn edit_by_index(&self, index: usize) -> Result<String>;
    fn delete_by_index(&self, index: usize) -> Result<String>;
    fn get_content_by_index(&self, index: usize) -> Result<String>;
    fn reload(&self);
    fn get_kakisute_list(&self) -> Vec<String>;
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
