use std::{
    fs,
    path::{Path, PathBuf},
    process,
};

use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use scrawl::error;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Action,

    /// <Optional> Specify the directory to store kakisute files
    #[clap(long = "data_dir")]
    data_dir: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Action {
    /// Create new kakisute
    New {
        /// <Optional> Specify file name
        filename: Option<String>,
    },
    List {},
}

struct DataDir {
    path: PathBuf,
}

impl DataDir {
    fn setup(dir: Option<String>) -> Self {
        let data_dir_path = if let Some(dir) = dir {
            Path::new(&dir).to_owned()
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

    fn join(&self, filename: &str) -> PathBuf {
        self.path.join(filename)
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

fn main() -> Result<(), error::ScrawlError> {
    let cli = Args::parse();

    let data_dir = DataDir::setup(cli.data_dir);

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
