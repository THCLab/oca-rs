use oca_bundle::state::oca::OCA;

pub fn to_digests(oca: &OCA) -> Vec<u8> {
    let mut digests: Vec<u8> = Vec::new();
    digests.push(oca.capture_base.said.as_bytes().len().try_into().unwrap());
    digests.extend(oca.capture_base.said.as_bytes());

    oca.overlays.iter().for_each(|overlay| {
        if let Some(overlay_type) = OverlayType::from_str(overlay.overlay_type()) {
            digests.push(overlay.said().as_bytes().len().try_into().unwrap());
            digests.push(overlay_type as u8);
            digests.extend(overlay.said().as_bytes());
        } else {
            panic!("Unknown overlay type: {}", overlay.overlay_type());
        }
    });

    digests
}

enum OverlayType {
    CharacterEncoding,
    Format,
    Meta,
    Label,
    Information,
    Standard,
    Conditional,
    Conformance,
    EntryCode,
    Entry,
    Cardinality,
    Unit,
    AttributeMapping,
    EntryCodeMapping,
    UnitMapping,
    Subset,
    CredentialLayout,
    FormLayout
}

impl OverlayType {
  fn from_str(s: &str) -> Option<OverlayType> {
    match s {
      "spec/overlays/character_encoding/1.0" => Some(OverlayType::CharacterEncoding),
      "spec/overlays/format/1.0" => Some(OverlayType::Format),
      "spec/overlays/meta/1.0" => Some(OverlayType::Meta),
      "spec/overlays/label/1.0" => Some(OverlayType::Label),
      "spec/overlays/information/1.0" => Some(OverlayType::Information),
      "spec/overlays/standard/1.0" => Some(OverlayType::Standard),
      "spec/overlays/conditional/1.0" => Some(OverlayType::Conditional),
      "spec/overlays/conformance/1.0" => Some(OverlayType::Conformance),
      "spec/overlays/entry_code/1.0" => Some(OverlayType::EntryCode),
      "spec/overlays/entry/1.0" => Some(OverlayType::Entry),
      "spec/overlays/cardinality/1.0" => Some(OverlayType::Cardinality),
      "spec/overlays/unit/1.0" => Some(OverlayType::Unit),
      "spec/overlays/attribute_mapping/1.0" => Some(OverlayType::AttributeMapping),
      "spec/overlays/entry_code_mapping/1.0" => Some(OverlayType::EntryCodeMapping),
      "spec/overlays/unit_mapping/1.0" => Some(OverlayType::UnitMapping),
      "spec/overlays/subset/1.0" => Some(OverlayType::Subset),
      "spec/overlays/credential_layout/1.0" => Some(OverlayType::CredentialLayout),
      "spec/overlays/form_layout/1.0" => Some(OverlayType::FormLayout),
      _ => None
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_digests() {
        let oca_str = r#"
{
  "capture_base": {
    "type": "spec/capture_base/1.0",
    "digest": "ExwipB2OhUo0NK_2Qfr1XDTE8hpK8fq4m2bbZeWHT3sU",
    "classification": "",
    "attributes": {
      "passed": "Boolean"
    },
    "flagged_attributes": []
  },
  "overlays": [
    {
      "capture_base": "ExwipB2OhUo0NK_2Qfr1XDTE8hpK8fq4m2bbZeWHT3sU",
      "digest": "EXw4pzkwssVAjL-Hl2sAbBk5DPWYPPDK6SfPDeIHUks8",
      "type": "spec/overlays/character_encoding/1.0",
      "default_character_encoding": "utf-8",
      "attribute_character_encoding": {}
    }
  ]
}
"#;
        let oca = oca_bundle::controller::load_oca(&mut oca_str.as_bytes()).unwrap().finalize();
        let digests = to_digests(&oca);
        assert_eq!(digests, vec![44, 69, 122, 67, 72, 88, 77, 86, 70, 120, 75, 83, 122, 81, 57, 49, 85, 68, 86, 65, 121, 56, 68, 45, 67, 57, 49, 73, 111, 99, 117, 101, 79, 100, 54, 70, 53, 97, 75, 101, 115, 122, 53, 95, 77, 44, 0, 69, 83, 53, 100, 102, 102, 79, 74, 55, 45, 114, 106, 87, 112, 66, 113, 104, 87, 105, 66, 82, 101, 51, 113, 100, 110, 111, 89, 54, 70, 69, 107, 105, 119, 105, 67, 100, 80, 104, 117, 74, 112, 69, 52])
    }
}
