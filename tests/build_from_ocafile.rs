#[cfg(test)]
mod test {
    use oca_rs::{
        data_storage::{DataStorage, InMemoryDataStorage},
        facade::build::Error,
        repositories::SQLiteConfig,
        Facade,
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

        assert_eq!(
            result.said.unwrap().to_string(),
            "EF5ERATRBBN_ewEo9buQbznirhBmvrSSC0O2GIR4Gbfs"
        );

        let search_result = facade.search_oca_bundle(None, "Ent".to_string(), 10, 1);
        assert_eq!(search_result.metadata.total, 1);
        Ok(())
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
FROM EF5ERATRBBN_ewEo9buQbznirhBmvrSSC0O2GIR4Gbfs
ADD ATTRIBUTE x=Text
"#
        .to_string();
        let result = facade.build_from_ocafile(ocafile)?;

        assert_eq!(
            result.said.unwrap().to_string(),
            "EBBLFLhdLLgmVOLJO0G6Bqa4-JhFyP8-E0HikwjuRB6w"
        );
        Ok(())
    }

    #[cfg(feature = "local-references")]
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

        assert_eq!(
            result.said.unwrap().to_string(),
            "EPJQXAl5fa9PKHjAtbX7EsdnFDXZCwcC7iYBt5YmdbqU"
        );
        let from_ocafile = r#"
FROM EJ9jPoPyZxJNtQsWI_yiHowfbP1B9SDOvlsSxlHbn9oW
ADD ATTRIBUTE x=Text
"#
        .to_string();

        let result = facade.build_from_ocafile(from_ocafile).unwrap();

        assert_eq!(
            result.said.unwrap().to_string(),
            "ENyO7FUBx7oILUYt8FwmLaDVmvOZGETXWHICultMSEpW"
        );
        let refs = facade.fetch_all_refs().unwrap();

        assert_eq!(refs.len(), 2);
        assert_eq!(
            refs.get("second").unwrap(),
            "EJ9jPoPyZxJNtQsWI_yiHowfbP1B9SDOvlsSxlHbn9oW"
        );

        Ok(())
    }

    #[cfg(feature = "local-references")]
    #[test]
    #[should_panic(expected = "Reference not found")]
    fn panic_while_building_from_unknown_reference() {
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
        let _ = facade.build_from_ocafile(ocafile);
    }
}
