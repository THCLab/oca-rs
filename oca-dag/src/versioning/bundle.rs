use oca_bundle::state::oca::OCABundle;

struct OCABundleDTO {
    bundle: OCABundle
}

impl OCABundleDTO {
    fn new(bundle: OCABundle) -> Self {
        Self {
            bundle
        }
    }
}

impl Into<Vec<u8>> for OCABundleDTO {
    fn into(self) -> Vec<u8> {
        let mut digests: Vec<u8> = Vec::new();
        if let Some(ref said) = self.bundle.capture_base.said {
            digests.push(said.to_string().as_bytes().len().try_into().unwrap());
            digests.extend(said.to_string().as_bytes());
        }

        self.bundle.overlays.iter().for_each(|overlay| {
            if let Some(overlay_type) = OverlayType::from_str(overlay.overlay_type()) {
                if let Some(ref said) = overlay.said() {
                    digests.push(said.to_string().as_bytes().len().try_into().unwrap());
                    digests.push(overlay_type as u8);
                    digests.extend(said.to_string().as_bytes());
                }
            } else {
                panic!("Unknown overlay type: {}", overlay.overlay_type());
            }
        });

        digests
    }
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
  "version": "OCAB10000023_",
  "said": "EOGGSNS6CMlMfj3rW5ltFOv0RQux9-W7sND8SIMqsAiC",
  "capture_base": {
    "said": "EIJGJmS_P9jwZDamB6cTG9MoXKRu21myjXsMi7GYddDy",
    "type": "spec/capture_base/1.0",
    "classification": "",
    "attributes": {
      "passed": "Boolean"
    },
    "flagged_attributes": []
  },
  "overlays": {}
}
"#;
        let oca = oca_bundle::controller::load_oca(&mut oca_str.as_bytes()).unwrap();
        let digests: Vec<u8> = OCABundleDTO::new(oca).into();
        assert_eq!(digests, vec![44, 69, 73, 74, 71, 74, 109, 83, 95, 80, 57, 106, 119, 90, 68, 97, 109, 66, 54, 99, 84, 71, 57, 77, 111, 88, 75, 82, 117, 50, 49, 109, 121, 106, 88, 115, 77, 105, 55, 71, 89, 100, 100, 68, 121])
    }
}
