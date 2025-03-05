#[cfg(test)]
mod test {
    use oca_rs::{
        data_storage::{DataStorage, InMemoryDataStorage},
        facade::{build::Error, build::ValidationError},
        repositories::SQLiteConfig,
        EncodeBundle, Facade, HashFunctionCode, SerializationFormats,
    };

    #[test]
    fn build_from_base() -> Result<(), Error> {
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

ADD ATTR_FRAMING \
        id=SNOMEDCT \
        label="Systematized Nomenclature of Medicine Clinical Terms" \
        location="https://bioportal.bioontology.org/ontologies/SNOMEDCT" \
        version=2023AA \
    ATTRS \
        d = {
            "http://purl.bioontology.org/ontology/SNOMEDCT/703503000": {
                "Predicate_id": "skos:exactMatch",
                "Framing_justification": "semapv:ManualMappingCuration"
            },
            "http://purl.bioontology.org/ontology/SNOMEDCT/703503001": {
                "Predicate_id": "skos:exactMatch",
                "Framing_justification": "semapv:ManualMappingCuration"
            }
        }
        i = {
            "http://purl.bioontology.org/ontology/SNOMEDCT/397669002": {
                "Predicate_id": "skos:exactMatch",
                "Framing_justification": "semapv:ManualMappingCuration"
            }
        }
"#.to_string();
        let mut facade = Facade::new(Box::new(db), Box::new(db_cache), cache_storage_config);

        let result = facade.build_from_ocafile(ocafile)?;

        assert_eq!(
            result.said.clone().unwrap().to_string(),
            "ENrE_hCckfWbzflorW6rCZz9wAxcJKSQBMJEVJVAV6rV"
        );

        let code = HashFunctionCode::Blake3_256;
        let format = SerializationFormats::JSON;
        let oca_bundle_encoded = result.encode(&code, &format).unwrap();
        let oca_bundle_version = String::from_utf8(oca_bundle_encoded[6..23].to_vec()).unwrap();
        assert_eq!(oca_bundle_version, "OCAS11JSON0009ac_");

        let search_result = facade.search_oca_bundle(None, "Ent".to_string(), 10, 1);
        assert_eq!(search_result.metadata.total, 1);
        Ok(())
    }

    #[test]
    fn build_from_other_bundle() -> Result<(), Error> {
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

        assert_eq!(
            result.said.unwrap().to_string(),
            "EA4tLtFQd-xCvDBEEGuod6PkgjMdqORDQgbwdqhX1QLA"
        );
        Ok(())
    }

    #[test]
    fn build_with_references() -> Result<(), Error> {
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
        let result = facade.build_from_ocafile(ocafile)?;

        assert_eq!(
            result.said.unwrap().to_string(),
            "EA6ptszucE7ehxcGJSDCHN3t-EG34CB1q_y0ZAckgJuD"
        );

        let from_ocafile = r#"
FROM EKtnSYGee8OwkYKryA_ZEWaYPgJRdncBQrFxaqrFwK1y
ADD ATTRIBUTE x=Text
"#
        .to_string();

        let result = facade.build_from_ocafile(from_ocafile)?;
        assert_eq!(
            result.said.unwrap().to_string(),
            "EOtPb2eo7mPN2AgQin3jgXSWGzz00CV1TNvD9yN79rjq"
        );
        let refs = facade.fetch_all_refs().unwrap();

        assert_eq!(refs.len(), 2);
        assert_eq!(
            refs.get("second").unwrap(),
            "EKtnSYGee8OwkYKryA_ZEWaYPgJRdncBQrFxaqrFwK1y"
        );

        Ok(())
    }

    #[test]
    fn build_with_link() -> Result<(), Error> {
        let db = InMemoryDataStorage::new();
        let db_cache = InMemoryDataStorage::new();
        let cache_storage_config = SQLiteConfig::build().unwrap();
        let mut facade = Facade::new(Box::new(db), Box::new(db_cache), cache_storage_config);
        let first_ocafile = r#"
-- name=first
ADD ATTRIBUTE a=Text
"#
        .to_string();
        facade.build_from_ocafile(first_ocafile)?;

        let second_ocafile = r#"
-- name=second
ADD ATTRIBUTE b=Text
ADD LINK refn:first ATTRS b=a
"#
        .to_string();

        let result = facade.build_from_ocafile(second_ocafile)?;

        assert_eq!(
            result.said.unwrap().to_string(),
            "ENFbq-Nzx23AuPvCRlGT-gMAnMqNnpsGdzfO4oQMhCNn"
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
        let error = result.unwrap_err();
        assert!(matches!(error, Error::ValidationError(_)));
        if let Error::ValidationError(validation_errors) = error {
            let validation_error = validation_errors.first().unwrap();
            assert!(matches!(validation_error, ValidationError::UnknownRefn(_)));
        }
    }
}
