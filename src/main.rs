use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use scrawl::error;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    New { filename: Option<String> },
    List {},
}

struct DataDir {
    path: PathBuf,
}

impl DataDir {
    fn setup() -> Self {
        let project_dirs = ProjectDirs::from("", "", "kakisute").unwrap();
        let _ = fs::create_dir_all(project_dirs.data_dir());
        DataDir {
            path: project_dirs.data_dir().to_path_buf(),
        }
    }

    fn join(&self, filename: &str) -> PathBuf {
        self.path.join(filename)
    }
}

fn main() -> Result<(), error::ScrawlError> {
    let cli = Args::parse();

    let data_dir = DataDir::setup();

    match cli.action {
        Action::New { filename } => {
            let file_path = get_file_name(data_dir, filename);
            let _utput = scrawl::edit(file_path).unwrap();
        }
        Action::List {} => {
            print!("List here");
        }
    }
    Ok(())
}

fn get_file_name(data_dir: DataDir, arg_filename: Option<String>) -> PathBuf {
    if let Some(arg_filename) = arg_filename {
        data_dir.join(&arg_filename)
    } else {
        data_dir.join("new_file.txt")
    }
}
