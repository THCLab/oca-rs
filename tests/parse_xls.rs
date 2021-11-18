#[cfg(feature = "xls_parser")]
use oca_rust::state::validator::Validator;
#[cfg(feature = "xls_parser")]
use oca_rust::xls_parser::parse;

#[cfg(feature = "xls_parser")]
#[test]
fn parse_xls() {
    let result = parse(format!(
        "{}/tests/assets/oca_template.xlsx",
        env!("CARGO_MANIFEST_DIR")
    ));

    assert!(result.is_ok());
    let parsed = result.unwrap();
    assert_eq!(parsed.languages.len(), 2);

    let oca = parsed.oca;
    assert_eq!(oca.capture_base.attributes.len(), 18);
    assert_eq!(oca.capture_base.pii.len(), 3);

    let validator = Validator::new().enforce_translations(vec!["en".to_string(), "zh".to_string()]);
    let validation_result = validator.validate(&oca);
    assert!(validation_result.is_ok());
}
