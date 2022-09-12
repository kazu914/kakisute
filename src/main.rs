use std::io::{self, Write};

use clap::{Parser, Subcommand};
use kakisute::{app::App, kakisute_list, ui};

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

    /// Delete existing kakisute
    Delete {
        #[clap(long = "latest")]
        is_latest: bool,
        file_name: Option<String>,
    },

    /// Start TUI mode
    Interact {},
}

fn main() -> anyhow::Result<()> {
    let cli = Args::parse();

    let mut app = App::new(cli.data_dir);

    match cli.action {
        Action::New { file_name } => {
            let file_name = app.create_kakisute(file_name)?;
            println!("Created: {}", file_name);
        }
        Action::List {} => {
            let list = app.get_kakisute_list();

            let stdout = io::stdout();
            let mut handle = io::BufWriter::new(stdout);
            for file in list {
                writeln!(handle, "{}", file.file_name()).unwrap();
            }
        }
        Action::Edit {
            is_latest,
            file_name,
        } => {
            let query = kakisute_list::single_query::SingleQuery::new(is_latest, file_name);
            let file_name = app.edit_by_single_query(query)?;
            println!("Edited: {}", file_name);
        }
        Action::Show {
            is_latest,
            file_name,
        } => {
            let query = kakisute_list::single_query::SingleQuery::new(is_latest, file_name);
            let content = app.get_content_by_single_query(query)?;
            println!("{}", content);
        }
        Action::Inspect {
            is_latest,
            file_name,
        } => {
            let query = kakisute_list::single_query::SingleQuery::new(is_latest, file_name);
            let info = app.inspect_by_query(query)?;
            println!("{}", info);
        }
        Action::Delete {
            is_latest,
            file_name,
        } => {
            let query = kakisute_list::single_query::SingleQuery::new(is_latest, file_name);
            let fil_name = app.delete_by_single_query(query)?;
            println!("Deleted: {}", fil_name);
        }
        Action::Interact {} => {
            let _ = ui::run_app(&mut app);
        }
    }
    Ok(())
}
