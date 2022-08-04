use clap::{Parser, Subcommand};
use scrawl::error;

use kakisute::kakisute_file::KakisuteFile;
use kakisute::{data_dir::DataDir, kakisute_list::KakisuteList};

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

    /// Print kakisute files
    List {},

    /// Edit existing kakiste
    Edit {
        #[clap(long = "latest")]
        is_latest: bool,
        filename: Option<String>,
    },
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
            let kakisute_list = KakisuteList::from_dir(data_dir.read_dir());
            kakisute_list.print_list();
        }
        Action::Edit {
            is_latest,
            filename,
        } => {
            let kakisute_list = KakisuteList::from_dir(data_dir.read_dir());
            match filename {
                Some(filename) => {
                    let kakisute = kakisute_list.get_by_filename(&filename);
                    match kakisute {
                        Some(kakiste) => {
                            let file_path = data_dir.join(kakiste.file_name());
                            let _utput = scrawl::edit(file_path).unwrap();
                        }
                        None => {
                            println!("Can not find: {}", filename);
                        }
                    }
                }
                None => {
                    if is_latest {
                        let kakisute_list = KakisuteList::from_dir(data_dir.read_dir());
                        let file_path = data_dir.join(&kakisute_list.get_latest().file_name());
                        let _utput = scrawl::edit(file_path).unwrap();
                    } else {
                        println!("edit expect a file name or \"--latest\" flag")
                    }
                }
            }
        }
    };
    Ok(())
}
