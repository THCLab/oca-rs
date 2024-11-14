#[cfg(test)]
pub mod dev;

#[cfg(test)]
mod test {
    use oca_rs::{
        data_storage::{DataStorage, InMemoryDataStorage},
        facade::{build::Error, build::ValidationError, bundle::BundleElement},
        repositories::SQLiteConfig,
        Facade,
        HashFunctionCode, SerializationFormats, EncodeBundle
    };

    #[test]
    fn build_from_base() -> Result<(), Vec<Error>> {
        let db = InMemoryDataStorage::new();
        let db_cache = InMemoryDataStorage::new();
        let cache_storage_config = SQLiteConfig::build().unwrap();
        let ocafile = r#"
ADD ATTRIBUTE d=Text i = Text passed=Boolean
ADD META en PROPS name = "Entrance credential" description = "Entrance credential"
ADD CHARACTER_ENCODING ATTRS d=utf-8 i=utf-8 passed=utf-8
ADD CONFORMANCE ATTRS d=M i=M passed=M
ADD LABEL en ATTRS d="Schema digest" i="Credential Issuee" passed="Passed"
ADD INFORMATION en ATTRS d="Schema digest" i="Credential Issuee" passed="Enables or disables passing"
"#.to_string();
        let mut facade = Facade::new(Box::new(db), Box::new(db_cache), cache_storage_config);

        let result = facade.build_from_ocafile(ocafile)?;

        assert!(matches!(result, BundleElement::Mechanics(_)));
        if let BundleElement::Mechanics(result) = result {
            assert_eq!(
                result.said.clone().unwrap().to_string(),
                "EKHBds6myKVIsQuT7Zr23M8Xk_gwq-2SaDRUprvqOXxa"
            );

            let code = HashFunctionCode::Blake3_256;
            let format = SerializationFormats::JSON;
            let oca_bundle_encoded = result.encode(&code, &format).unwrap();
            let oca_bundle_version = String::from_utf8(
                oca_bundle_encoded[6..23].to_vec()
            ).unwrap();
            assert_eq!(oca_bundle_version, "OCAS11JSON000646_");

            let search_result = facade.search_oca_bundle(None, "Ent".to_string(), 10, 1);
            assert_eq!(search_result.metadata.total, 1);
            Ok(())
        } else {
            panic!("Expected BundleElement::Mechanics")
        }
    }

    #[test]
    fn build_from_other_bundle() -> Result<(), Vec<Error>> {
        let db = InMemoryDataStorage::new();
        let db_cache = InMemoryDataStorage::new();
        let cache_storage_config = SQLiteConfig::build().unwrap();
        let mut facade = Facade::new(Box::new(db), Box::new(db_cache), cache_storage_config);
        let other_ocafile = r#"
ADD ATTRIBUTE d=Text i=Text passed=Boolean
ADD META en PROPS name="Entrance credential" description="Entrance credential"
ADD CHARACTER_ENCODING ATTRS d=utf-8 i=utf-8 passed=utf-8
ADD CONFORMANCE ATTRS d=M i=M passed=M
ADD LABEL en ATTRS d="Schema digest" i="Credential Issuee" passed="Passed"
ADD INFORMATION en ATTRS d="Schema digest" i="Credential Issuee" passed="Enables or disables passing"
"#.to_string();
        facade.build_from_ocafile(other_ocafile)?;

        let ocafile = r#"
FROM EObIQDZX7SGy2oPOZue8qCdLWKSq10pXqMWdrXpBXIDa
ADD ATTRIBUTE x=Text
"#
        .to_string();
        let result = facade.build_from_ocafile(ocafile)?;

        if let BundleElement::Mechanics(result) = result {
            assert_eq!(
                result.said.unwrap().to_string(),
                "EAMguWL--P5gad3xZoT2fd-qjoBDVkK82pb7KET1lrS1"
            );
            Ok(())
        } else {
            panic!("Expected BundleElement::Mechanics")
        }
    }

    #[test]
    fn build_with_references() -> Result<(), Vec<Error>> {
        let db = InMemoryDataStorage::new();
        let db_cache = InMemoryDataStorage::new();
        let cache_storage_config = SQLiteConfig::build().unwrap();
        let mut facade = Facade::new(Box::new(db), Box::new(db_cache), cache_storage_config);
        let second_ocafile = r#"
-- name=first
ADD ATTRIBUTE b=Text
"#
        .to_string();
        facade.build_from_ocafile(second_ocafile)?;

        let third_ocafile = r#"
-- name=second
ADD ATTRIBUTE c=Text
"#
        .to_string();

        facade.build_from_ocafile(third_ocafile)?;

        let ocafile = r#"
ADD ATTRIBUTE A=refs:EI_5ohTYptgOrXldUfZujgd7vcXK9zwa6aNqk4-UDWzq
ADD ATTRIBUTE B=refn:first
ADD ATTRIBUTE C=Array[refn:second]
"#
        .to_string();
        let result = facade.build_from_ocafile(ocafile).unwrap();

        assert!(matches!(result, BundleElement::Mechanics(_)));
        if let BundleElement::Mechanics(mechanics) = result {
            assert_eq!(
                mechanics.said.unwrap().to_string(),
                "EGv65yGtFZG5CSRaS4q46dC3UWsW3vycbMFOqPFPvhWi"
            );
        }

        let from_ocafile = r#"
FROM EE15xNvWNy89ZBFhMBukb2kovfO2Y73y1Si2oFFkWFpy
ADD ATTRIBUTE x=Text
"#
        .to_string();

        let result = facade.build_from_ocafile(from_ocafile).unwrap();
        assert!(matches!(result, BundleElement::Mechanics(_)));
        if let BundleElement::Mechanics(mechanics) = result {
            assert_eq!(
                mechanics.said.unwrap().to_string(),
                "EE-Ru8mxNWhql7Q2ibY2-uuK9cIKxR2S9rc-eRkEeBwO"
            );
        }
        let refs = facade.fetch_all_refs().unwrap();

        assert_eq!(refs.len(), 2);
        assert_eq!(
            refs.get("second").unwrap(),
            "EE15xNvWNy89ZBFhMBukb2kovfO2Y73y1Si2oFFkWFpy"
        );

        Ok(())
    }

    #[test]
    fn fail_while_building_from_unknown_reference() {
        let db = InMemoryDataStorage::new();
        let db_cache = InMemoryDataStorage::new();
        let cache_storage_config = SQLiteConfig::build().unwrap();
        let mut facade = Facade::new(Box::new(db), Box::new(db_cache), cache_storage_config);

        let ocafile = r#"
ADD ATTRIBUTE A=refs:EI_5ohTYptgOrXldUfZujgd7vcXK9zwa6aNqk4-UDWzq
ADD ATTRIBUTE B=refn:second
ADD ATTRIBUTE C=Array[refn:third]
"#
        .to_string();
        let result = facade.build_from_ocafile(ocafile);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        let error = errors.first().unwrap();
        let Error::ValidationError(validation_errors) = error;
        let validation_error = validation_errors.first().unwrap();
        assert!(
            matches!(
                validation_error,
                ValidationError::UnknownRefn(_)
            )
        );
    }
}
