use std::{
    fs::{self, ReadDir},
    path::{Path, PathBuf},
    process,
};

use directories::ProjectDirs;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub struct DataDir {
    path: PathBuf,
}

impl DataDir {
    pub fn setup(dir: Option<String>) -> Self {
        let data_dir_path = if let Some(dir) = dir {
            let expanded_dir = shellexpand::full(&dir)
                .unwrap_or_else(|err| {
                    eprintln!("Error: Can't understand data directory: {:?}", dir);
                    eprintln!("{:?}", err);
                    process::exit(1)
                })
                .to_string();
            Path::new(&expanded_dir).to_owned()
        } else {
            let project_dirs = ProjectDirs::from("", "", PKG_NAME).unwrap();
            project_dirs.data_dir().to_path_buf()
        };

        Self::create_dir(&data_dir_path);
        Self::check_readonly(&data_dir_path);

        DataDir {
            path: data_dir_path.to_path_buf(),
        }
    }

    pub fn join(&self, file_name: &str) -> PathBuf {
        self.path.join(file_name)
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn read_dir(&self) -> ReadDir {
        fs::read_dir(self.path()).unwrap()
    }

    fn create_dir(path: &PathBuf) {
        fs::create_dir_all(&path).unwrap_or_else(|err| {
            eprintln!("Error: Can't make data directory: {:?}", path);
            eprintln!("{:?}", err);
            process::exit(1)
        });
    }

    fn check_readonly(path: &PathBuf) {
        let metadata = path.metadata();
        if let Ok(metadata) = metadata {
            if metadata.permissions().readonly() {
                eprintln!("Error: Directory {:?} is READONLY", path);
                process::exit(1)
            }
        } else {
            eprintln!(
                "Unexpected Error: Can not get permissions information: {:?}",
                path
            );
            process::exit(1)
        }
    }
}
