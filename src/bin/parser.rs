const VERSION: &str = env!("CARGO_PKG_VERSION");

use clap::{App, Arg};
use oca_rust::state::{oca::OCA, validator};
use oca_rust::xls_parser::{self, entries::ParsedResult as ParsedEntries};
use said::derivation::SelfAddressing;
use std::io::prelude::*;

fn main() {
    let matches = App::new("XLS(X) Parser")
        .version(VERSION)
        .subcommand(
            App::new("parse")
            .about("Parse XLS(X) file to OCA or entries")
            .subcommand(
                App::new("oca")
                    .about("Parse XLS(X) file to OCA")
                    .arg(
                        Arg::new("path")
                            .short('p')
                            .long("path")
                            .required(true)
                            .takes_value(true)
                            .about("Path to XLS(X) file. Sample XLS(X) file can be found here: https://github.com/THCLab/oca-rust/blob/main/tests/assets/oca_template.xlsx"),
                    )
                    .arg(
                        Arg::new("form-layout")
                            .long("form-layout")
                            .required(false)
                            .takes_value(true)
                            .about("Path to YAML file with Form Layout."),
                    )
                    .arg(
                        Arg::new("credential-layout")
                            .long("credential-layout")
                            .required(false)
                            .takes_value(true)
                            .about("Path to YAML file with Credential Layout."),
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
            .subcommand(
                App::new("entries")
                    .about("Parse XLS(X) file to entries")
                    .arg(
                        Arg::new("path")
                            .short('p')
                            .long("path")
                            .required(true)
                            .takes_value(true)
                            .about("Path to XLS(X) file. Sample XLS(X) file can be found here: https://github.com/THCLab/oca-rust/blob/main/tests/assets/entries_template.xlsx"),
                    )
                    .arg(
                        Arg::new("zip")
                            .long("zip")
                            .takes_value(false)
                            .about("Generate OCA in zip file"),
                    ),
            )
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("parse") {
        if let Some(ref matches) = matches.subcommand_matches("oca") {
            let path = matches.value_of("path").unwrap().to_string();
            let form_layout_path: Option<&str> = matches.value_of("form-layout");
            let credential_layout_path: Option<&str> = matches.value_of("credential-layout");
            let result =
                xls_parser::oca::parse(path.clone(), form_layout_path, credential_layout_path);

            if let Err(e) = result {
                println!("Error: {}", e);
                return;
            }

            let parsed = result.unwrap();
            let validation = !matches.is_present("no-validation");
            let to_be_zipped = matches.is_present("zip");
            let mut errors: Vec<validator::Error> = vec![];
            if validation {
                let validator =
                    validator::Validator::new().enforce_translations(parsed.languages.clone());
                let validation_result = validator.validate(&parsed.oca);
                if let Err(e) = validation_result {
                    errors = e
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
                    match zip_oca(parsed.oca, filename.clone()) {
                        Ok(_) => println!("OCA written to {}.zip", filename.clone()),
                        Err(e) => println!("Error: {:?}", e),
                    }
                } else {
                    let v = serde_json::to_value(&parsed.oca).unwrap();
                    println!("{}", v);
                }
            } else {
                println!("{:#?}", errors);
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("entries") {
            let path = matches.value_of("path").unwrap().to_string();
            let result = xls_parser::entries::parse(path.clone());

            if let Err(e) = result {
                println!("Error: {}", e);
                return;
            }

            let parsed = result.unwrap();
            let to_be_zipped = matches.is_present("zip");

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
                match zip_entries(parsed, filename.clone()) {
                    Ok(_) => println!("Entries written to {}.zip", filename.clone()),
                    Err(e) => println!("Error: {:?}", e),
                }
            } else {
                let v = serde_json::to_value(&parsed).unwrap();
                println!("{}", v);
            }
        }
    }
}

fn zip_oca(oca: OCA, filename: String) -> zip::result::ZipResult<()> {
    let zip_name = format!("{}.zip", filename.clone());
    let zip_path = std::path::Path::new(zip_name.as_str());
    let file = std::fs::File::create(&zip_path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    let cb_json = serde_json::to_string(&oca.capture_base).unwrap();
    let cb_sai = SelfAddressing::Blake3_256.derive(cb_json.as_bytes());
    zip.start_file(
        format!("{}.json", cb_sai),
        zip::write::FileOptions::default(),
    )?;
    zip.write(cb_json.as_bytes())?;

    for overlay in oca.overlays.iter() {
        let overlay_json = serde_json::to_string(&overlay).unwrap();
        let overlay_sai = SelfAddressing::Blake3_256.derive(overlay_json.as_bytes());
        zip.start_file(
            format!("{}/{}.json", cb_sai, overlay_sai,),
            zip::write::FileOptions::default(),
        )?;
        zip.write(overlay_json.as_bytes())?;
    }

    zip.finish()?;
    Ok(())
}

fn zip_entries(parsed: ParsedEntries, filename: String) -> zip::result::ZipResult<()> {
    let zip_name = format!("{}.zip", filename.clone());
    let zip_path = std::path::Path::new(zip_name.as_str());
    let file = std::fs::File::create(&zip_path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    let codes_json = serde_json::to_string(&parsed.codes).unwrap();
    let codes_sai = SelfAddressing::Blake3_256.derive(codes_json.as_bytes());
    zip.start_file(
        format!("{}.json", codes_sai),
        zip::write::FileOptions::default(),
    )?;
    zip.write(codes_json.as_bytes())?;

    for (lang, translation) in parsed.translations.iter() {
        let translation_json = serde_json::to_string(&translation).unwrap();
        let translation_sai = SelfAddressing::Blake3_256.derive(translation_json.as_bytes());
        zip.start_file(
            format!("[{}] {}.json", lang, translation_sai,),
            zip::write::FileOptions::default(),
        )?;
        zip.write(translation_json.as_bytes())?;
    }

    zip.finish()?;
    Ok(())
}
