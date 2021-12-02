#[cfg(feature = "xls_parser")]
use oca_rust::xls_parser::entries::parse;

#[cfg(feature = "xls_parser")]
#[test]
fn parse_entries_xls() {
    let result = parse(format!(
        "{}/tests/assets/entries_template.xlsx",
        env!("CARGO_MANIFEST_DIR")
    ));

    assert!(result.is_ok());
    let parsed = result.unwrap();
    assert_eq!(parsed.codes.len(), 42);
    assert_eq!(parsed.translations.len(), 3);
}
