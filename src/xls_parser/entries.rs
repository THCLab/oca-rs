use calamine::{open_workbook_auto, DataType, Reader};
use crate::state::language::Language;
use std::collections::{BTreeMap, HashMap};
use serde::Serialize;

#[derive(Serialize)]
pub struct ParsedResult {
    pub codes: Vec<String>,
    pub translations: BTreeMap<Language, BTreeMap<String, String>>,
}

const ENTRY_CODE_INDEX: u32 = 0;
const ENTRY_LABEL_INDEX: u32 = 1;
const SAMPLE_TEMPLATE_MSG: &str = "Sample file template can be found here: https://github.com/THCLab/oca-rust/blob/main/tests/assets/entries_template.xlsx";

pub fn parse(path: String) -> Result<ParsedResult, String> {
    let mut workbook = open_workbook_auto(path).or(Err(
        "Provided file cannot be parsed. Check if file exists and format is XLS(X)",
    ))?;
    let mut sheet_names = workbook.sheet_names().to_vec();
    sheet_names.retain(|n| n != "READ ME");

    let mut translation_sheets: Vec<(Language, _)> = vec![];
    for translation_sheet_name in sheet_names {
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
    let entries_range = (start - 1, first_translation_sheet.height() as u32);

    let mut label_trans: HashMap<u32, HashMap<Language, String>> = HashMap::new();
    for (lang, sheet) in translation_sheets.iter() {
        for entry_index in (entries_range.0)..(entries_range.1) {
            if let Some(DataType::String(label_value)) = sheet.get_value((entry_index, ENTRY_LABEL_INDEX))
            {
                match label_trans.get_mut(&entry_index) {
                    Some(entry_label_tr) => {
                        entry_label_tr.insert(lang.to_string(), label_value.clone());
                    }
                    None => {
                        let mut entry_label_tr: HashMap<Language, String> = HashMap::new();
                        entry_label_tr.insert(lang.to_string(), label_value.clone());
                        label_trans.insert(entry_index, entry_label_tr);
                    }
                }
            }
        }
    }

    let mut codes: Vec<String> = vec![];
    let mut translations: BTreeMap<Language, BTreeMap<String, String>> = BTreeMap::new();

    for entry_index in entries_range.0..entries_range.1 {
        let entry_code = first_translation_sheet
            .get_value((entry_index, ENTRY_CODE_INDEX))
            .unwrap()
            .to_string();
        codes.push(entry_code.clone());

        if let Some(label_tr) = label_trans.get(&entry_index).cloned() {
            for (lang, label) in label_tr.iter() {
                match translations.get_mut(lang) {
                    Some(tr) => {
                        tr.insert(entry_code.clone(), label.clone());
                    }
                    None => {
                        let mut tr: BTreeMap<String, String> = BTreeMap::new();
                        tr.insert(entry_code.clone(), label.clone());
                        translations.insert(lang.clone(), tr);
                    }
                }
            }
        }
    }

    Ok(
        ParsedResult {
            codes,
            translations,
        }
    )
}
