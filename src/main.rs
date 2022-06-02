use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    Add { snipet: String },
    List {},
}

fn main() {
    let cli = Args::parse();
    match cli.action {
        Action::Add { snipet } => {
            println!("INPUT: {}", snipet)
        }
        Action::List {} => {
            print!("List here")
        }
    }
}
