use std::fs;

use anyhow::{Context, Ok, Result};

use crate::data_dir::DataDir;

pub fn edit(data_dir: &DataDir, file_name: &str) -> Result<()> {
    let file_path = data_dir.join(file_name);
    scrawl::edit(file_path).with_context(|| format!("Failed to edit {}", file_name))?;
    Ok(())
}

pub fn get_content(data_dir: &DataDir, file_name: &str) -> Result<String> {
    let file_path = data_dir.join(file_name);
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to get content {}", file_name))?;
    Ok(content)
}

pub fn delete(data_dir: &DataDir, file_name: &str) -> Result<()> {
    let file_path = data_dir.join(file_name);
    fs::remove_file(file_path).with_context(|| format!("Failed to delete {}", file_name))
}
