use clap::{Parser, Subcommand};
use kakisute::{app::App, kakisute_list};
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
        file_name: Option<String>,
    },

    /// Print kakisute files
    List {},

    /// Edit existing kakiste
    Edit {
        #[clap(long = "latest")]
        is_latest: bool,
        file_name: Option<String>,
    },

    /// Show existing kakisute
    Show {
        #[clap(long = "latest")]
        is_latest: bool,
        file_name: Option<String>,
    },

    /// Inspect existing kakisute
    Inspect {
        #[clap(long = "latest")]
        is_latest: bool,
        file_name: Option<String>,
    },
}

fn main() -> Result<(), error::ScrawlError> {
    let cli = Args::parse();

    let app = App::new(cli.data_dir);

    match cli.action {
        Action::New { file_name } => {
            app.create_kakisute(file_name);
        }
        Action::List {} => {
            app.list();
        }
        Action::Edit {
            is_latest,
            file_name,
        } => {
            let query = kakisute_list::single_query::SingleQuery::new(is_latest, file_name);
            app.edit(query);
        }
        Action::Show {
            is_latest,
            file_name,
        } => {
            let query = kakisute_list::single_query::SingleQuery::new(is_latest, file_name);
            app.show(query);
        }
        Action::Inspect {
            is_latest,
            file_name,
        } => {
            let query = kakisute_list::single_query::SingleQuery::new(is_latest, file_name);
            app.inspect(query);
        }
    }
    Ok(())
}
