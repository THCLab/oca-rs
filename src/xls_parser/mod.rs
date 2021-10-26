use crate::state::{
    attribute::{Attribute, AttributeType, Entry},
    encoding::Encoding,
    language::Language,
    oca::OCA,
};
use calamine::{open_workbook_auto, DataType, Reader};
use std::collections::{BTreeMap, HashMap};

pub struct ParsedResult {
    pub oca_list: Vec<OCA>,
    pub languages: Vec<Language>,
}

const ATTR_NAME_INDEX: u32 = 1;
const ATTR_TYPE_INDEX: u32 = 2;
const PII_FLAG_INDEX: u32 = 3;
const ENCODING_INDEX: u32 = 4;
const FORMAT_INDEX: u32 = 5;

const LABEL_INDEX: u32 = 3;
const ENTRIES_INDEX: u32 = 4;
const INFORMATION_INDEX: u32 = 5;
const SAMPLE_TEMPLATE_MSG: &str = "Sample file template can be found here: https://github.com/THCLab/oca-rust/blob/main/tests/assets/oca_template.xlsx";

pub fn parse(path: String) -> Result<ParsedResult, Box<dyn std::error::Error>> {
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

    let mut oca_ranges: Vec<(u32, u32)> = vec![];

    let first_translation_sheet = &translation_sheets
        .first()
        .ok_or(format!(
            "Missing translation sheets. {}",
            SAMPLE_TEMPLATE_MSG
        ))?
        .1;
    let mut start: u32 = 3;
    let mut prev_name = first_translation_sheet.get_value((start, 0)).unwrap();
    for (i, oca_name) in first_translation_sheet
        .rows()
        .map(|r| r.first().unwrap())
        .enumerate()
    {
        if i < 3 {
            continue;
        }
        if oca_name != prev_name {
            oca_ranges.push((start, i as u32));
            start = i as u32;
        }
        prev_name = &oca_name;
    }

    oca_ranges.push((start, first_translation_sheet.height() as u32));

    let mut oca_list: Vec<OCA> = vec![];
    for oca_range in oca_ranges {
        let mut oca = OCA::new(Encoding::Utf8);

        let mut attributes: Vec<(u32, Attribute)> = vec![];
        for attr_index in oca_range.0..oca_range.1 {
            let mut attribute = Attribute::new(
                main_sheet
                    .get_value((attr_index, ATTR_NAME_INDEX))
                    .unwrap()
                    .to_string(),
                serde_json::from_str::<AttributeType>(&format!(
                    r#""{}""#,
                    &main_sheet.get_value((attr_index, ATTR_TYPE_INDEX)).unwrap()
                ))
                .or_else(|e| {
                    Err(format!(
                        "Parsing attribute type in row {} failed. {}",
                        attr_index + 1,
                        e.to_string()
                    ))
                })?,
            );
            if let Some(DataType::String(_value)) =
                main_sheet.get_value((attr_index, PII_FLAG_INDEX))
            {
                attribute = attribute.set_pii();
            }
            if let Some(DataType::String(encoding_value)) =
                main_sheet.get_value((attr_index, ENCODING_INDEX))
            {
                let encoding =
                    serde_json::from_str::<Encoding>(&format!(r#""{}""#, encoding_value)).or_else(
                        |e| {
                            Err(format!(
                                "Parsing character encoding in row {} failed. {}",
                                attr_index + 1,
                                e.to_string()
                            ))
                        },
                    )?;
                attribute = attribute.add_encoding(encoding);
            }

            if let Some(DataType::String(format_value)) =
                main_sheet.get_value((attr_index, FORMAT_INDEX))
            {
                attribute = attribute.add_format(format_value.clone());
            }
            attributes.push((attr_index, attribute));
        }

        let mut name_trans: HashMap<Language, String> = HashMap::new();
        let mut description_trans: HashMap<Language, String> = HashMap::new();

        let mut label_trans: HashMap<u32, HashMap<Language, String>> = HashMap::new();
        let mut entries_trans: HashMap<u32, BTreeMap<String, HashMap<Language, String>>> =
            HashMap::new();
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
                if let Some(DataType::String(label_value)) =
                    sheet.get_value((attr_index, LABEL_INDEX))
                {
                    let splitted_label_value = label_value
                        .split("|")
                        .collect::<Vec<&str>>()
                        .pop()
                        .unwrap()
                        .to_string();
                    match label_trans.get_mut(&attr_index) {
                        Some(attr_label_tr) => {
                            attr_label_tr.insert(lang.to_string(), splitted_label_value);
                        }
                        None => {
                            let mut attr_label_tr: HashMap<Language, String> = HashMap::new();
                            attr_label_tr.insert(lang.to_string(), splitted_label_value);
                            label_trans.insert(attr_index, attr_label_tr);
                        }
                    }
                }

                if let Some(DataType::String(entries_value)) =
                    sheet.get_value((attr_index, ENTRIES_INDEX))
                {
                    let entries = entries_value.split("|").collect::<Vec<&str>>().iter().fold(
                        HashMap::new(),
                        |mut acc, x| {
                            let splitted = x.split(":").collect::<Vec<&str>>();
                            acc.insert(
                                splitted.get(0).unwrap().clone(),
                                splitted.get(1).unwrap().clone(),
                            );
                            acc
                        },
                    );
                    match entries_trans.get_mut(&attr_index) {
                        Some(attr_entries_tr) => {
                            for (entry_key, entry_value) in entries {
                                match attr_entries_tr.get_mut(&entry_key.to_string()) {
                                    Some(attr_entry_tr) => {
                                        attr_entry_tr
                                            .insert(lang.to_string(), entry_value.to_string());
                                    }
                                    None => {
                                        let mut attr_entry_tr: HashMap<Language, String> =
                                            HashMap::new();
                                        attr_entry_tr
                                            .insert(lang.to_string(), entry_value.to_string());
                                        attr_entries_tr
                                            .insert(entry_key.to_string(), attr_entry_tr);
                                    }
                                }
                            }
                        }
                        None => {
                            let mut attr_entries_tr: BTreeMap<String, HashMap<Language, String>> =
                                BTreeMap::new();
                            for (entry_key, entry_value) in entries {
                                match attr_entries_tr.get_mut(&entry_key.to_string()) {
                                    Some(attr_entry_tr) => {
                                        attr_entry_tr
                                            .insert(lang.to_string(), entry_value.to_string());
                                    }
                                    None => {
                                        let mut attr_entry_tr: HashMap<Language, String> =
                                            HashMap::new();
                                        attr_entry_tr
                                            .insert(lang.to_string(), entry_value.to_string());
                                        attr_entries_tr
                                            .insert(entry_key.to_string(), attr_entry_tr);
                                    }
                                }
                            }
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
        for (i, mut attribute) in attributes {
            if let Some(label_tr) = label_trans.get(&i).cloned() {
                attribute = attribute.add_label(label_tr);
            }
            if let Some(entries_tr) = entries_trans.get(&i).cloned() {
                let entries = entries_tr
                    .iter()
                    .map(|(e_key, e_value)| Entry::new(e_key.to_string(), e_value.clone()))
                    .collect::<Vec<_>>();
                attribute = attribute.add_entries(entries);
            }
            if let Some(info_tr) = information_trans.get(&i).cloned() {
                attribute = attribute.add_information(info_tr);
            }
            oca = oca.add_attribute(attribute);
        }
        oca = oca.add_name(name_trans);
        oca = oca.add_description(description_trans);
        oca = oca.finalize();
        oca_list.push(oca);
    }

    Ok(ParsedResult {
        oca_list,
        languages,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_xlsx_file() {
        let result = parse(format!(
            "{}/tests/assets/oca_template.xlsx",
            env!("CARGO_MANIFEST_DIR")
        ));
        assert!(result.is_ok());

        if let Ok(parsed) = result {
            assert_eq!(parsed.oca_list.len(), 3);
            assert_eq!(parsed.languages.len(), 2);
        }
    }

    #[test]
    fn parse_xls_file() {
        let result = parse(format!(
            "{}/tests/assets/oca_template.xls",
            env!("CARGO_MANIFEST_DIR")
        ));
        assert!(result.is_ok());

        if let Ok(parsed) = result {
            assert_eq!(parsed.oca_list.len(), 3);
            assert_eq!(parsed.languages.len(), 2);
        }
    }

    #[test]
    fn parse_schema_names() {
        let result = parse(format!(
            "{}/tests/assets/oca_template.xlsx",
            env!("CARGO_MANIFEST_DIR")
        ));
        assert!(result.is_ok());

        if let Ok(parsed) = result {
            let expected_names = vec![
                "HCF Project Onboarding Form".to_string(),
                "SDG Capture Segment".to_string(),
                "GICS® Capture Segment".to_string(),
            ];
            for (i, oca) in parsed.oca_list.iter().enumerate() {
                let meta_en_overlay = &serde_json::to_value(&oca.overlays.iter().find(|o| {
                    o.overlay_type().to_string().contains("/meta/")
                        && o.language().unwrap().to_string() == "en".to_string()
                }))
                .unwrap();
                if let serde_json::Value::String(en_name) = &meta_en_overlay.get("name").unwrap() {
                    assert_eq!(Some(en_name), expected_names.get(i));
                }
            }
        }
    }

    #[test]
    fn return_error_when_file_type_is_invalid() {
        let result = parse(format!(
            "{}/tests/assets/invalid_format.txt",
            env!("CARGO_MANIFEST_DIR")
        ));
        assert!(result.is_err());
    }
}
