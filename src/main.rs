use std::io::{self, Write};

use clap::{AppSettings, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use kakisute::{
    repository::Repository,
    service::{Service, ServiceTrait},
    ui,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Action,

    /// <Optional> Specify the directory to store kakisute
    #[clap(long = "data_dir")]
    data_dir: Option<String>,
}

#[derive(Subcommand, Debug)]
#[clap(setting(AppSettings::DeriveDisplayOrder))]
enum Action {
    /// Create new kakisute
    New {
        /// <Optional> Specify kakisute name
        kakisute_name: Option<String>,
    },

    /// Print kakisute list
    List {},

    /// Edit kakiste
    Edit {
        #[clap(long = "latest")]
        is_latest: bool,
        kakisute_name: Option<String>,
    },

    /// Show kakisute content
    Show {
        #[clap(long = "latest")]
        is_latest: bool,
        kakisute_name: Option<String>,
    },

    /// Inspect kakisute
    Inspect {
        #[clap(long = "latest")]
        is_latest: bool,
        kakisute_name: Option<String>,
    },

    /// Delete kakisute
    Delete {
        #[clap(long = "latest")]
        is_latest: bool,
        kakisute_name: Option<String>,
    },

    /// Start TUI mode
    Interact {},

    /// Generate completion script
    Completion {
        #[clap(long, short, arg_enum)]
        shell: Shell,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Args::parse();
    let repository = Repository::new(cli.data_dir);
    let mut service = Service::new(&repository);

    match cli.action {
        Action::New { kakisute_name } => {
            let created_kakisute_name = service.create_kakisute(kakisute_name)?;
            println!("Created: {}", created_kakisute_name);
        }
        Action::List {} => {
            let kakisute_list = service.get_kakisute_list();

            let stdout = io::stdout();
            let mut handle = io::BufWriter::new(stdout);
            for file_name in kakisute_list.get_kakisute_file_name_list() {
                writeln!(handle, "{}", file_name).ok();
            }
        }
        Action::Edit {
            is_latest,
            kakisute_name,
        } => {
            let query = kakisute::service::search_query::SingleQuery::new(is_latest, kakisute_name);
            let edited_kakisute_name = service.edit_by_single_query(query)?;
            println!("Edited: {}", edited_kakisute_name);
        }
        Action::Show {
            is_latest,
            kakisute_name,
        } => {
            let query = kakisute::service::search_query::SingleQuery::new(is_latest, kakisute_name);
            let content = service.get_content_by_single_query(query)?;
            println!("{}", content);
        }
        Action::Inspect {
            is_latest,
            kakisute_name,
        } => {
            let query = kakisute::service::search_query::SingleQuery::new(is_latest, kakisute_name);
            let info = service.inspect_by_query(query)?;
            println!("{}", info);
        }
        Action::Delete {
            is_latest,
            kakisute_name,
        } => {
            let query = kakisute::service::search_query::SingleQuery::new(is_latest, kakisute_name);
            let deleted_kakisute_name = service.delete_by_single_query(query)?;
            println!("Deleted: {}", deleted_kakisute_name);
        }
        Action::Interact {} => {
            let _ = ui::index::run_app(&mut service);
        }
        Action::Completion { shell } => {
            print_completer(shell);
        }
    }
    Ok(())
}

fn print_completer<G: Generator>(generator: G) {
    let mut app = Args::command();
    let name = app.get_name().to_owned();
    generate(generator, &mut app, name, &mut std::io::stdout());
}
