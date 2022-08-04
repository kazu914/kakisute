use clap::{Parser, Subcommand};
use kakisute::{data_dir::DataDir, services::*};
use scrawl::error;

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
            let new_service = new_service::NewService::new(&data_dir);
            new_service.create(filename);
        }
        Action::List {} => {
            let list_service = list_service::ListService::new(&data_dir);
            list_service.list();
        }
        Action::Edit {
            is_latest,
            filename,
        } => {
            let edit_service = edit_service::EditService::new(&data_dir);
            edit_service.edit(is_latest, filename);
        }
    };
    Ok(())
}
