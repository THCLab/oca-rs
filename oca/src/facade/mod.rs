use crate::data_storage::DataStorage;

mod build;
mod fetch;

pub struct Facade {
    db: Box<dyn DataStorage>,
}

impl Facade {
    pub fn new(db: Box<dyn DataStorage>) -> Self {
        Self { db }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data_storage::{SledDataStorage, SledDataStorageConfig};

    #[test]
    fn facade_build_from_ocafile_from_base() -> Result<(), Vec<String>> {
        let db = SledDataStorage::new().config(
            SledDataStorageConfig::build()
                .path("db_test".to_string())
                .unwrap(),
        );
        let ocafile = r#"
ADD ATTRIBUTE d=Text i=Text passed=Boolean
ADD META en PROPS name="Entrance credential" description="Entrance credential"
ADD CHARACTER_ENCODING ATTRS d=utf-8 i=utf-8 passed=utf-8
ADD CONFORMANCE ATTRS d=M i=M passed=M
ADD LABEL en ATTRS d="Schema digest" i="Credential Issuee" passed="Passed"
ADD INFORMATION en ATTRS d="Schema digest" i="Credential Issuee" passed="Enables or disables passing"
"#.to_string();
        let facade = Facade::new(Box::new(db));

        let result = facade.build_from_ocafile(ocafile)?;

        assert_eq!(
            result.said.unwrap().to_string(),
            "EF5ERATRBBN_ewEo9buQbznirhBmvrSSC0O2GIR4Gbfs"
        );
        Ok(())
    }
}
