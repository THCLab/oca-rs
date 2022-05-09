use crate::state::{
    attribute::{AttributeBuilder, AttributeType, Entries, Entry},
    encoding::Encoding,
    entries::EntriesElement,
    entry_codes::EntryCodes,
    language::Language,
    oca::{OCABuilder, OCA},
};
use calamine::{open_workbook_auto, DataType, Reader};
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::prelude::*;

pub struct ParsedResult {
    pub oca: OCA,
    pub languages: Vec<Language>,
}

const CLASSIFICATION_INDEX: u32 = 0;
const ATTR_NAME_INDEX: u32 = 1;
const ATTR_TYPE_INDEX: u32 = 2;
const PII_FLAG_INDEX: u32 = 3;
const ENCODING_INDEX: u32 = 4;
const FORMAT_INDEX: u32 = 5;
const ENTRY_CODES_INDEX: u32 = 6;
const CONDITION_INDEX: u32 = 7;
const DEPENDENCIES_INDEX: u32 = 8;
const CARDINALITY_INDEX: u32 = 9;
const CONFORMANCE_INDEX: u32 = 10;
const UNIT_INDEX: u32 = 11;

const LABEL_INDEX: u32 = 3;
const ENTRIES_INDEX: u32 = 4;
const INFORMATION_INDEX: u32 = 5;
const SAMPLE_TEMPLATE_MSG: &str = "Sample file template can be found here: https://github.com/THCLab/oca-rust/blob/main/tests/assets/oca_template.xlsx";

pub fn parse(
    path: String,
    form_layout_path: Option<&str>,
    credential_layout_path: Option<&str>,
) -> Result<ParsedResult, Box<dyn std::error::Error>> {
    let mut form_layout = None;
    let mut credential_layout = None;
    if let Some(path) = form_layout_path {
        let mut file = File::open(path).expect("Unable to open file");
        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .expect("Unable to read file");

        form_layout = Some(contents);
    }
    if let Some(path) = credential_layout_path {
        let mut file = File::open(path).expect("Unable to open file");
        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .expect("Unable to read file");

        credential_layout = Some(contents);
    }
    let mut workbook = open_workbook_auto(path).or(Err(
        "Provided file cannot be parsed. Check if file exists and format is XLS(X)",
    ))?;
    let mut sheet_names = workbook.sheet_names().to_vec();
    let mut languages = vec![];
    sheet_names.retain(|n| n != "READ ME");

    let main_sheet_name = sheet_names
        .first()
        .ok_or(format!("Missing sheets. {}", SAMPLE_TEMPLATE_MSG))?;
    let main_sheet = workbook.worksheet_range(main_sheet_name).unwrap().unwrap();
    let translation_sheet_names = sheet_names.split_off(1);
    let mut translation_sheets: Vec<(Language, _)> = vec![];

    for translation_sheet_name in translation_sheet_names {
        languages.push(translation_sheet_name.clone());
        translation_sheets.push((
            translation_sheet_name.clone(),
            workbook
                .worksheet_range(&translation_sheet_name.clone())
                .unwrap()
                .unwrap(),
        ));
    }

    let first_translation_sheet = &translation_sheets
        .first()
        .ok_or(format!(
            "Missing translation sheets. {}",
            SAMPLE_TEMPLATE_MSG
        ))?
        .1;
    let start: u32 = 3;
    let oca_range = (start, first_translation_sheet.height() as u32);

    let mut oca_builder = OCABuilder::new(Encoding::Utf8);

    if let Some(layout) = form_layout {
        oca_builder = oca_builder.add_form_layout(layout);
    }
    if let Some(layout) = credential_layout {
        oca_builder = oca_builder.add_credential_layout(layout);
    }

    let mut classification = String::new();
    let classification_value = main_sheet.get_value((oca_range.0, CLASSIFICATION_INDEX));
    if let Some(class) = classification_value {
        classification = class.to_string();
    }
    oca_builder = oca_builder.add_classification(classification);

    let mut attribute_names = vec![];
    let mut attribute_builders: Vec<(u32, AttributeBuilder)> = vec![];
    for attr_index in oca_range.0..oca_range.1 {
        let mut attribute_name = main_sheet
            .get_value((attr_index, ATTR_NAME_INDEX))
            .unwrap()
            .to_string();
        let attribute_count = attribute_names
            .iter()
            .filter(|&name| *name == attribute_name)
            .count();
        if attribute_count > 0 {
            attribute_name = format!("{}-{}", attribute_name, attribute_count);
        }

        attribute_names.push(attribute_name.clone());
        let attribute_type_value = &format!(
            r#"{}"#,
            &main_sheet.get_value((attr_index, ATTR_TYPE_INDEX)).unwrap()
        );
        let attribute_type_tmp = attribute_type_value.split(":").collect::<Vec<&str>>();
        let attribute_type = attribute_type_tmp.get(0).unwrap();
        let attribute_sai = attribute_type_tmp.get(1);
        let mut attribute_builder = AttributeBuilder::new(
            attribute_name.clone(),
            serde_json::from_str::<AttributeType>(format!("\"{}\"", attribute_type).as_str())
                .or_else(|e| {
                    Err(format!(
                        "Parsing attribute type in row {} ({}) failed. {}",
                        attr_index + 1,
                        attribute_name,
                        e.to_string()
                    ))
                })?,
        );
        if let Some(sai) = attribute_sai {
            attribute_builder = attribute_builder.add_sai(sai.to_string());
        }
        if let Some(DataType::String(_value)) = main_sheet.get_value((attr_index, PII_FLAG_INDEX)) {
            attribute_builder = attribute_builder.set_pii();
        }
        if let Some(DataType::String(encoding_value)) =
            main_sheet.get_value((attr_index, ENCODING_INDEX))
        {
            let encoding = serde_json::from_str::<Encoding>(&format!(r#""{}""#, encoding_value))
                .or_else(|e| {
                    Err(format!(
                        "Parsing character encoding in row {} failed. {}",
                        attr_index + 1,
                        e.to_string()
                    ))
                })?;
            attribute_builder = attribute_builder.add_encoding(encoding);
        }

        if let Some(DataType::String(format_value)) =
            main_sheet.get_value((attr_index, FORMAT_INDEX))
        {
            attribute_builder = attribute_builder.add_format(format_value.clone());
        }
        if let Some(DataType::String(entry_codes_value)) =
            main_sheet.get_value((attr_index, ENTRY_CODES_INDEX))
        {
            if entry_codes_value != &"[SAI]".to_string() {
                let entry_codes: EntryCodes;
                if entry_codes_value.starts_with("SAI:") {
                    let sai = entry_codes_value.strip_prefix("SAI:").unwrap();
                    entry_codes = EntryCodes::Sai(sai.to_string());
                } else {
                    let codes: Vec<String> = entry_codes_value
                        .split("|")
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|c| c.to_string())
                        .collect();
                    entry_codes = EntryCodes::Array(codes);
                }
                attribute_builder = attribute_builder.add_entry_codes(entry_codes);
            }
        }

        if let Some(DataType::String(condition_value)) =
            main_sheet.get_value((attr_index, CONDITION_INDEX))
        {
            if let Some(DataType::String(dependencies_value)) =
                main_sheet.get_value((attr_index, DEPENDENCIES_INDEX))
            {
                attribute_builder = attribute_builder.add_condition(
                    condition_value.clone(),
                    dependencies_value
                        .split(",")
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|c| c.to_string())
                        .collect(),
                );
            }
        }

        if let Some(DataType::Float(cardinality_value)) =
            main_sheet.get_value((attr_index, CARDINALITY_INDEX))
        {
            attribute_builder = attribute_builder.add_cardinality(cardinality_value.to_string().clone());
        }

        if let Some(DataType::String(conformance_value)) =
            main_sheet.get_value((attr_index, CONFORMANCE_INDEX))
        {
            attribute_builder = attribute_builder.add_conformance(conformance_value.clone());
        }

        if let Some(DataType::String(unit_value)) =
            main_sheet.get_value((attr_index, UNIT_INDEX))
        {
            let mut metric_system = String::new();
            let mut unit = unit_value.clone();

            let mut splitted: Vec<String> = unit_value
                .split("|")
                .collect::<Vec<&str>>()
                .iter()
                .map(|c| c.to_string())
                .collect();
            if splitted.len() > 1 {
                unit = splitted.pop().unwrap();
                metric_system = splitted.join("|");
            }

            attribute_builder = attribute_builder.add_unit(metric_system, unit);
        }
        attribute_builders.push((attr_index, attribute_builder));
    }

    let mut name_trans: HashMap<Language, String> = HashMap::new();
    let mut description_trans: HashMap<Language, String> = HashMap::new();

    let mut label_trans: HashMap<u32, HashMap<Language, String>> = HashMap::new();
    let mut entries_trans: HashMap<u32, HashMap<Language, EntriesElement>> = HashMap::new();
    let mut information_trans: HashMap<u32, HashMap<Language, String>> = HashMap::new();

    for (lang, sheet) in translation_sheets.iter() {
        name_trans.insert(
            lang.to_string(),
            sheet.get_value((oca_range.0, 0)).unwrap().to_string(),
        );
        description_trans.insert(
            lang.to_string(),
            sheet.get_value((oca_range.0, 1)).unwrap().to_string(),
        );

        for attr_index in (oca_range.0)..(oca_range.1) {
            if let Some(DataType::String(label_value)) = sheet.get_value((attr_index, LABEL_INDEX))
            {
                match label_trans.get_mut(&attr_index) {
                    Some(attr_label_tr) => {
                        attr_label_tr.insert(lang.to_string(), label_value.clone());
                    }
                    None => {
                        let mut attr_label_tr: HashMap<Language, String> = HashMap::new();
                        attr_label_tr.insert(lang.to_string(), label_value.clone());
                        label_trans.insert(attr_index, attr_label_tr);
                    }
                }
            }

            if let Some(DataType::String(entries_value)) =
                sheet.get_value((attr_index, ENTRIES_INDEX))
            {
                let entries_el: EntriesElement;
                if entries_value.starts_with("SAI:") {
                    let sai = entries_value.strip_prefix("SAI:").unwrap();
                    entries_el = EntriesElement::Sai(sai.to_string());
                } else {
                    let entries_obj = entries_value.split("|").collect::<Vec<&str>>().iter().fold(
                        BTreeMap::new(),
                        |mut acc, x| {
                            let splitted = x.split(":").collect::<Vec<&str>>();
                            acc.insert(
                                splitted.get(0).unwrap().to_string(),
                                splitted.get(1).unwrap().to_string(),
                            );
                            acc
                        },
                    );
                    entries_el = EntriesElement::Object(entries_obj);
                }
                match entries_trans.get_mut(&attr_index) {
                    Some(attr_entries_tr) => {
                        if attr_entries_tr.get(lang).is_none() {
                            attr_entries_tr.insert(lang.to_string(), entries_el);
                        }
                    }
                    None => {
                        let mut attr_entries_tr: HashMap<Language, EntriesElement> = HashMap::new();
                        attr_entries_tr.insert(lang.to_string(), entries_el);
                        entries_trans.insert(attr_index, attr_entries_tr);
                    }
                }
            }

            if let Some(DataType::String(information_value)) =
                sheet.get_value((attr_index, INFORMATION_INDEX))
            {
                match information_trans.get_mut(&attr_index) {
                    Some(attr_info_tr) => {
                        attr_info_tr.insert(lang.to_string(), information_value.clone());
                    }
                    None => {
                        let mut attr_info_tr: HashMap<Language, String> = HashMap::new();
                        attr_info_tr.insert(lang.to_string(), information_value.clone());
                        information_trans.insert(attr_index, attr_info_tr);
                    }
                }
            }
        }
    }
    for (i, mut attribute_builder) in attribute_builders {
        if let Some(label_tr) = label_trans.get(&i).cloned() {
            attribute_builder = attribute_builder.add_label(label_tr);
        }
        if let Some(lang_entries_tr) = entries_trans.get(&i).cloned() {
            let mut entries: Option<Entries> = None;
            for (lang, entries_tr) in lang_entries_tr.iter() {
                match entries_tr {
                    EntriesElement::Sai(sai) => match entries {
                        Some(Entries::Sai(ref mut lang_sai)) => {
                            lang_sai.insert(lang.to_string(), sai.to_string());
                        }
                        Some(Entries::Object(_)) => {}
                        None => {
                            let mut lang_sai: HashMap<Language, String> = HashMap::new();
                            lang_sai.insert(lang.to_string(), sai.to_string());
                            entries = Some(Entries::Sai(lang_sai));
                        }
                    },
                    EntriesElement::Object(entries_obj) => match entries {
                        Some(Entries::Sai(_)) => {}
                        Some(Entries::Object(ref mut entry_vec)) => {
                            for (e_key, e_val) in entries_obj.iter() {
                                let lang_entry = &mut entry_vec
                                    .iter_mut()
                                    .find(|el| &el.id == e_key)
                                    .ok_or(format!(
                                        "Unknown entry code in {} translation: {}",
                                        lang, e_key
                                    ))?
                                    .translations;
                                lang_entry.insert(lang.to_string(), e_val.clone());
                            }
                        }
                        None => {
                            let mut entry_vec: Vec<Entry> = vec![];
                            for (e_key, e_val) in entries_obj.iter() {
                                let mut lang_entry: HashMap<Language, String> = HashMap::new();
                                lang_entry.insert(lang.to_string(), e_val.clone());
                                entry_vec.push(Entry::new(e_key.to_string(), lang_entry))
                            }
                            entries = Some(Entries::Object(entry_vec));
                        }
                    },
                }
            }
            if let Some(ent) = entries {
                attribute_builder = attribute_builder.add_entries(ent);
            }
        }
        if let Some(info_tr) = information_trans.get(&i).cloned() {
            attribute_builder = attribute_builder.add_information(info_tr);
        }
        oca_builder = oca_builder.add_attribute(attribute_builder.build());
    }
    oca_builder = oca_builder.add_name(name_trans);
    oca_builder = oca_builder.add_description(description_trans);
    let oca = oca_builder.finalize();

    Ok(ParsedResult { oca, languages })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_xlsx_file() {
        let result = parse(
            format!(
                "{}/tests/assets/oca_template.xlsx",
                env!("CARGO_MANIFEST_DIR")
            ),
            None,
            None,
        );
        assert!(result.is_ok());
        if let Ok(parsed) = result {
            assert_eq!(parsed.oca.capture_base.attributes.len(), 18);
            assert_eq!(parsed.languages.len(), 2);
        }
    }

    #[test]
    fn parse_xls_file() {
        let result = parse(
            format!(
                "{}/tests/assets/oca_template.xls",
                env!("CARGO_MANIFEST_DIR")
            ),
            None,
            None,
        );
        assert!(result.is_ok());

        if let Ok(parsed) = result {
            assert_eq!(parsed.oca.capture_base.attributes.len(), 18);
            assert_eq!(parsed.languages.len(), 2);
        }
    }

    #[test]
    fn return_error_when_file_type_is_invalid() {
        let result = parse(
            format!(
                "{}/tests/assets/invalid_format.txt",
                env!("CARGO_MANIFEST_DIR")
            ),
            None,
            None,
        );
        assert!(result.is_err());
    }
}
