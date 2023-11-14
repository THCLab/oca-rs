use oca_rs::{
    data_storage::{DataStorage, InMemoryDataStorage},
    facade::build::Error,
    repositories::SQLiteConfig,
    Facade,
};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build_from_base() -> Result<(), Vec<Error>> {
        let db = InMemoryDataStorage::new();
        let db_cache = InMemoryDataStorage::new();
        let cache_storage_config = SQLiteConfig::build().unwrap();
        let ocafile = r#"
ADD ATTRIBUTE d=Text i=Text passed=Boolean
ADD META en PROPS name="Entrance credential" description="Entrance credential"
ADD CHARACTER_ENCODING ATTRS d=utf-8 i=utf-8 passed=utf-8
ADD CONFORMANCE ATTRS d=M i=M passed=M
ADD LABEL en ATTRS d="Schema digest" i="Credential Issuee" passed="Passed"
ADD INFORMATION en ATTRS d="Schema digest" i="Credential Issuee" passed="Enables or disables passing"
"#.to_string();
        let mut facade =
            Facade::new(Box::new(db), Box::new(db_cache), cache_storage_config);

        let result = facade.build_from_ocafile(ocafile)?;

        assert_eq!(
            result.said.unwrap().to_string(),
            "EF5ERATRBBN_ewEo9buQbznirhBmvrSSC0O2GIR4Gbfs"
        );
        Ok(())
    }

    #[test]
    fn build_from_other_bundle() -> Result<(), Vec<Error>> {
        let db = InMemoryDataStorage::new();
        let db_cache = InMemoryDataStorage::new();
        let cache_storage_config = SQLiteConfig::build().unwrap();
        let mut facade =
            Facade::new(Box::new(db), Box::new(db_cache), cache_storage_config);
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
"#.to_string();
        let result = facade.build_from_ocafile(ocafile)?;

        assert_eq!(
            result.said.unwrap().to_string(),
            "EBBLFLhdLLgmVOLJO0G6Bqa4-JhFyP8-E0HikwjuRB6w"
        );
        Ok(())
    }

    #[test]
    fn build_with_references() -> Result<(), Vec<Error>> {
        let db = InMemoryDataStorage::new();
        let db_cache = InMemoryDataStorage::new();
        let cache_storage_config = SQLiteConfig::build().unwrap();
        let mut facade =
            Facade::new(Box::new(db), Box::new(db_cache), cache_storage_config);
        let first_ocafile = r#"
ADD ATTRIBUTE a=Text
"#.to_string();
        facade.build_from_ocafile(first_ocafile)?;
        let second_ocafile = r#"
-- name=other
ADD ATTRIBUTE b=Text
"#.to_string();
        facade.build_from_ocafile(second_ocafile)?;

        let ocafile = r#"
ADD ATTRIBUTE A=refs:EI_5ohTYptgOrXldUfZujgd7vcXK9zwa6aNqk4-UDWzq
ADD ATTRIBUTE B=refn:other
"#.to_string();
        let result = facade.build_from_ocafile(ocafile)?;

        assert_eq!(
            result.said.unwrap().to_string(),
            "EPZNx7Vbl06cYdsAKbRVtgxUOoLcap61Go7ueau1RjEN"
        );
        Ok(())
    }
}
