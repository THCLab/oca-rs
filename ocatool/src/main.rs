use std::fs;
use std::path::PathBuf;

use clap::Parser as ClapParser;
use clap::Subcommand;
use oca_rs::Facade;
use oca_rs::data_storage::SledDataStorageConfig;
use oca_rs::repositories::SQLiteConfig;

use oca_rs::data_storage::SledDataStorage;
use oca_rs::data_storage::DataStorage;

extern crate dirs;

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
    /// Build oca objects out of ocafile
    Build {
        #[arg(short, long)]
        file: Option<String>,
        #[arg(short, long)]
        local_repository_path: Option<String>,
    },
    /// Publish oca objects into online repository
    Publish {
        #[arg(short, long)]
        repository: String,
    },
    /// Sign specific object to claim ownership
    Sign {
        #[arg(short, long)]
        scid: String,
    },
    Show {
        #[arg(short, long)]
        said: String,
        #[arg(short, long)]
        local_repository_path: Option<String>,
    },
    Get {
        #[arg(short, long)]
        said: String,
        #[arg(short, long)]
        local_repository_path: Option<String>,
    },
    List {
        #[arg(short, long)]
        local_repository_path: Option<String>,
    }

}

fn create_or_open_local_storage(path: PathBuf) -> SledDataStorage {
    // TODO use PathBuf in SqlConfig to be more platform independent
   let config = SledDataStorageConfig::build().path(path).unwrap();

   SledDataStorage::new()
                .config(config)
}


fn get_oca_facade(local_repository_path: PathBuf) -> Facade {
    let db = create_or_open_local_storage(local_repository_path.join("oca_repository"));
    let cache_storage_config = SQLiteConfig::build().path(local_repository_path.join("read_db")).unwrap();
    Facade::new(Box::new(db), cache_storage_config)
}

fn get_repository_path(local_repository_path: &Option<String>) -> PathBuf {

    let path = match local_repository_path {
        None => {
            let mut p = dirs::home_dir().unwrap();
            p.push(".ocatool");
            Some(p)
        },
        Some(p) => Some(PathBuf::from(p)),
    };
    path.unwrap()
}

fn main() {
    env_logger::init();

    let args = Args::parse();


    match &args.command {
        Some(Commands::Build { file, local_repository_path }) => {
            info!("Building OCA bundle from oca file");

            let unparsed_file = match file {
                Some(file) => fs::read_to_string(file).expect("Can't read file"),
                None => fs::read_to_string("OCAfile").expect("Can't read file"),
            };

            let path = get_repository_path(local_repository_path);
            let mut facade = get_oca_facade(path);
            // build from ocafile does everything including storing that in db
            // maybe we could get better naming for it
            let result = facade.build_from_ocafile(unparsed_file);

            if let Ok(oca_bundle) = result {
                let serialized_bundle = serde_json::to_string_pretty(&oca_bundle).unwrap();
                fs::write("output".to_string() + ".ocabundle", serialized_bundle).expect("Unable to write file");
                println!("OCA bundle created in local repository with SCID: {:?}", oca_bundle.said.unwrap());
            } else {
                println!("{:?}", result);
            }
        }
        Some(Commands::Publish { repository: _ }) => {
            info!("Publish OCA bundle to repository");
            unimplemented!("Coming soon!")
        }
        Some(Commands::Sign { scid: _ }) => {
            info!("Sign OCA bundle byc SCID");
            unimplemented!("Coming soon!")
        }
        Some(Commands::List { local_repository_path }) => {
            info!("List OCA object from local repository");
            let path = get_repository_path(local_repository_path);
            let facade = get_oca_facade(path);
            let result = facade.fetch_all_oca_bundle(10, 1).unwrap().records;
            info!("Found {}, results", result.len());
            for bundle in result {
                println!("SAID: {}", bundle.said.unwrap());
            }
        }
        Some(Commands::Show { said, local_repository_path } )=> {
            info!("Search for OCA object in local repository");
            let path = get_repository_path(local_repository_path);
            let facade = get_oca_facade(path);
            match facade.get_oca_bundle_ocafile(said.to_string()) {
             Ok(ocafile) => {
                println!("{}", ocafile);
             },
             Err(errors) => {
                println!("{:?}", errors);
             }
            }
        }
        Some(Commands::Get { said, local_repository_path }) => {
            let path = get_repository_path(local_repository_path);
            let facade = get_oca_facade(path);
            match facade.get_oca_bundle(said.to_string()) {
             Ok(oca_bundle) => {
                 let content = serde_json::to_value(oca_bundle).expect("Field to read oca bundle");
                 println!("{}", serde_json::to_string_pretty(&content).expect("Faild to format oca bundle"));
             },
             Err(errors) => {
                println!("{:?}", errors);
             }
            }
        }
        None => {}
    }
}

// ocafile build -i OCAfile
// ocafile build -s scid
// ocafile publish
// ocafile fetch SAI
// ocafile inspect
