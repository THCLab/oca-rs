#[cfg(feature = "xls_parser")]
use oca_rust::state::validator::Validator;
#[cfg(feature = "xls_parser")]
use oca_rust::xls_parser::oca::parse;

#[cfg(feature = "xls_parser")]
#[test]
fn parse_oca_xls() {
    let result = parse(
        format!(
            "{}/tests/assets/oca_template.xlsx",
            env!("CARGO_MANIFEST_DIR")
        ),
        None,
        None,
    );

    assert!(result.is_ok());
    let parsed = result.unwrap();
    assert_eq!(parsed.languages.len(), 2);

    let oca = parsed.oca_builder.finalize();
    assert_eq!(oca.capture_base.attributes.len(), 17);
    assert_eq!(oca.capture_base.flagged_attributes.len(), 3);

    let validator = Validator::new().enforce_translations(vec!["en".to_string(), "zh".to_string()]);
    let validation_result = validator.validate(&oca);
    assert!(validation_result.is_ok());
}
