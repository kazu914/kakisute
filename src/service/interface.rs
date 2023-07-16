use anyhow::Result;
use std::fs::ReadDir;
pub trait IRepository {
    fn read_dir(&self) -> ReadDir;
    fn edit(&self, file_name: &str) -> Result<()>;
    fn get_path(&self, file_name: &str) -> Result<String>;
    fn delete(&self, file_name: &str) -> Result<()>;
    fn get_content(&self, file_name: &str) -> Result<String>;
}
