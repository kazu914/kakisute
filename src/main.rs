use std::{
    fs,
    io::{self, Write},
};

use clap::{Parser, Subcommand};
use scrawl::error;

use kakisute::data_dir::DataDir;
use kakisute::kakisute_file::KakisuteFile;

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

fn main() -> Result<(), error::ScrawlError> {
    let cli = Args::parse();

    let data_dir = DataDir::setup(cli.data_dir);

    match cli.action {
        Action::New { filename } => {
            let kakisute_file = KakisuteFile::new(filename);
            let file_path = data_dir.join(&kakisute_file.file_name());
            let _utput = scrawl::edit(file_path).unwrap();
        }
        Action::List {} => {
            for file in fs::read_dir(data_dir.path()).unwrap() {
                let kakisute_file = KakisuteFile::from_path(&file.unwrap().path());
                if let Some(kakisute_file) = kakisute_file {
                    let stdout = io::stdout();
                    let mut handle = io::BufWriter::new(stdout);
                    writeln!(handle, "{}", kakisute_file.file_name()).unwrap();
                }
            }
        }
    }
    Ok(())
}
