use crate::state::{ oca::OCA, oca::overlay, entry_codes::EntryCodes, entries::EntriesElement };
use std::convert::TryInto;
use std::iter::FromIterator;
use std::collections::HashMap;
use xlsxwriter::*;

pub fn generate(oca_list: &Vec<OCA>, filename: String) -> Result<(), Vec<String>> {
    let oca = oca_list.get(0).unwrap();

    let workbook = Workbook::new(format!("{}-data_entry.xlsx", filename).as_str()).unwrap();
    let format_header1 = workbook
        .add_format()
        .set_font_size(10.)
        .set_bold()
        .set_text_wrap()
        .set_align(FormatAlignment::VerticalTop)
        .set_border_bottom(FormatBorder::Thin);
    let format_header2 = workbook
        .add_format()
        .set_font_size(10.)
        .set_bold()
        .set_text_wrap()
        .set_align(FormatAlignment::VerticalTop)
        .set_border_bottom(FormatBorder::Thin)
        .set_border_right(FormatBorder::Thin);
    let format_attr1 = workbook
        .add_format()
        .set_font_size(10.)
        .set_text_wrap()
        .set_align(FormatAlignment::VerticalTop);
    let format_attr2 = workbook
        .add_format()
        .set_font_size(10.)
        .set_text_wrap()
        .set_align(FormatAlignment::VerticalTop)
        .set_border_right(FormatBorder::Thin);

    let format_data_header1 = workbook
        .add_format()
        .set_font_size(11.)
        .set_bg_color(FormatColor::Custom(0xE7E6E6))
        .set_text_wrap()
        .set_align(FormatAlignment::VerticalTop)
        .set_border_bottom(FormatBorder::Thin);
    let format_data_header2 = workbook
        .add_format()
        .set_font_size(11.)
        .set_bg_color(FormatColor::Custom(0xE7E6E6))
        .set_text_wrap()
        .set_align(FormatAlignment::VerticalTop)
        .set_border_bottom(FormatBorder::Thin)
        .set_border_right(FormatBorder::Thin);

    let mut sheet1 = workbook.add_worksheet(Some("Schema Description")).unwrap();
    sheet1.set_row(0, 35., None);
    sheet1.set_column(0, 0, 13., None);
    sheet1.write_string(0, 0, "CB: Classification", Some(&format_header1)).unwrap();
    sheet1.set_column(1, 1, 17., None);
    sheet1.write_string(0, 1, "CB: Attribute Name", Some(&format_header1)).unwrap();
    sheet1.set_column(2, 2, 12.5, None);
    sheet1.write_string(0, 2, "CB: Attribute Type", Some(&format_header1)).unwrap();
    sheet1.write_string(0, 3, "CB: Flagged Attribute", Some(&format_header2)).unwrap();

    let mut sheet2 = workbook.add_worksheet(Some("schema conformant data")).unwrap();

    let mut attributes_index: HashMap<String, u32> = HashMap::new();

    for (i, (attr_name, attr_type)) in oca.capture_base.attributes.iter().enumerate() {
        let attr_i: u32 = (i+1).try_into().unwrap();
        attributes_index.insert(attr_name.to_string(), attr_i);

        sheet1.write_string(attr_i, 0, &oca.capture_base.classification, Some(&format_attr2)).unwrap();
        sheet1.write_string(attr_i, 1, attr_name, Some(&format_attr2)).unwrap();
        sheet1.write_string(attr_i, 2, attr_type, Some(&format_attr1)).unwrap();
        sheet1.write_string(attr_i, 3, if oca.capture_base.flagged_attributes.contains(attr_name) { "Y" } else { "" }, Some(&format_attr2)).unwrap();

        /*
        match attr_type.as_str() {
            "DateTime" => {
                let format_col = workbook.add_format().set_num_format("[0-9]{2}");
                sheet2.set_column(i.try_into().unwrap(), i.try_into().unwrap(), 12., Some(&format_col)).unwrap();
            },
            _ => {}
        }
        */
        sheet2.write_string(0, i.try_into().unwrap(), attr_name, Some(&format_data_header1)).unwrap();
    }

    sheet2.set_column(0, (oca.capture_base.attributes.keys().collect::<Vec<&String>>().len()-1).try_into().unwrap(), 12., None);

    let mut lang: Option<&String> = None;
    let mut skipped: usize = 0;

    for (i, o) in oca.overlays.iter().enumerate() {
        if o.overlay_type().contains("/character_encoding/") {
            sheet1.set_column((i+4-skipped).try_into().unwrap(), (i+4-skipped).try_into().unwrap(), 15., None);
            sheet1.write_string(0, (i+4-skipped).try_into().unwrap(), "OL: Character Encoding", Some(&format_header2)).unwrap();

            let overlay = o.as_any().downcast_ref::<overlay::CharacterEncoding>().unwrap();
            for (attr_name, encoding) in &overlay.attribute_character_encoding {
                if let Ok(serde_json::Value::String(e)) = serde_json::to_value(&encoding) {
                    sheet1.write_string(*attributes_index.get(attr_name.clone().as_str()).unwrap(), (i+4-skipped).try_into().unwrap(), &e, Some(&format_attr2)).unwrap();
                }
            }
        } else if o.overlay_type().contains("/cardinality/") {
            sheet1.set_column((i+4-skipped).try_into().unwrap(), (i+4-skipped).try_into().unwrap(), 15., None);
            sheet1.write_string(0, (i+4-skipped).try_into().unwrap(), "OL: Cardinality", Some(&format_header2)).unwrap();

            let overlay = o.as_any().downcast_ref::<overlay::Cardinality>().unwrap();
            for (attr_name, cardinality) in &overlay.attribute_cardinality {
                sheet1.write_string(*attributes_index.get(attr_name.clone().as_str()).unwrap(), (i+4-skipped).try_into().unwrap(), &cardinality, Some(&format_attr2)).unwrap();
            }
        } else if o.overlay_type().contains("/conformance/") {
            sheet1.set_column((i+4-skipped).try_into().unwrap(), (i+4-skipped).try_into().unwrap(), 15., None);
            sheet1.write_string(0, (i+4-skipped).try_into().unwrap(), "OL: Conformance", Some(&format_header2)).unwrap();

            let overlay = o.as_any().downcast_ref::<overlay::Conformance>().unwrap();
            for (attr_name, conformance) in &overlay.attribute_conformance {
                sheet1.write_string(*attributes_index.get(attr_name.clone().as_str()).unwrap(), (i+4-skipped).try_into().unwrap(), &conformance, Some(&format_attr2)).unwrap();
            }
        } else if o.overlay_type().contains("/conditional/") {
            sheet1.set_column((i+4-skipped).try_into().unwrap(), (i+4-skipped).try_into().unwrap(), 15., None);
            sheet1.write_string(0, (i+4-skipped).try_into().unwrap(), "OL: Conditional [Condition]", Some(&format_header2)).unwrap();
            sheet1.set_column((i+5-skipped).try_into().unwrap(), (i+4-skipped).try_into().unwrap(), 15., None);
            sheet1.write_string(0, (i+5-skipped).try_into().unwrap(), "OL: Conditional [Dependencies]", Some(&format_header2)).unwrap();

            let overlay = o.as_any().downcast_ref::<overlay::Conditional>().unwrap();
            for (attr_name, condition) in &overlay.attribute_conditions {
                sheet1.write_string(*attributes_index.get(attr_name.clone().as_str()).unwrap(), (i+4-skipped).try_into().unwrap(), &condition, Some(&format_attr2)).unwrap();
            }
            for (attr_name, dependencies) in &overlay.attribute_dependencies {
                sheet1.write_string(*attributes_index.get(attr_name.clone().as_str()).unwrap(), (i+5-skipped).try_into().unwrap(), &dependencies.join(","), Some(&format_attr2)).unwrap();
            }

            skipped -= 1;
        } else if o.overlay_type().contains("/format/") {
            sheet1.set_column((i+4-skipped).try_into().unwrap(), (i+4-skipped).try_into().unwrap(), 15., None);
            sheet1.write_string(0, (i+4-skipped).try_into().unwrap(), "OL: Format", Some(&format_header2)).unwrap();

            let overlay = o.as_any().downcast_ref::<overlay::Format>().unwrap();
            for (attr_name, format) in &overlay.attribute_formats {
                sheet1.write_string(*attributes_index.get(attr_name.clone().as_str()).unwrap(), (i+4-skipped).try_into().unwrap(), &format, Some(&format_attr2)).unwrap();

                if let "DateTime" = oca.capture_base.attributes.get(attr_name).unwrap().as_str() {
                    let format_attr = workbook
                        .add_format()
                        .set_num_format(format);

                    for r in 1..1001 {
                        sheet2.write_blank(r, (*attributes_index.get(attr_name.clone().as_str()).unwrap() - 1).try_into().unwrap(), Some(&format_attr));
                    }
                }
            }
        } else if o.overlay_type().contains("/entry_code/") {
            sheet1.write_string(0, (i+4-skipped).try_into().unwrap(), "OL: Entry Code", Some(&format_header2)).unwrap();

            let overlay = o.as_any().downcast_ref::<overlay::EntryCode>().unwrap();
            for (attr_name, entry_codes) in &overlay.attribute_entry_codes {
                if let EntryCodes::Array(codes) = entry_codes {
                    sheet1.write_string(*attributes_index.get(attr_name.clone().as_str()).unwrap(), (i+4-skipped).try_into().unwrap(), &codes.join("|"), Some(&format_attr2)).unwrap();
                    let mut validation = DataValidation::new(
                        DataValidationType::List,
                        DataValidationCriteria::None,
                        DataValidationErrorType::Stop,
                    );
                    validation.value_list = Some(codes.to_vec());
                    sheet2.data_validation_range(1, (*attributes_index.get(attr_name.clone().as_str()).unwrap() - 1).try_into().unwrap(), 1001, (*attributes_index.get(attr_name.clone().as_str()).unwrap() - 1).try_into().unwrap(), &validation);
                }
            }
        } else if o.overlay_type().contains("/label/") {
            if lang.is_none() {
                lang = o.language()
            }
            if lang == o.language() {
                sheet1.set_column((i+4-skipped).try_into().unwrap(), (i+4-skipped).try_into().unwrap(), 17., None);
                sheet1.write_string(0, (i+4-skipped).try_into().unwrap(), "OL: Label", Some(&format_header2)).unwrap();

                let overlay = o.as_any().downcast_ref::<overlay::Label>().unwrap();
                for (attr_name, label) in &overlay.attribute_labels {
                    sheet1.write_string(*attributes_index.get(attr_name.clone().as_str()).unwrap(), (i+4-skipped).try_into().unwrap(), &label, Some(&format_attr2)).unwrap();
                }
            } else {
                skipped += 1;
            }
        } else if o.overlay_type().contains("/entry/") {
            if lang.is_none() {
                lang = o.language()
            }
            if lang == o.language() {
                sheet1.set_column((i+4-skipped).try_into().unwrap(), (i+4-skipped).try_into().unwrap(), 20., None);
                sheet1.write_string(0, (i+4-skipped).try_into().unwrap(), "OL: Entry", Some(&format_header2)).unwrap();

                let overlay = o.as_any().downcast_ref::<overlay::Entry>().unwrap();
                for (attr_name, entries) in &overlay.attribute_entries {
                    if let EntriesElement::Object(entries_map) = entries {
                        sheet1.write_string(*attributes_index.get(attr_name.clone().as_str()).unwrap(), (i+4-skipped).try_into().unwrap(), &Vec::from_iter::<Vec<String>>(entries_map.iter().map(|(k, v)| format!("{}:{}", k, v)).collect()).join("|"), Some(&format_attr2)).unwrap();
                    }
                }
            } else {
                skipped += 1;
            }
        } else if o.overlay_type().contains("/information/") {
            if lang.is_none() {
                lang = o.language()
            }
            if lang == o.language() {
                sheet1.set_column((i+4-skipped).try_into().unwrap(), (i+4-skipped).try_into().unwrap(), 20., None);
                sheet1.write_string(0, (i+4-skipped).try_into().unwrap(), "OL: Information", Some(&format_header2)).unwrap();

                let overlay = o.as_any().downcast_ref::<overlay::Information>().unwrap();
                for (attr_name, info) in &overlay.attribute_information {
                    sheet1.write_string(*attributes_index.get(attr_name.clone().as_str()).unwrap(), (i+4-skipped).try_into().unwrap(), &info, Some(&format_attr2)).unwrap();
                }
            } else {
                skipped += 1;
            }
        }
    }

    workbook.close().unwrap();

    Ok(())
}
