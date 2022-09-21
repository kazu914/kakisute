use std::process;

use anyhow::{anyhow, Ok, Result};

use crate::{
    data_dir::DataDir,
    kakisute_file::KakisuteFile,
    kakisute_list::{single_query::SingleQuery, KakisuteList},
    operation,
};

pub struct App {
    data_dir: DataDir,
    kakisute_list: KakisuteList,
}

impl App {
    pub fn new(data_dir_arg: Option<String>) -> Self {
        let data_dir = DataDir::setup(data_dir_arg);
        let kakisute_list = KakisuteList::from_dir(data_dir.read_dir());
        App {
            data_dir,
            kakisute_list,
        }
    }

    pub fn create_kakisute(&self, file_name: Option<String>) -> Result<String> {
        let kakisute = KakisuteFile::new(file_name);
        operation::edit(&self.data_dir, kakisute.file_name())?;
        Ok(kakisute.file_name().to_string())
    }

    pub fn get_kakisute_by_index(&self, index: usize) -> Result<&KakisuteFile> {
        let kakisute = self.kakisute_list.get(index);
        if let Some(kakisute) = kakisute {
            Ok(kakisute)
        } else {
            Err(anyhow!("Could not get the content"))
        }
    }

    pub fn edit_by_index(&self, index: usize) -> Result<&str> {
        let kakisute = self.get_kakisute_by_index(index)?;
        operation::edit(&self.data_dir, kakisute.file_name())?;
        Ok(kakisute.file_name())
    }

    pub fn delete_by_index(&self, index: usize) -> Result<&str> {
        let kakisute = self.get_kakisute_by_index(index)?;
        operation::delete(&self.data_dir, kakisute.file_name())?;
        Ok(kakisute.file_name())
    }
    pub fn get_contetent_by_index(&self, index: usize) -> Result<String> {
        let kakisute = self.get_kakisute_by_index(index)?;
        let content = operation::get_content(&self.data_dir, kakisute.file_name())?;
        Ok(content)
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

    pub fn inspect_by_index(&self, index: usize) -> Result<String> {
        let kakisute = self.get_kakisute_by_index(index)?;
        Ok(self
            .data_dir
            .join(kakisute.file_name())
            .to_string_lossy()
            .to_string())
    }

    pub fn inspect_by_query(&self, query: SingleQuery) -> Result<String> {
        let index = self.get_index_by_single_query(query);
        self.inspect_by_index(index)
    }

    fn get_index_by_single_query(&self, query: SingleQuery) -> usize {
        self.kakisute_list
            .get_matching_index(query)
            .unwrap_or_else(|| {
                eprintln!("Can not find one matching the query");
                process::exit(1)
            })
    }
}

impl AppTrait for App {
    fn create_kakisute(&self, file_name: Option<String>) -> Result<String> {
        let kakisute = KakisuteFile::new(file_name);
        operation::edit(&self.data_dir, kakisute.file_name())?;
        Ok(kakisute.file_name().to_string())
    }

    fn get_kakisute_by_index(&self, index: usize) -> Result<&KakisuteFile> {
        let kakisute = self.kakisute_list.get(index);
        if let Some(kakisute) = kakisute {
            Ok(kakisute)
        } else {
            Err(anyhow!("Could not get the content"))
        }
    }

    fn edit_by_index(&self, index: usize) -> Result<&str> {
        let kakisute = self.get_kakisute_by_index(index)?;
        operation::edit(&self.data_dir, kakisute.file_name())?;
        Ok(kakisute.file_name())
    }

    fn delete_by_index(&self, index: usize) -> Result<&str> {
        let kakisute = self.get_kakisute_by_index(index)?;
        operation::delete(&self.data_dir, kakisute.file_name())?;
        Ok(kakisute.file_name())
    }

    fn get_contetent_by_index(&self, index: usize) -> Result<String> {
        let kakisute = self.get_kakisute_by_index(index)?;
        let content = operation::get_content(&self.data_dir, kakisute.file_name())?;
        Ok(content)
    }
    fn reload(&mut self) {
        self.kakisute_list = KakisuteList::from_dir(self.data_dir.read_dir());
    }

    fn get_kakisute_list(&self) -> Vec<KakisuteFile> {
        self.kakisute_list.get_list()
    }
}

pub trait AppTrait {
    fn create_kakisute(&self, file_name: Option<String>) -> Result<String>;
    fn get_kakisute_by_index(&self, index: usize) -> Result<&KakisuteFile>;
    fn edit_by_index(&self, index: usize) -> Result<&str>;
    fn delete_by_index(&self, index: usize) -> Result<&str>;
    fn get_contetent_by_index(&self, index: usize) -> Result<String>;
    fn reload(&mut self);
    fn get_kakisute_list(&self) -> Vec<KakisuteFile>;
}
