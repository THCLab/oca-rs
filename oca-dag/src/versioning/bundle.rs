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
            if let Some(ref said) = overlay.said() {
                digests.push(said.to_string().as_bytes().len().try_into().unwrap());
                // digests.push(overlay.overlay_type().into());
                digests.extend(said.to_string().as_bytes());
            }
        });

        digests
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
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
