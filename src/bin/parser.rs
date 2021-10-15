const VERSION: &str = env!("CARGO_PKG_VERSION");

use clap::{App, Arg};
use oca_rust::state::{oca::OCA, validator};
use oca_rust::xls_parser;
use said::derivation::SelfAddressing;
use std::collections::BTreeMap;
use std::io::prelude::*;

fn main() {
    let matches = App::new("XLS Parser")
        .version(VERSION)
        .subcommand(
            App::new("parse")
                .about("Parse XLSX file to OCA")
                .arg(
                    Arg::new("path")
                        .short('p')
                        .long("path")
                        .required(true)
                        .takes_value(true)
                        .about("Path to XLSX file"),
                )
                .arg(
                    Arg::new("no-validation")
                        .long("no-validation")
                        .takes_value(false)
                        .about("Disables OCA validation"),
                )
                .arg(
                    Arg::new("zip")
                        .long("zip")
                        .takes_value(false)
                        .about("Generate OCA in zip file"),
                ),
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("parse") {
        let path = matches.value_of("path").unwrap().to_string();
        let parsed = xls_parser::parse(path.clone());

        let validation = !matches.is_present("no-validation");
        let to_be_zipped = matches.is_present("zip");
        let mut errors: BTreeMap<usize, Vec<validator::Error>> = BTreeMap::new();
        if validation {
            for (i, oca) in parsed.oca_list.iter().enumerate() {
                let validator =
                    validator::Validator::new().enforce_translations(parsed.languages.clone());
                let validation_result = validator.validate(&oca);
                if let Err(e) = validation_result {
                    errors.insert(i, e);
                }
            }
        }

        if errors.is_empty() {
            if to_be_zipped {
                let filename = path
                    .split("/")
                    .collect::<Vec<&str>>()
                    .pop()
                    .unwrap()
                    .rsplit(".")
                    .collect::<Vec<&str>>()
                    .pop()
                    .unwrap()
                    .to_string();
                match zip_oca(parsed.oca_list, filename.clone()) {
                    Ok(_) => println!("OCA written to {}.zip", filename.clone()),
                    Err(e) => println!("Error: {:?}", e),
                }
            } else {
                let v = serde_json::to_value(&parsed.oca_list).unwrap();
                println!("{}", v);
            }
        } else {
            println!("{:#?}", errors);
        }
    }
}

fn zip_oca(oca_list: Vec<OCA>, filename: String) -> zip::result::ZipResult<()> {
    let zip_name = format!("{}.zip", filename.clone());
    let zip_path = std::path::Path::new(zip_name.as_str());
    let file = std::fs::File::create(&zip_path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    for oca in oca_list.iter() {
        let cb_json = serde_json::to_string(&oca.capture_base).unwrap();
        let cb_sai = SelfAddressing::Blake3_256.derive(cb_json.as_bytes());
        zip.start_file(
            format!("{}/{}.json", filename.clone(), cb_sai),
            zip::write::FileOptions::default(),
        )?;
        zip.write(cb_json.as_bytes())?;

        for overlay in oca.overlays.iter() {
            let overlay_json = serde_json::to_string(&overlay).unwrap();
            let overlay_sai = SelfAddressing::Blake3_256.derive(overlay_json.as_bytes());
            zip.start_file(
                format!("{}/{}/{}.json", filename.clone(), cb_sai, overlay_sai,),
                zip::write::FileOptions::default(),
            )?;
            zip.write(overlay_json.as_bytes())?;
        }
    }

    zip.finish()?;
    Ok(())
}
