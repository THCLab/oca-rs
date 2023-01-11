use crate::state::{entries::EntriesElement, entry_codes::EntryCodes, oca::overlay, oca::OCA};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::convert::TryInto;
use std::iter::FromIterator;
use xlsxwriter::*;

pub fn generate(oca_list: &[OCA], filename: String) -> Result<(), Vec<String>> {
    let mut errors: Vec<String> = vec![];
    let oca = oca_list.get(0).unwrap();

    let workbook =
        Workbook::new(format!("{}-data_entry.xlsx", filename).as_str()).map_err(|e| {
            errors.push(e.to_string());
            errors.clone()
        })?;
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

    let format_data_header = workbook
        .add_format()
        .set_font_size(10.)
        .set_bg_color(FormatColor::Custom(0xE7E6E6))
        .set_align(FormatAlignment::VerticalTop)
        .set_border_bottom(FormatBorder::Thin)
        .set_border_right(FormatBorder::Thin);

    let format_lookup_header = workbook
        .add_format()
        .set_font_size(10.)
        .set_bold()
        .set_bg_color(FormatColor::Custom(0xE7E6E6))
        .set_text_wrap()
        .set_align(FormatAlignment::VerticalTop);
    let format_lookup_attr = workbook
        .add_format()
        .set_font_size(10.)
        .set_bold()
        .set_text_wrap()
        .set_align(FormatAlignment::VerticalTop);
    let format_lookup_value = workbook
        .add_format()
        .set_font_size(10.)
        .set_text_wrap()
        .set_align(FormatAlignment::VerticalTop);

    let mut sheet1 = workbook
        .add_worksheet(Some("Schema Description"))
        .map_err(|e| {
            errors.push(e.to_string());
            errors.clone()
        })?;
    sheet1.set_row(0, 35., None).map_err(|e| {
        errors.push(e.to_string());
        errors.clone()
    })?;
    sheet1.set_column(0, 0, 13., None).map_err(|e| {
        errors.push(e.to_string());
        errors.clone()
    })?;
    sheet1
        .write_string(0, 0, "CB: Classification", Some(&format_header1))
        .map_err(|e| {
            errors.push(e.to_string());
            errors.clone()
        })?;
    sheet1.set_column(1, 1, 17., None).map_err(|e| {
        errors.push(e.to_string());
        errors.clone()
    })?;
    sheet1
        .write_string(0, 1, "CB: Attribute Name", Some(&format_header1))
        .map_err(|e| {
            errors.push(e.to_string());
            errors.clone()
        })?;
    sheet1.set_column(2, 2, 12.5, None).map_err(|e| {
        errors.push(e.to_string());
        errors.clone()
    })?;
    sheet1
        .write_string(0, 2, "CB: Attribute Type", Some(&format_header1))
        .map_err(|e| {
            errors.push(e.to_string());
            errors.clone()
        })?;
    sheet1
        .write_string(0, 3, "CB: Flagged Attribute", Some(&format_header2))
        .map_err(|e| {
            errors.push(e.to_string());
            errors.clone()
        })?;

    let mut sheet2 = workbook.add_worksheet(Some("Data Entry")).map_err(|e| {
        errors.push(e.to_string());
        errors.clone()
    })?;
    let mut sheet3 = workbook
        .add_worksheet(Some("schema conformant data"))
        .map_err(|e| {
            errors.push(e.to_string());
            errors.clone()
        })?;

    let mut attributes_index: HashMap<String, u32> = HashMap::new();

    for (i, (attr_name, attr_type)) in oca.capture_base.attributes.iter().enumerate() {
        let attr_i: u32 = (i + 1).try_into().unwrap();
        attributes_index.insert(attr_name.to_string(), attr_i);

        sheet1
            .write_string(
                attr_i,
                0,
                &oca.capture_base.classification,
                Some(&format_attr2),
            )
            .map_err(|e| {
                errors.push(e.to_string());
                errors.clone()
            })?;
        sheet1
            .write_string(attr_i, 1, attr_name, Some(&format_attr2))
            .map_err(|e| {
                errors.push(e.to_string());
                errors.clone()
            })?;
        sheet1
            .write_string(attr_i, 2, attr_type, Some(&format_attr1))
            .map_err(|e| {
                errors.push(e.to_string());
                errors.clone()
            })?;
        sheet1
            .write_string(
                attr_i,
                3,
                if oca.capture_base.flagged_attributes.contains(attr_name) {
                    "Y"
                } else {
                    ""
                },
                Some(&format_attr2),
            )
            .map_err(|e| {
                errors.push(e.to_string());
                errors.clone()
            })?;

        sheet3
            .write_string(
                0,
                i.try_into().unwrap(),
                attr_name,
                Some(&format_data_header),
            )
            .map_err(|e| {
                errors.push(e.to_string());
                errors.clone()
            })?;
        let letter = char::from_u32((65 + i).try_into().unwrap()).unwrap();
        for r in 1..1001 {
            let formula = format!(
                "=IF(ISBLANK('Data Entry'!${}${}),\"\",'Data Entry'!${}${})",
                letter,
                r + 1,
                letter,
                r + 1
            );
            sheet3
                .write_formula(r, i.try_into().unwrap(), &formula, None)
                .map_err(|e| {
                    errors.push(e.to_string());
                    errors.clone()
                })?;
        }
    }

    sheet2
        .set_column(
            0,
            (oca.capture_base.attributes.keys().count() - 1)
                .try_into()
                .unwrap(),
            12.,
            None,
        )
        .map_err(|e| {
            errors.push(e.to_string());
            errors.clone()
        })?;
    sheet3
        .set_column(
            0,
            (oca.capture_base.attributes.keys().count() - 1)
                .try_into()
                .unwrap(),
            12.,
            None,
        )
        .map_err(|e| {
            errors.push(e.to_string());
            errors.clone()
        })?;

    let mut lang: Option<&String> = None;
    let mut skipped: usize = 0;
    let mut lookup_entries: HashMap<String, &BTreeMap<String, String>> = HashMap::new();

    let languages: Vec<Option<&String>> = oca
        .overlays
        .iter()
        .map(|o| o.language())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    for language_option in languages {
        if let Some(l) = language_option {
            if l.to_lowercase().starts_with("en") {
                lang = language_option;
            }
        }
    }

    for (i, o) in oca.overlays.iter().enumerate() {
        if o.overlay_type().contains("/character_encoding/") {
            sheet1
                .set_column(
                    (i + 4 - skipped).try_into().unwrap(),
                    (i + 4 - skipped).try_into().unwrap(),
                    15.,
                    None,
                )
                .map_err(|e| {
                    errors.push(e.to_string());
                    errors.clone()
                })?;
            sheet1
                .write_string(
                    0,
                    (i + 4 - skipped).try_into().unwrap(),
                    "OL: Character Encoding",
                    Some(&format_header2),
                )
                .map_err(|e| {
                    errors.push(e.to_string());
                    errors.clone()
                })?;

            let overlay = o
                .as_any()
                .downcast_ref::<overlay::CharacterEncoding>()
                .unwrap();

            for j in 0..oca.capture_base.attributes.len() {
                sheet1
                    .write_blank(
                        (j + 1).try_into().unwrap(),
                        (i + 4 - skipped).try_into().unwrap(),
                        Some(&format_attr2),
                    )
                    .map_err(|e| {
                        errors.push(e.to_string());
                        errors.clone()
                    })?;
            }
            for (attr_name, encoding) in &overlay.attribute_character_encoding {
                if let Ok(serde_json::Value::String(e)) = serde_json::to_value(encoding) {
                    sheet1
                        .write_string(
                            *attributes_index.get(attr_name.clone().as_str()).unwrap(),
                            (i + 4 - skipped).try_into().unwrap(),
                            &e,
                            Some(&format_attr2),
                        )
                        .map_err(|e| {
                            errors.push(e.to_string());
                            errors.clone()
                        })?;
                }
            }
        } else if o.overlay_type().contains("/cardinality/") {
            sheet1
                .set_column(
                    (i + 4 - skipped).try_into().unwrap(),
                    (i + 4 - skipped).try_into().unwrap(),
                    15.,
                    None,
                )
                .map_err(|e| {
                    errors.push(e.to_string());
                    errors.clone()
                })?;
            sheet1
                .write_string(
                    0,
                    (i + 4 - skipped).try_into().unwrap(),
                    "OL: Cardinality",
                    Some(&format_header2),
                )
                .map_err(|e| {
                    errors.push(e.to_string());
                    errors.clone()
                })?;

            let overlay = o.as_any().downcast_ref::<overlay::Cardinality>().unwrap();
            for j in 0..oca.capture_base.attributes.len() {
                sheet1
                    .write_blank(
                        (j + 1).try_into().unwrap(),
                        (i + 4 - skipped).try_into().unwrap(),
                        Some(&format_attr2),
                    )
                    .map_err(|e| {
                        errors.push(e.to_string());
                        errors.clone()
                    })?;
            }
            for (attr_name, cardinality) in &overlay.attribute_cardinality {
                sheet1
                    .write_string(
                        *attributes_index.get(attr_name.clone().as_str()).unwrap(),
                        (i + 4 - skipped).try_into().unwrap(),
                        cardinality,
                        Some(&format_attr2),
                    )
                    .map_err(|e| {
                        errors.push(e.to_string());
                        errors.clone()
                    })?;
            }
        } else if o.overlay_type().contains("/conformance/") {
            sheet1
                .set_column(
                    (i + 4 - skipped).try_into().unwrap(),
                    (i + 4 - skipped).try_into().unwrap(),
                    15.,
                    None,
                )
                .map_err(|e| {
                    errors.push(e.to_string());
                    errors.clone()
                })?;
            sheet1
                .write_string(
                    0,
                    (i + 4 - skipped).try_into().unwrap(),
                    "OL: Conformance",
                    Some(&format_header2),
                )
                .map_err(|e| {
                    errors.push(e.to_string());
                    errors.clone()
                })?;

            let overlay = o.as_any().downcast_ref::<overlay::Conformance>().unwrap();
            for j in 0..oca.capture_base.attributes.len() {
                sheet1
                    .write_blank(
                        (j + 1).try_into().unwrap(),
                        (i + 4 - skipped).try_into().unwrap(),
                        Some(&format_attr2),
                    )
                    .map_err(|e| {
                        errors.push(e.to_string());
                        errors.clone()
                    })?;
            }
            for (attr_name, conformance) in &overlay.attribute_conformance {
                sheet1
                    .write_string(
                        *attributes_index.get(attr_name.clone().as_str()).unwrap(),
                        (i + 4 - skipped).try_into().unwrap(),
                        conformance,
                        Some(&format_attr2),
                    )
                    .map_err(|e| {
                        errors.push(e.to_string());
                        errors.clone()
                    })?;
            }
        } else if o.overlay_type().contains("/conditional/") {
            sheet1
                .set_column(
                    (i + 4 - skipped).try_into().unwrap(),
                    (i + 4 - skipped).try_into().unwrap(),
                    15.,
                    None,
                )
                .map_err(|e| {
                    errors.push(e.to_string());
                    errors.clone()
                })?;
            sheet1
                .write_string(
                    0,
                    (i + 4 - skipped).try_into().unwrap(),
                    "OL: Conditional [Condition]",
                    Some(&format_header2),
                )
                .map_err(|e| {
                    errors.push(e.to_string());
                    errors.clone()
                })?;
            sheet1
                .set_column(
                    (i + 5 - skipped).try_into().unwrap(),
                    (i + 4 - skipped).try_into().unwrap(),
                    15.,
                    None,
                )
                .map_err(|e| {
                    errors.push(e.to_string());
                    errors.clone()
                })?;
            sheet1
                .write_string(
                    0,
                    (i + 5 - skipped).try_into().unwrap(),
                    "OL: Conditional [Dependencies]",
                    Some(&format_header2),
                )
                .map_err(|e| {
                    errors.push(e.to_string());
                    errors.clone()
                })?;

            let overlay = o.as_any().downcast_ref::<overlay::Conditional>().unwrap();
            for j in 0..oca.capture_base.attributes.len() {
                sheet1
                    .write_blank(
                        (j + 1).try_into().unwrap(),
                        (i + 4 - skipped).try_into().unwrap(),
                        Some(&format_attr2),
                    )
                    .map_err(|e| {
                        errors.push(e.to_string());
                        errors.clone()
                    })?;
                sheet1
                    .write_blank(
                        (j + 1).try_into().unwrap(),
                        (i + 5 - skipped).try_into().unwrap(),
                        Some(&format_attr2),
                    )
                    .map_err(|e| {
                        errors.push(e.to_string());
                        errors.clone()
                    })?;
            }
            for (attr_name, condition) in &overlay.attribute_conditions {
                sheet1
                    .write_string(
                        *attributes_index.get(attr_name.clone().as_str()).unwrap(),
                        (i + 4 - skipped).try_into().unwrap(),
                        condition,
                        Some(&format_attr2),
                    )
                    .map_err(|e| {
                        errors.push(e.to_string());
                        errors.clone()
                    })?;
            }
            for (attr_name, dependencies) in &overlay.attribute_dependencies {
                sheet1
                    .write_string(
                        *attributes_index.get(attr_name.clone().as_str()).unwrap(),
                        (i + 5 - skipped).try_into().unwrap(),
                        &dependencies.join(","),
                        Some(&format_attr2),
                    )
                    .map_err(|e| {
                        errors.push(e.to_string());
                        errors.clone()
                    })?;
            }

            skipped -= 1;
        } else if o.overlay_type().contains("/format/") {
            sheet1
                .set_column(
                    (i + 4 - skipped).try_into().unwrap(),
                    (i + 4 - skipped).try_into().unwrap(),
                    15.,
                    None,
                )
                .map_err(|e| {
                    errors.push(e.to_string());
                    errors.clone()
                })?;
            sheet1
                .write_string(
                    0,
                    (i + 4 - skipped).try_into().unwrap(),
                    "OL: Format",
                    Some(&format_header2),
                )
                .map_err(|e| {
                    errors.push(e.to_string());
                    errors.clone()
                })?;

            let overlay = o.as_any().downcast_ref::<overlay::Format>().unwrap();
            for j in 0..oca.capture_base.attributes.len() {
                sheet1
                    .write_blank(
                        (j + 1).try_into().unwrap(),
                        (i + 4 - skipped).try_into().unwrap(),
                        Some(&format_attr2),
                    )
                    .map_err(|e| {
                        errors.push(e.to_string());
                        errors.clone()
                    })?;
            }
            for (attr_name, format) in &overlay.attribute_formats {
                sheet1
                    .write_string(
                        *attributes_index.get(attr_name.clone().as_str()).unwrap(),
                        (i + 4 - skipped).try_into().unwrap(),
                        format,
                        Some(&format_attr2),
                    )
                    .map_err(|e| {
                        errors.push(e.to_string());
                        errors.clone()
                    })?;

                if let "DateTime" = oca.capture_base.attributes.get(attr_name).unwrap().as_str() {
                    let format_attr = workbook.add_format().set_num_format(format);

                    for r in 1..1001 {
                        sheet2
                            .write_blank(
                                r,
                                (*attributes_index.get(attr_name.clone().as_str()).unwrap() - 1)
                                    .try_into()
                                    .unwrap(),
                                Some(&format_attr),
                            )
                            .map_err(|e| {
                                errors.push(e.to_string());
                                errors.clone()
                            })?;
                    }
                }
            }
        } else if o.overlay_type().contains("/entry_code/") {
            sheet1
                .write_string(
                    0,
                    (i + 4 - skipped).try_into().unwrap(),
                    "OL: Entry Code",
                    Some(&format_header2),
                )
                .map_err(|e| {
                    errors.push(e.to_string());
                    errors.clone()
                })?;

            let overlay = o.as_any().downcast_ref::<overlay::EntryCode>().unwrap();
            for j in 0..oca.capture_base.attributes.len() {
                sheet1
                    .write_blank(
                        (j + 1).try_into().unwrap(),
                        (i + 4 - skipped).try_into().unwrap(),
                        Some(&format_attr2),
                    )
                    .map_err(|e| {
                        errors.push(e.to_string());
                        errors.clone()
                    })?;
            }
            for (attr_name, entry_codes) in &overlay.attribute_entry_codes {
                if let EntryCodes::Array(codes) = entry_codes {
                    sheet1
                        .write_string(
                            *attributes_index.get(attr_name.clone().as_str()).unwrap(),
                            (i + 4 - skipped).try_into().unwrap(),
                            &codes.join("|"),
                            Some(&format_attr2),
                        )
                        .map_err(|e| {
                            errors.push(e.to_string());
                            errors.clone()
                        })?;
                }
            }
        } else if o.overlay_type().contains("/label/") {
            if lang.is_none() {
                lang = o.language()
            }
            if lang == o.language() {
                sheet1
                    .set_column(
                        (i + 4 - skipped).try_into().unwrap(),
                        (i + 4 - skipped).try_into().unwrap(),
                        17.,
                        None,
                    )
                    .map_err(|e| {
                        errors.push(e.to_string());
                        errors.clone()
                    })?;
                sheet1
                    .write_string(
                        0,
                        (i + 4 - skipped).try_into().unwrap(),
                        "OL: Label",
                        Some(&format_header2),
                    )
                    .map_err(|e| {
                        errors.push(e.to_string());
                        errors.clone()
                    })?;

                let overlay = o.as_any().downcast_ref::<overlay::Label>().unwrap();

                for j in 0..oca.capture_base.attributes.len() {
                    sheet1
                        .write_blank(
                            (j + 1).try_into().unwrap(),
                            (i + 4 - skipped).try_into().unwrap(),
                            Some(&format_attr2),
                        )
                        .map_err(|e| {
                            errors.push(e.to_string());
                            errors.clone()
                        })?;
                }
                for (attr_name, label) in &overlay.attribute_labels {
                    sheet1
                        .write_string(
                            *attributes_index.get(attr_name.clone().as_str()).unwrap(),
                            (i + 4 - skipped).try_into().unwrap(),
                            label,
                            Some(&format_attr2),
                        )
                        .map_err(|e| {
                            errors.push(e.to_string());
                            errors.clone()
                        })?;
                    sheet2
                        .write_string(
                            0,
                            (*attributes_index.get(attr_name.clone().as_str()).unwrap() - 1)
                                .try_into()
                                .unwrap(),
                            label,
                            Some(&format_data_header),
                        )
                        .map_err(|e| {
                            errors.push(e.to_string());
                            errors.clone()
                        })?;
                }
            } else {
                skipped += 1;
            }
        } else if o.overlay_type().contains("/entry/") {
            if lang.is_none() {
                lang = o.language()
            }
            if lang == o.language() {
                sheet1
                    .set_column(
                        (i + 4 - skipped).try_into().unwrap(),
                        (i + 4 - skipped).try_into().unwrap(),
                        20.,
                        None,
                    )
                    .map_err(|e| {
                        errors.push(e.to_string());
                        errors.clone()
                    })?;
                sheet1
                    .write_string(
                        0,
                        (i + 4 - skipped).try_into().unwrap(),
                        "OL: Entry",
                        Some(&format_header2),
                    )
                    .map_err(|e| {
                        errors.push(e.to_string());
                        errors.clone()
                    })?;

                let overlay = o.as_any().downcast_ref::<overlay::Entry>().unwrap();
                for j in 0..oca.capture_base.attributes.len() {
                    sheet1
                        .write_blank(
                            (j + 1).try_into().unwrap(),
                            (i + 4 - skipped).try_into().unwrap(),
                            Some(&format_attr2),
                        )
                        .map_err(|e| {
                            errors.push(e.to_string());
                            errors.clone()
                        })?;
                }
                for (attr_name, entries) in &overlay.attribute_entries {
                    if let EntriesElement::Object(entries_map) = entries {
                        lookup_entries.insert(attr_name.to_string(), entries_map);

                        sheet1
                            .write_string(
                                *attributes_index.get(attr_name.clone().as_str()).unwrap(),
                                (i + 4 - skipped).try_into().unwrap(),
                                &Vec::from_iter::<Vec<String>>(
                                    entries_map
                                        .iter()
                                        .map(|(k, v)| format!("{}:{}", k, v))
                                        .collect(),
                                )
                                .join("|"),
                                Some(&format_attr2),
                            )
                            .map_err(|e| {
                                errors.push(e.to_string());
                                errors.clone()
                            })?;
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
                sheet1
                    .set_column(
                        (i + 4 - skipped).try_into().unwrap(),
                        (i + 4 - skipped).try_into().unwrap(),
                        20.,
                        None,
                    )
                    .map_err(|e| {
                        errors.push(e.to_string());
                        errors.clone()
                    })?;
                sheet1
                    .write_string(
                        0,
                        (i + 4 - skipped).try_into().unwrap(),
                        "OL: Information",
                        Some(&format_header2),
                    )
                    .map_err(|e| {
                        errors.push(e.to_string());
                        errors.clone()
                    })?;

                let overlay = o.as_any().downcast_ref::<overlay::Information>().unwrap();
                for j in 0..oca.capture_base.attributes.len() {
                    sheet1
                        .write_blank(
                            (j + 1).try_into().unwrap(),
                            (i + 4 - skipped).try_into().unwrap(),
                            Some(&format_attr2),
                        )
                        .map_err(|e| {
                            errors.push(e.to_string());
                            errors.clone()
                        })?;
                }
                for (attr_name, info) in &overlay.attribute_information {
                    sheet1
                        .write_string(
                            *attributes_index.get(attr_name.clone().as_str()).unwrap(),
                            (i + 4 - skipped).try_into().unwrap(),
                            info,
                            Some(&format_attr2),
                        )
                        .map_err(|e| {
                            errors.push(e.to_string());
                            errors.clone()
                        })?;
                }
            } else {
                skipped += 1;
            }
        }
    }

    let mut lookup_table: HashMap<String, (usize, usize)> = HashMap::new();
    let lookup_start = oca.capture_base.attributes.len() + 5;
    sheet1
        .write_string(
            (lookup_start).try_into().unwrap(),
            0,
            "Lookup tables:",
            Some(&format_lookup_header),
        )
        .map_err(|e| {
            errors.push(e.to_string());
            errors.clone()
        })?;
    sheet1
        .write_blank(
            (lookup_start).try_into().unwrap(),
            1,
            Some(&format_lookup_header),
        )
        .map_err(|e| {
            errors.push(e.to_string());
            errors.clone()
        })?;
    let mut offset = 0;
    for (attr_name, entries_map) in lookup_entries {
        sheet1
            .write_string(
                (lookup_start + 1 + offset).try_into().unwrap(),
                0,
                &attr_name,
                Some(&format_lookup_attr),
            )
            .map_err(|e| {
                errors.push(e.to_string());
                errors.clone()
            })?;
        lookup_table.insert(
            attr_name.to_string(),
            (
                lookup_start + 3 + offset,
                lookup_start + 2 + offset + entries_map.len(),
            ),
        );
        for (i, (k, v)) in entries_map.iter().enumerate() {
            sheet1
                .write_string(
                    (lookup_start + 2 + offset + i).try_into().unwrap(),
                    0,
                    v,
                    Some(&format_lookup_value),
                )
                .map_err(|e| {
                    errors.push(e.to_string());
                    errors.clone()
                })?;
            sheet1
                .write_string(
                    (lookup_start + 2 + offset + i).try_into().unwrap(),
                    1,
                    k,
                    Some(&format_lookup_value),
                )
                .map_err(|e| {
                    errors.push(e.to_string());
                    errors.clone()
                })?;
        }
        offset += entries_map.len() + 2;
    }

    for (attr_name, (start, end)) in &lookup_table {
        let mut validation = DataValidation::new(
            DataValidationType::ListFormula,
            DataValidationCriteria::None,
            DataValidationErrorType::Stop,
        );
        validation.dropdown = true;
        validation.value_formula = Some(format!("'Schema Description'!$A${}:$A${}", start, end));
        let col_i = *attributes_index.get(attr_name.clone().as_str()).unwrap() - 1;
        let letter = char::from_u32(65 + col_i).unwrap();
        sheet2
            .data_validation_range(
                1,
                (*attributes_index.get(attr_name.clone().as_str()).unwrap() - 1)
                    .try_into()
                    .unwrap(),
                1001,
                (*attributes_index.get(attr_name.clone().as_str()).unwrap() - 1)
                    .try_into()
                    .unwrap(),
                &validation,
            )
            .map_err(|e| {
                errors.push(e.to_string());
                errors.clone()
            })?;

        for r in 1..1001 {
            let formula = format!("=IF(ISBLANK('Data Entry'!${}${}),\"\",VLOOKUP('Data Entry'!${}${},'Schema Description'!$A${}:$B${},2))", letter, r+1, letter, r+1, start, end);
            sheet3
                .write_formula(r, col_i.try_into().unwrap(), &formula, None)
                .map_err(|e| {
                    errors.push(e.to_string());
                    errors.clone()
                })?;
        }
    }

    let mut protection = Protection::new();
    protection.no_select_locked_cells = false;
    protection.no_select_unlocked_cells = false;
    sheet1.protect("oca", &protection);
    sheet3.protect("oca", &protection);
    workbook.close().map_err(|e| {
        errors.push(e.to_string());
        errors.clone()
    })?;

    Ok(())
}
