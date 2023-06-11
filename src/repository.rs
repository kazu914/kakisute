use std::fs::{self, ReadDir};

use anyhow::{Context, Ok, Result};

use crate::service::RepositoryTrait;

use self::data_dir::DataDir;

mod data_dir;

pub struct Repository {
    data_dir: DataDir,
}

impl Repository {
    pub fn new(data_dir: Option<String>) -> Self {
        let data_dir = DataDir::setup(data_dir);
        Repository { data_dir }
    }
}

impl RepositoryTrait for Repository {
    fn edit(&self, file_name: &str) -> Result<()> {
        let file_path = &self.data_dir.join(file_name);
        scrawl::edit(file_path).with_context(|| format!("Failed to edit {}", file_name))?;
        Ok(())
    }

    fn get_content(&self, file_name: &str) -> Result<String> {
        let file_path = &self.data_dir.join(file_name);
        let content = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to get content {}", file_name))?;
        Ok(content)
    }

    fn delete(&self, file_name: &str) -> Result<()> {
        let file_path = &self.data_dir.join(file_name);
        fs::remove_file(file_path).with_context(|| format!("Failed to delete {}", file_name))
    }

    fn get_path(&self, file_name: &str) -> Result<String> {
        let path = self.data_dir.join(file_name).to_string_lossy().to_string();
        Ok(path)
    }

    fn read_dir(&self) -> ReadDir {
        self.data_dir.read_dir()
    }
}
