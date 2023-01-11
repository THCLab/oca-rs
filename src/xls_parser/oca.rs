use crate::state::{
    attribute::{AttributeBuilder, AttributeType, Entries, Entry},
    encoding::Encoding,
    entries::EntriesElement,
    entry_codes::EntryCodes,
    language::Language,
    oca::OCABuilder,
};
use calamine::{open_workbook_auto, DataType, Range, Reader};
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::prelude::*;

pub struct ParsedResult {
    pub oca_builder: OCABuilder,
    pub languages: Vec<Language>,
}

const SAMPLE_TEMPLATE_MSG: &str = "Template file can be found here: https://github.com/THCLab/oca-ecosystem/raw/main/examples/template.xlsx";

pub fn parse(
    path: String,
    default_form_layout: bool,
    form_layout_path: Option<&str>,
    default_credential_layout: bool,
    credential_layout_path: Option<&str>,
) -> Result<ParsedResult, Vec<std::string::String>> {
    let mut errors: Vec<String> = vec![];
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
    let mut workbook = open_workbook_auto(path).map_err(|_| {
        errors.push(
            "Provided file cannot be parsed. Check if file exists and format is XLS(X)".to_string(),
        );
        errors.clone()
    })?;
    let mut sheet_names = workbook.sheet_names().to_vec();
    let mut languages = vec![];
    sheet_names
        .retain(|n| n != "READ ME" && n != "README" && n != "Start Here" && n != "Documentation");

    let main_sheet_name = sheet_names.first().ok_or_else(|| {
        errors.push(format!("Missing sheets. {}", SAMPLE_TEMPLATE_MSG));
        errors.clone()
    })?;
    if !(main_sheet_name.eq("Main") || main_sheet_name.eq("main")) {
        errors.push(format!(
            "Provided XLS file does not match template. Missing Main sheet. {}",
            SAMPLE_TEMPLATE_MSG
        ));
    }

    if !errors.is_empty() {
        return Err(errors);
    }
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

    let mut column_indicies: HashMap<&str, u32> = HashMap::new();
    for row in main_sheet.rows().filter(|&row| {
        if let DataType::String(v) = &row[0] {
            v.starts_with("CB") || v.starts_with("OL")
        } else {
            false
        }
    }) {
        for (i, value) in row.iter().enumerate() {
            if let DataType::String(v) = value {
                if v.starts_with("CB-CL:") {
                    column_indicies.insert("CLASSIFICATION_INDEX", i as u32);
                } else if v.starts_with("CB-AN:") {
                    column_indicies.insert("ATTR_NAME_INDEX", i as u32);
                } else if v.starts_with("CB-AT:") {
                    column_indicies.insert("ATTR_TYPE_INDEX", i as u32);
                } else if v.starts_with("CB-RS:") {
                    column_indicies.insert("REFERENCE_SAI_INDEX", i as u32);
                } else if v.starts_with("CB-FA:") {
                    column_indicies.insert("FLAGGED_INDEX", i as u32);
                } else if v.starts_with("OL-CH:") {
                    column_indicies.insert("ENCODING_INDEX", i as u32);
                } else if v.starts_with("OL-ST:") {
                    column_indicies.insert("STANDARD_INDEX", i as u32);
                } else if v.starts_with("OL-FT:") {
                    column_indicies.insert("FORMAT_INDEX", i as u32);
                } else if v.starts_with("OL-EC:") {
                    column_indicies.insert("ENTRY_CODES_INDEX", i as u32);
                } else if v.starts_with("OL-UT:") {
                    column_indicies.insert("UNIT_INDEX", i as u32);
                } else if v.starts_with("OL-CC:") {
                    column_indicies.insert("CONDITION_INDEX", i as u32);
                } else if v.starts_with("OL-CD:") {
                    column_indicies.insert("DEPENDENCIES_INDEX", i as u32);
                } else if v.starts_with("OL-CR:") {
                    column_indicies.insert("CARDINALITY_INDEX", i as u32);
                } else if v.starts_with("OL-CN:") {
                    column_indicies.insert("CONFORMANCE_INDEX", i as u32);
                } else if v.starts_with("OL-AM:") {
                    column_indicies.insert("ATTRIBUTE_MAPPING_INDEX", i as u32);
                } else if v.starts_with("OL-EM:") {
                    column_indicies.insert("ENTRY_CODE_MAPPING_INDEX", i as u32);
                }
            }
        }
    }

    if column_indicies.get("ATTR_NAME_INDEX").is_none() {
        errors.push("Not found column with Attribute Names definiton".to_string());
    }
    if column_indicies.get("ATTR_NAME_INDEX").is_none() {
        errors.push("Not found column with Attribute Names definiton".to_string());
    }
    if !errors.is_empty() {
        return Err(errors);
    }

    let sheet_version = main_sheet.get_value((0, 0));
    let start: u32 = 3;
    let end_offset = if let Some(_) = sheet_version { 1 } else { 3 };
    let mut end: u32 = main_sheet.height() as u32;
    for (i, row) in main_sheet.rows().enumerate().rev() {
        if row
            .iter()
            .any(|cell| cell != &DataType::Empty && cell.get_string().unwrap().trim().ne(""))
        {
            end = end_offset + i as u32;
            break;
        }
    }
    let oca_range = (start, end);

    let mut oca_builder = OCABuilder::new(Encoding::Utf8);

    if let Some(layout) = form_layout {
        oca_builder = oca_builder.add_form_layout(layout);
    } else if default_form_layout {
        oca_builder = oca_builder.add_default_form_layout();
    }
    if let Some(layout) = credential_layout {
        oca_builder = oca_builder.add_credential_layout(layout);
    } else if default_credential_layout {
        oca_builder = oca_builder.add_default_credential_layout();
    }

    let mut classification = String::new();
    if let Some(classification_index) = column_indicies.get("CLASSIFICATION_INDEX") {
        let classification_value = main_sheet.get_value((oca_range.0, *classification_index));
        if let Some(class) = classification_value {
            classification = class.to_string().trim().to_string();
        }
    }
    oca_builder = oca_builder.add_classification(classification);

    // let mut attribute_names = vec![];
    let mut attribute_builders: Vec<(u32, AttributeBuilder)> = vec![];
    fn parse_row(
        attr_index: u32,
        main_sheet: &Range<DataType>,
        column_indicies: HashMap<&str, u32>,
    ) -> Result<AttributeBuilder, String> {
        let attribute_name = main_sheet
            .get_value((attr_index, *column_indicies.get("ATTR_NAME_INDEX").unwrap()))
            .unwrap()
            .to_string()
            .trim()
            .to_string();
        if attribute_name.is_empty() {
            return Err(format!(
                "Parsing attribute in row {} failed. Attribute name is empty.",
                attr_index + 1
            ));
        }
        /*
        let attribute_count = attribute_names
            .iter()
            .filter(|&name| *name == attribute_name)
            .count();
        if attribute_count > 0 {
            attribute_name = format!("{}-{}", attribute_name, attribute_count);
        }

        attribute_names.push(attribute_name.clone());
        */
        let attribute_type = &format!(
            r#"{}"#,
            &main_sheet
                .get_value((attr_index, *column_indicies.get("ATTR_TYPE_INDEX").unwrap()))
                .unwrap()
        )
        .trim()
        .to_string();
        let mut attribute_sai: Option<String> = None;
        if attribute_type.contains("Reference") {
            match column_indicies.get("REFERENCE_SAI_INDEX") {
                Some(reference_sai_index) => {
                    if let Some(sai_value) =
                        &main_sheet.get_value((attr_index, *reference_sai_index))
                    {
                        if let DataType::Empty = sai_value {
                            return Err(format!(
                                "Parsing attribute type in row {} ({}) failed. Missing reference SAI",
                                attr_index + 1,
                                attribute_name,
                            ));
                        }
                        attribute_sai = Some(format!(r#"{}"#, sai_value).trim().to_string());
                    }
                }
                None => return Err("Missing CB-RS: Reference SAI column".to_string()),
            }
        }
        let mut attribute_builder = AttributeBuilder::new(
            attribute_name.clone(),
            serde_json::from_str::<AttributeType>(format!("\"{}\"", attribute_type).as_str())
                .map_err(|e| {
                    format!(
                        "Parsing attribute type in row {} ({}) failed. {}",
                        attr_index + 1,
                        attribute_name,
                        e
                    )
                })?,
        );
        if let Some(sai) = attribute_sai {
            let mut sai_string = sai;
            if sai_string.ends_with(']') {
                sai_string.pop();
            }
            attribute_builder = attribute_builder.add_sai(sai_string);
        }
        if let Some(flagged_index) = column_indicies.get("FLAGGED_INDEX") {
            if let Some(DataType::String(_value)) =
                main_sheet.get_value((attr_index, *flagged_index))
            {
                attribute_builder = attribute_builder.set_flagged();
            }
        }
        if let Some(encoding_index) = column_indicies.get("ENCODING_INDEX") {
            if let Some(DataType::String(encoding_value)) =
                main_sheet.get_value((attr_index, *encoding_index))
            {
                let encoding =
                    serde_json::from_str::<Encoding>(&format!(r#""{}""#, encoding_value.trim()))
                        .map_err(|e| {
                            format!(
                                "Parsing character encoding in row {} failed. {}",
                                attr_index + 1,
                                e
                            )
                        })?;
                attribute_builder = attribute_builder.add_encoding(encoding);
            }
        }

        if let Some(format_index) = column_indicies.get("FORMAT_INDEX") {
            if let Some(DataType::String(format_value)) =
                main_sheet.get_value((attr_index, *format_index))
            {
                attribute_builder =
                    attribute_builder.add_format(format_value.clone().trim().to_string());
            }
        }
        if let Some(entry_codes_index) = column_indicies.get("ENTRY_CODES_INDEX") {
            if let Some(DataType::String(entry_codes_value)) =
                main_sheet.get_value((attr_index, *entry_codes_index))
            {
                if entry_codes_value != &"[SAI]".to_string() {
                    let entry_codes: EntryCodes = if entry_codes_value.starts_with("SAI:") {
                        let sai = entry_codes_value.strip_prefix("SAI:").unwrap();
                        EntryCodes::Sai(sai.to_string())
                    } else {
                        let codes: Vec<String> = entry_codes_value
                            .trim()
                            .to_string()
                            .split('|')
                            .collect::<Vec<&str>>()
                            .iter()
                            .map(|c| c.to_string().trim().to_string())
                            .collect();
                        EntryCodes::Array(codes)
                    };
                    attribute_builder = attribute_builder.add_entry_codes(entry_codes);
                }
            }
        }

        if let Some(condition_index) = column_indicies.get("CONDITION_INDEX") {
            if let Some(DataType::String(condition_value)) =
                main_sheet.get_value((attr_index, *condition_index))
            {
                if let Some(dependencies_index) = column_indicies.get("DEPENDENCIES_INDEX") {
                    if let Some(DataType::String(dependencies_value)) =
                        main_sheet.get_value((attr_index, *dependencies_index))
                    {
                        attribute_builder = attribute_builder.add_condition(
                            condition_value.clone().trim().to_string(),
                            dependencies_value
                                .split(',')
                                .collect::<Vec<&str>>()
                                .iter()
                                .map(|c| c.to_string().trim().to_string())
                                .collect(),
                        );
                    }
                }
            }
        }

        if let Some(cardinality_index) = column_indicies.get("CARDINALITY_INDEX") {
            if let Some(DataType::String(cardinality_value)) =
                main_sheet.get_value((attr_index, *cardinality_index))
            {
                attribute_builder = attribute_builder
                    .add_cardinality(cardinality_value.to_string().trim().to_string());
            }
        }

        if let Some(conformance_index) = column_indicies.get("CONFORMANCE_INDEX") {
            if let Some(DataType::String(conformance_value)) =
                main_sheet.get_value((attr_index, *conformance_index))
            {
                attribute_builder =
                    attribute_builder.add_conformance(conformance_value.clone().trim().to_string());
            }
        }

        if let Some(unit_index) = column_indicies.get("UNIT_INDEX") {
            if let Some(DataType::String(unit_value)) =
                main_sheet.get_value((attr_index, *unit_index))
            {
                let mut metric_system = String::new();
                let mut unit = unit_value.clone().trim().to_string();

                let mut splitted: Vec<String> = unit_value
                    .trim()
                    .to_string()
                    .split('|')
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|c| c.to_string().trim().to_string())
                    .collect();
                if splitted.len() > 1 {
                    unit = splitted.pop().unwrap();
                    metric_system = splitted.join("|");
                }

                attribute_builder = attribute_builder.add_unit(metric_system, unit);
            }
        }

        if let Some(attribute_mapping_index) = column_indicies.get("ATTRIBUTE_MAPPING_INDEX") {
            if let Some(DataType::String(mapping_value)) =
                main_sheet.get_value((attr_index, *attribute_mapping_index))
            {
                attribute_builder =
                    attribute_builder.add_mapping(mapping_value.clone().trim().to_string());
            }
        }

        if let Some(entry_code_mapping_index) = column_indicies.get("ENTRY_CODE_MAPPING_INDEX") {
            if let Some(DataType::String(mapping_value)) =
                main_sheet.get_value((attr_index, *entry_code_mapping_index))
            {
                attribute_builder = attribute_builder.add_entry_codes_mapping(
                    mapping_value
                        .trim()
                        .to_string()
                        .split('|')
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|c| c.to_string().trim().to_string())
                        .collect(),
                );
            }
        }
        Ok(attribute_builder)
    }

    for attr_index in oca_range.0..oca_range.1 {
        match parse_row(attr_index, &main_sheet, column_indicies.clone()) {
            Ok(attribute_builder) => attribute_builders.push((attr_index, attribute_builder)),
            Err(e) => errors.push(e),
        }
    }

    let mut name_trans: HashMap<Language, String> = HashMap::new();
    let mut description_trans: HashMap<Language, String> = HashMap::new();
    let mut extra_trans: HashMap<String, HashMap<Language, String>> = HashMap::new();

    let mut label_trans: HashMap<u32, HashMap<Language, String>> = HashMap::new();
    let mut entries_trans: HashMap<u32, HashMap<Language, EntriesElement>> = HashMap::new();
    let mut information_trans: HashMap<u32, HashMap<Language, String>> = HashMap::new();

    for (lang, sheet) in translation_sheets.iter() {
        let mut sheet_column_indicies: HashMap<&str, u32> = HashMap::new();
        for row in sheet.rows().filter(|&row| {
            if let DataType::String(v) = &row[0] {
                v.starts_with("CB") || v.starts_with("OL")
            } else {
                false
            }
        }) {
            for (i, value) in row.iter().enumerate() {
                if let DataType::String(v) = value {
                    if v.starts_with("OL-MN:") {
                        sheet_column_indicies.insert("META_NAME_INDEX", i as u32);
                    } else if v.starts_with("OL-MV:") {
                        sheet_column_indicies.insert("META_VALUE_INDEX", i as u32);
                    } else if v.starts_with("CB-AN:") {
                        sheet_column_indicies.insert("ATTR_NAME_INDEX", i as u32);
                    } else if v.starts_with("OL-LA:") {
                        sheet_column_indicies.insert("LABEL_INDEX", i as u32);
                    } else if v.starts_with("OL-EN:") {
                        sheet_column_indicies.insert("ENTRIES_INDEX", i as u32);
                    } else if v.starts_with("OL-IN:") {
                        sheet_column_indicies.insert("INFORMATION_INDEX", i as u32);
                    }
                }
            }
        }

        for attr_index in (oca_range.0)..(sheet.height() as u32) {
            if let Some(name_index) = sheet_column_indicies.get("META_NAME_INDEX") {
                if let Some(DataType::String(name_value)) =
                    sheet.get_value((attr_index, *name_index))
                {
                    if let Some(value_index) = sheet_column_indicies.get("META_VALUE_INDEX") {
                        if let Some(DataType::String(value_value)) =
                            sheet.get_value((attr_index, *value_index))
                        {
                            if name_value.eq("name") {
                                name_trans.insert(lang.to_string(), value_value.to_string());
                            } else if name_value.eq("description") {
                                description_trans.insert(lang.to_string(), value_value.to_string());
                            } else {
                                match extra_trans.get_mut(name_value) {
                                    Some(e_trans) => {
                                        e_trans.insert(lang.to_string(), value_value.to_string());
                                    }
                                    None => {
                                        let mut e_trans = HashMap::new();
                                        e_trans.insert(lang.to_string(), value_value.to_string());
                                        extra_trans.insert(name_value.to_string(), e_trans);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        for attr_index in (oca_range.0)..(oca_range.1) {
            if let Some(label_index) = sheet_column_indicies.get("LABEL_INDEX") {
                if let Some(DataType::String(label_value)) =
                    sheet.get_value((attr_index, *label_index))
                {
                    match label_trans.get_mut(&attr_index) {
                        Some(attr_label_tr) => {
                            attr_label_tr
                                .insert(lang.to_string(), label_value.clone().trim().to_string());
                        }
                        None => {
                            let mut attr_label_tr: HashMap<Language, String> = HashMap::new();
                            attr_label_tr
                                .insert(lang.to_string(), label_value.clone().trim().to_string());
                            label_trans.insert(attr_index, attr_label_tr);
                        }
                    }
                }
            }

            if let Some(entries_index) = sheet_column_indicies.get("ENTRIES_INDEX") {
                if let Some(DataType::String(entries_value)) =
                    sheet.get_value((attr_index, *entries_index))
                {
                    let entries_el: EntriesElement = if entries_value.starts_with("SAI:") {
                        let sai = entries_value
                            .strip_prefix("SAI:")
                            .unwrap()
                            .trim()
                            .to_string();
                        EntriesElement::Sai(sai.to_string())
                    } else {
                        let entries_values = entries_value.split('|').collect::<Vec<&str>>();
                        for (i, entries_value_element) in entries_values.iter().enumerate() {
                            let splitted = entries_value_element.split(':').collect::<Vec<&str>>();
                            let entry_key = splitted.first().unwrap().trim().to_string();
                            if entry_key.is_empty() {
                                errors.push(format!(
                                        "Parsing attribute in row {} failed. Invalid Entry Overlay definition for {} language. Missing entry key in position {}",
                                                    attr_index + 1,
                                                    lang,
                                                    i + 1)
                                    );
                                return Err(errors);
                            }
                            splitted.get(1).ok_or_else(|| {
                                errors.push(format!(
                                        "Parsing attribute in row {} failed. Invalid Entry Overlay definition for {} language. Missing entry value in position {}",
                                                    attr_index + 1,
                                                    lang,
                                                    i + 1)
                                    );
                                errors.clone()
                            })?;
                        }
                        let entries_obj =
                            entries_values.iter().fold(BTreeMap::new(), |mut acc, x| {
                                let splitted = x.split(':').collect::<Vec<&str>>();
                                acc.insert(
                                    splitted.first().unwrap().to_string().trim().to_string(),
                                    splitted.get(1).unwrap().to_string().trim().to_string(),
                                );
                                acc
                            });
                        EntriesElement::Object(entries_obj)
                    };
                    match entries_trans.get_mut(&attr_index) {
                        Some(attr_entries_tr) => {
                            if attr_entries_tr.get(lang).is_none() {
                                attr_entries_tr.insert(lang.to_string(), entries_el);
                            }
                        }
                        None => {
                            let mut attr_entries_tr: HashMap<Language, EntriesElement> =
                                HashMap::new();
                            attr_entries_tr.insert(lang.to_string(), entries_el);
                            entries_trans.insert(attr_index, attr_entries_tr);
                        }
                    }
                }
            }

            if let Some(information_index) = sheet_column_indicies.get("INFORMATION_INDEX") {
                if let Some(DataType::String(information_value)) =
                    sheet.get_value((attr_index, *information_index))
                {
                    match information_trans.get_mut(&attr_index) {
                        Some(attr_info_tr) => {
                            attr_info_tr.insert(
                                lang.to_string(),
                                information_value.clone().trim().to_string(),
                            );
                        }
                        None => {
                            let mut attr_info_tr: HashMap<Language, String> = HashMap::new();
                            attr_info_tr.insert(
                                lang.to_string(),
                                information_value.clone().trim().to_string(),
                            );
                            information_trans.insert(attr_index, attr_info_tr);
                        }
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
                                    .ok_or_else(|| {
                                        errors.push(format!(
                                            "Unknown entry code in {} translation: {}",
                                            lang, e_key
                                        ));
                                        errors.clone()
                                    })?
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
    for (name, value_trans) in extra_trans {
        oca_builder = oca_builder.add_meta(name, value_trans);
    }
    // let oca = oca_builder.finalize();

    if errors.is_empty() {
        Ok(ParsedResult {
            oca_builder,
            languages,
        })
    } else {
        Err(errors)
    }
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
            true,
            None,
            true,
            None,
        );
        assert!(result.is_ok());
        if let Ok(parsed) = result {
            assert_eq!(parsed.languages.len(), 2);
            let oca = parsed.oca_builder.finalize();
            assert_eq!(oca.capture_base.attributes.len(), 17);
        }
    }

    #[test]
    fn parse_xls_file() {
        let result = parse(
            format!(
                "{}/tests/assets/oca_template.xls",
                env!("CARGO_MANIFEST_DIR")
            ),
            true,
            None,
            true,
            None,
        );
        assert!(result.is_ok());

        if let Ok(parsed) = result {
            assert_eq!(parsed.languages.len(), 2);
            let oca = parsed.oca_builder.finalize();
            assert_eq!(oca.capture_base.attributes.len(), 17);
        }
    }

    #[test]
    fn return_error_when_file_type_is_invalid() {
        let result = parse(
            format!(
                "{}/tests/assets/invalid_format.txt",
                env!("CARGO_MANIFEST_DIR")
            ),
            true,
            None,
            true,
            None,
        );
        assert!(result.is_err());
    }
}
