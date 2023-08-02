use std::fs;

use clap::Parser as ClapParser;
use clap::Subcommand;
use oca_rs::Facade;

use oca_rs::data_storage::SledDataStorage;
use oca_rs::data_storage::DataStorage;


#[macro_use]
extern crate log;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Build {
        #[arg(short, long)]
        file: Option<String>,
    },
    Publish {
        #[arg(short, long)]
        repository: String,
    },
    Sign {
        #[arg(short, long)]
        scid: String,
    },

}

fn main() {
    env_logger::init();

    let args = Args::parse();


    match &args.command {
        Some(Commands::Build { file }) => {
            info!("Building OCA bundle from oca file");

            let unparsed_file = match file {
                Some(file) => fs::read_to_string(file).expect("Can't read file"),
                None => fs::read_to_string("OCAfile").expect("Can't read file"),
            };

            let db = SledDataStorage::open("db_test");
            let facade = Facade::new(Box::new(db));
            let result = facade.build_from_ocafile(unparsed_file);
        
            println!("{:?}", result);
            if let Ok(oca_bundle) = result {
                let serialized_bundle = serde_json::to_string_pretty(&oca_bundle).unwrap();
                fs::write("output".to_string() + ".ocabundle", serialized_bundle).expect("Unable to write file");
            }
        }
        Some(Commands::Publish { repository: _ }) => {
            info!("Publish OCA bundle to repository")
        }
        Some(Commands::Sign { scid: _ }) => {
            info!("Sign OCA bundle byc SCID")
        }
        None => {}
    }
}

// ocafile build -i OCAfile
// ocafile build -s scid
// ocafile publish
// ocafile fetch SAI
// ocafile inspect
