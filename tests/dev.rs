#[cfg(test)]
mod test {
    use oca_rs::{
        data_storage::{DataStorage, InMemoryDataStorage},
        facade::{build::Error, bundle::Bundle},
        repositories::SQLiteConfig,
        Facade,
    };

    #[test]
    fn dev() -> Result<(), Vec<Error>> {
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
        let mut facade =
            Facade::new(Box::new(db), Box::new(db_cache), cache_storage_config);

        let result = facade.build_from_ocafile(ocafile)?;

        let mut bundle = Bundle::new();

        bundle.add(result);

        let ocafile_rename = r#"
-- precompiler=transformation
RENAME ATTRIBUTE d=first_name
"#
        .to_string();
        let result_rename = facade.build_from_ocafile(ocafile_rename)?;
        bundle.add(result_rename);

        let ocafile_rename2 = r#"
-- precompiler=transformation
RENAME ATTRIBUTE i=last_name
"#
        .to_string();
        let result_rename2 = facade.build_from_ocafile(ocafile_rename2)?;
        bundle.add(result_rename2);

        bundle.fill_said();

        println!("{}", bundle.encode().unwrap());

        Ok(())
    }
}
