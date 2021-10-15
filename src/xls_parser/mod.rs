use crate::state::{
    attribute::{Attribute, AttributeType, Entry},
    encoding::Encoding,
    language::Language,
    oca::OCA,
};
use calamine::{open_workbook, DataType, Reader, Xlsx};
use core::str::FromStr;
use std::collections::{BTreeMap, HashMap};

pub struct ParsedResult {
    pub oca_list: Vec<OCA>,
    pub languages: Vec<Language>,
}

pub fn parse(path: String) -> ParsedResult {
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
    let mut sheet_names = workbook.sheet_names().to_vec();
    let mut languages = vec![];
    sheet_names.retain(|n| n != "READ ME");
    let translation_sheet_names = sheet_names.split_off(1);

    let main_sheet_name = sheet_names.first().unwrap();
    let main_sheet = workbook.worksheet_range(main_sheet_name).unwrap().unwrap();
    let mut translation_sheets: Vec<(Language, _)> = vec![];

    for translation_sheet_name in translation_sheet_names {
        let mut lang = translation_sheet_name.clone();
        if lang.chars().count() == 2 {
            let lang_lower = lang.to_lowercase();
            let lang_upper = lang.to_uppercase();
            lang.clear();
            lang.push_str(&lang_lower);
            lang.push_str("_");
            lang.push_str(&lang_upper);
        }
        let lang_enum = Language::from_str(&lang).unwrap();
        languages.push(lang_enum.clone());
        translation_sheets.push((
            lang_enum,
            workbook
                .worksheet_range(&translation_sheet_name.clone())
                .unwrap()
                .unwrap(),
        ));
    }

    let mut oca_ranges: Vec<(usize, usize)> = vec![];

    let first_translation_sheet = &translation_sheets.first().unwrap().1;
    let mut start: usize = 3;
    let mut prev_name = first_translation_sheet.get((start, 0)).unwrap();
    for (i, oca_name) in first_translation_sheet
        .rows()
        .map(|r| r.first().unwrap())
        .enumerate()
    {
        if i < 3 {
            continue;
        }
        if oca_name != prev_name {
            oca_ranges.push((start - 2, i - 2));
            start = i;
        }
        prev_name = &oca_name;
    }

    oca_ranges.push((start - 2, first_translation_sheet.height() - 2));

    let mut oca_list: Vec<OCA> = vec![];
    for oca_range in oca_ranges {
        let mut oca = OCA::new(Encoding::Utf8);

        let mut attributes: Vec<(usize, Attribute)> = vec![];
        for attr_index in oca_range.0..oca_range.1 {
            let mut attribute = Attribute::new(
                main_sheet.get((attr_index, 1)).unwrap().to_string(),
                serde_json::from_str::<AttributeType>(&format!(
                    r#""{}""#,
                    &main_sheet.get((attr_index, 2)).unwrap()
                ))
                .unwrap(),
            );
            if let Some(DataType::String(_value)) = main_sheet.get((attr_index, 3)) {
                attribute = attribute.set_pii();
            }
            if let Some(DataType::String(encoding_value)) = main_sheet.get((attr_index, 4)) {
                let encoding =
                    serde_json::from_str::<Encoding>(&format!(r#""{}""#, encoding_value)).unwrap();
                attribute = attribute.add_encoding(encoding);
            }

            if let Some(DataType::String(format_value)) = main_sheet.get((attr_index, 5)) {
                attribute = attribute.add_format(format_value.clone());
            }
            attributes.push((attr_index, attribute));
        }

        let mut name_trans: HashMap<Language, String> = HashMap::new();
        let mut description_trans: HashMap<Language, String> = HashMap::new();

        let label_index = 3;
        let mut label_trans: HashMap<usize, HashMap<Language, String>> = HashMap::new();

        let entries_index = 4;
        let mut entries_trans: HashMap<usize, BTreeMap<String, HashMap<Language, String>>> =
            HashMap::new();

        let information_index = 5;
        let mut information_trans: HashMap<usize, HashMap<Language, String>> = HashMap::new();

        for (lang_enum, sheet) in translation_sheets.iter() {
            name_trans.insert(*lang_enum, sheet.get((oca_range.0, 0)).unwrap().to_string());
            description_trans.insert(*lang_enum, sheet.get((oca_range.0, 1)).unwrap().to_string());

            for attr_index in (oca_range.0)..(oca_range.1 + 2) {
                if let Some(DataType::String(label_value)) =
                    sheet.get((attr_index + 2, label_index))
                {
                    let splitted_label_value = label_value
                        .split("|")
                        .collect::<Vec<&str>>()
                        .pop()
                        .unwrap()
                        .to_string();
                    match label_trans.get_mut(&attr_index) {
                        Some(attr_label_tr) => {
                            attr_label_tr.insert(*lang_enum, splitted_label_value);
                        }
                        None => {
                            let mut attr_label_tr: HashMap<Language, String> = HashMap::new();
                            attr_label_tr.insert(*lang_enum, splitted_label_value);
                            label_trans.insert(attr_index, attr_label_tr);
                        }
                    }
                }

                if let Some(DataType::String(entries_value)) =
                    sheet.get((attr_index + 2, entries_index))
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
                                        attr_entry_tr.insert(*lang_enum, entry_value.to_string());
                                    }
                                    None => {
                                        let mut attr_entry_tr: HashMap<Language, String> =
                                            HashMap::new();
                                        attr_entry_tr.insert(*lang_enum, entry_value.to_string());
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
                                        attr_entry_tr.insert(*lang_enum, entry_value.to_string());
                                    }
                                    None => {
                                        let mut attr_entry_tr: HashMap<Language, String> =
                                            HashMap::new();
                                        attr_entry_tr.insert(*lang_enum, entry_value.to_string());
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
                    sheet.get((attr_index + 2, information_index))
                {
                    match information_trans.get_mut(&attr_index) {
                        Some(attr_info_tr) => {
                            attr_info_tr.insert(*lang_enum, information_value.clone());
                        }
                        None => {
                            let mut attr_info_tr: HashMap<Language, String> = HashMap::new();
                            attr_info_tr.insert(*lang_enum, information_value.clone());
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

    ParsedResult {
        oca_list,
        languages,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let parsed = parse(format!(
            "{}/examples/oca_template.xlsx",
            env!("CARGO_MANIFEST_DIR")
        ));

        assert_eq!(parsed.oca_list.len(), 3);
        assert_eq!(parsed.languages.len(), 2);
        // println!(
        //     "{}",
        //     serde_json::to_string_pretty(&serde_json::to_value(&parsed.oca_list).unwrap()).unwrap()
        // );
    }
}
