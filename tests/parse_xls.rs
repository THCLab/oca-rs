#[cfg(feature = "xls_parser")]
use oca_rust::state::validator::Validator;
#[cfg(feature = "xls_parser")]
use oca_rust::xls_parser::parse;

#[cfg(feature = "xls_parser")]
#[test]
fn parse_xls() {
    let parsed = parse(format!(
        "{}/examples/oca_template.xlsx",
        env!("CARGO_MANIFEST_DIR")
    ));

    assert_eq!(parsed.oca_list.len(), 3);
    assert_eq!(parsed.languages.len(), 2);

    let oca = parsed.oca_list.get(0).unwrap();
    assert_eq!(oca.capture_base.attributes.len(), 10);
    assert_eq!(oca.capture_base.pii.len(), 3);

    let validator = Validator::new().enforce_translations(vec!["en".to_string(), "zh".to_string()]);
    let validation_result = validator.validate(&oca);
    assert!(validation_result.is_ok());
}
