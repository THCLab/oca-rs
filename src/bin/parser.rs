const VERSION: &str = env!("CARGO_PKG_VERSION");

use clap::{Arg, Command};
use oca_rust::state::{oca::OCA, validator};
use oca_rust::xls_parser::{self, entries::ParsedResult as ParsedEntries};
use said::derivation::SelfAddressing;
use std::io::prelude::*;

fn main() {
    let matches = Command::new("XLS(X) Parser")
        .version(VERSION)
        .subcommand(
            Command::new("parse")
            .about("Parse XLS(X) file to OCA or entries")
            .subcommand(
                Command::new("oca")
                    .about("Parse XLS(X) file to OCA")
                    .arg(
                        Arg::new("path")
                            .short('p')
                            .long("path")
                            .multiple_occurrences(true)
                            .multiple_values(true)
                            .required(true)
                            .takes_value(true)
                            .help("Path to XLS(X) file. Sample XLS(X) file can be found here: https://github.com/THCLab/oca-rust/blob/main/tests/assets/oca_template.xlsx"),
                    )
                    .arg(
                        Arg::new("form-layout")
                            .long("form-layout")
                            .required(false)
                            .takes_value(true)
                            .help("Path to YAML file with Form Layout."),
                    )
                    .arg(
                        Arg::new("credential-layout")
                            .long("credential-layout")
                            .required(false)
                            .takes_value(true)
                            .help("Path to YAML file with Credential Layout."),
                    )
                    .arg(
                        Arg::new("no-validation")
                            .long("no-validation")
                            .takes_value(false)
                            .help("Disables OCA validation"),
                    )
                    .arg(
                        Arg::new("zip")
                            .long("zip")
                            .takes_value(false)
                            .help("Generate OCA in zip file"),
                    ),
            )
            .subcommand(
                Command::new("entries")
                    .about("Parse XLS(X) file to entries")
                    .arg(
                        Arg::new("path")
                            .short('p')
                            .long("path")
                            .required(true)
                            .takes_value(true)
                            .help("Path to XLS(X) file. Sample XLS(X) file can be found here: https://github.com/THCLab/oca-rust/blob/main/tests/assets/entries_template.xlsx"),
                    )
                    .arg(
                        Arg::new("zip")
                            .long("zip")
                            .takes_value(false)
                            .help("Generate OCA in zip file"),
                    ),
            )
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("parse") {
        if let Some(matches) = matches.subcommand_matches("oca") {
            let _validation = !matches.is_present("no-validation");
            let to_be_zipped = matches.is_present("zip");
            let paths: Vec<&str> = matches.values_of("path").unwrap().collect();
            let first_path = paths.first().unwrap().to_string();
            let mut parsed_oca_builder_list = vec![];
            let mut parsed_oca_list = vec![];
            let errors: Vec<validator::Error> = vec![];

            for (i, p) in paths.iter().enumerate() {
                let path = p.to_string();
                let form_layout_path: Option<&str> = if i == 0 {
                    matches.value_of("form-layout")
                } else {
                    None
                };
                let credential_layout_path: Option<&str> = if i == 0 {
                    matches.value_of("credential-layout")
                } else {
                    None
                };
                let result =
                    xls_parser::oca::parse(path.clone(), form_layout_path, credential_layout_path);

                if let Err(e) = result {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({ "errors": e })).unwrap()
                    );
                    return;
                }

                let parsed = result.unwrap();
                parsed_oca_builder_list.push(parsed.oca_builder)
            }

            parsed_oca_builder_list.reverse();
            let mut root_oca_builder = parsed_oca_builder_list.pop().unwrap();

            for mut oca_builder in parsed_oca_builder_list {
                let cs_json = serde_json::to_string(&oca_builder.oca.capture_base).unwrap();
                let sai = format!("{}", SelfAddressing::Blake3_256.derive(cs_json.as_bytes()));
                root_oca_builder.add_form_layout_reference(
                    sai.clone(),
                    oca_builder.build_default_form_layout(),
                );
                oca_builder.form_layout = Some(String::new());
                root_oca_builder.add_credential_layout_reference(
                    sai.clone(),
                    oca_builder.build_default_credential_layout(),
                );
                oca_builder.credential_layout = Some(String::new());
                parsed_oca_list.push(oca_builder.finalize());
            }

            parsed_oca_list.push(root_oca_builder.finalize());
            parsed_oca_list.reverse();

            /*
            if validation {
                for oca in parsed_oca_list {
                    let validator =
                        validator::Validator::new().enforce_translations(parsed.languages.clone());
                    let validation_result = validator.validate(&oca);
                    if let Err(errs) = validation_result {
                        for e in errs {
                            errors.push(e);
                        }
                    }
                }
            }
            */

            if errors.is_empty() {
                if to_be_zipped {
                    let filename = first_path
                        .split('/')
                        .collect::<Vec<&str>>()
                        .pop()
                        .unwrap()
                        .rsplit('.')
                        .collect::<Vec<&str>>()
                        .pop()
                        .unwrap()
                        .to_string();
                    match zip_oca(parsed_oca_list, filename.clone()) {
                        Ok(_) => println!("OCA written to {}.zip", filename),
                        Err(e) => println!("Error: {:?}", e),
                    }
                } else {
                    let v = serde_json::to_value(&parsed_oca_list).unwrap();
                    println!("{}", v);
                }
            } else {
                println!("{:#?}", errors);
            }
        }

        if let Some(matches) = matches.subcommand_matches("entries") {
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
                    .split('/')
                    .collect::<Vec<&str>>()
                    .pop()
                    .unwrap()
                    .rsplit('.')
                    .collect::<Vec<&str>>()
                    .pop()
                    .unwrap()
                    .to_string();
                match zip_entries(parsed, filename.clone()) {
                    Ok(_) => println!("Entries written to {}.zip", filename),
                    Err(e) => println!("Error: {:?}", e),
                }
            } else {
                let v = serde_json::to_value(&parsed).unwrap();
                println!("{}", v);
            }
        }
    }
}

fn zip_oca(oca_list: Vec<OCA>, filename: String) -> zip::result::ZipResult<()> {
    let zip_name = format!("{}.zip", filename);
    let zip_path = std::path::Path::new(zip_name.as_str());
    let file = std::fs::File::create(zip_path).unwrap();
    let mut zip = zip::ZipWriter::new(file);

    let mut root_cb_sai = String::new();
    let mut files_json = serde_json::json!({});
    for (i, oca) in oca_list.iter().enumerate() {
        let cb_json = serde_json::to_string(&oca.capture_base).unwrap();
        let cb_sai = SelfAddressing::Blake3_256.derive(cb_json.as_bytes());
        if i == 0 {
            root_cb_sai = format!("{}", cb_sai)
        }

        zip.start_file(
            format!("{}.json", cb_sai),
            zip::write::FileOptions::default(),
        )?;
        zip.write_all(cb_json.as_bytes())?;
        let files = files_json.as_object_mut().unwrap();
        files.insert(format!("capture_base-{}", i), serde_json::json!(cb_sai));

        for overlay in oca.overlays.iter() {
            let overlay_json = serde_json::to_string(&overlay).unwrap();
            let overlay_sai = SelfAddressing::Blake3_256.derive(overlay_json.as_bytes());
            zip.start_file(
                format!("{}.json", overlay_sai,),
                zip::write::FileOptions::default(),
            )?;
            zip.write_all(overlay_json.as_bytes())?;

            let overlay_type = overlay.overlay_type().split('/').collect::<Vec<&str>>()[2];
            let files_overlay_key = match overlay.language() {
                Some(lang) => format!("[{}] {} ({})", cb_sai, overlay_type, lang),
                None => format!("[{}] {}", cb_sai, overlay_type),
            };
            let files = files_json.as_object_mut().unwrap();
            files.insert(files_overlay_key, serde_json::json!(overlay_sai));
        }
    }

    zip.start_file(
        String::from("meta.json"),
        zip::write::FileOptions::default(),
    )?;
    zip.write_all(
        serde_json::to_string_pretty(
            &serde_json::json!({ "root": root_cb_sai, "files": files_json }),
        )
        .unwrap()
        .as_bytes(),
    )?;

    zip.finish()?;
    Ok(())
}

fn zip_entries(parsed: ParsedEntries, filename: String) -> zip::result::ZipResult<()> {
    let zip_name = format!("{}.zip", filename);
    let zip_path = std::path::Path::new(zip_name.as_str());
    let file = std::fs::File::create(zip_path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    let codes_json = serde_json::to_string(&parsed.codes).unwrap();
    let codes_sai = SelfAddressing::Blake3_256.derive(codes_json.as_bytes());
    zip.start_file(
        format!("{}.json", codes_sai),
        zip::write::FileOptions::default(),
    )?;
    zip.write_all(codes_json.as_bytes())?;

    for (lang, translation) in parsed.translations.iter() {
        let translation_json = serde_json::to_string(&translation).unwrap();
        let translation_sai = SelfAddressing::Blake3_256.derive(translation_json.as_bytes());
        zip.start_file(
            format!("[{}] {}.json", lang, translation_sai,),
            zip::write::FileOptions::default(),
        )?;
        zip.write_all(translation_json.as_bytes())?;
    }

    zip.finish()?;
    Ok(())
}
