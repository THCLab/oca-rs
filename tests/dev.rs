#[cfg(test)]
mod test {
    use oca_rs::{
        data_storage::{DataStorage, InMemoryDataStorage},
        facade::{build::Error, bundle::{Bundle, BundleElement}},
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
ADD UNIT ATTRS d=kg
"#.to_string();
        let mut facade =
            Facade::new(Box::new(db), Box::new(db_cache), cache_storage_config);

        let result = facade.build_from_ocafile(ocafile)?;

        let mut bundle = Bundle::new();

        bundle.add(result);

        let ocafile_rename = r#"
-- precompiler=transformation
-- source=refs:source_said
-- target=refs:target_said
RENAME ATTRIBUTE d=first_name
"#
        .to_string();
        let result_rename = facade.build_from_ocafile(ocafile_rename)?;
        bundle.add(result_rename);

        let ocafile_rename2 = r#"
-- precompiler=transformation
-- source=refs:source_said
-- target=refs:target_said
RENAME ATTRIBUTE i=last_name
"#
        .to_string();
        let result_rename2 = facade.build_from_ocafile(ocafile_rename2)?;
        bundle.add(result_rename2);

        bundle.fill_said();

        println!("{}", bundle.encode().unwrap());

        Ok(())
    }

    #[test]
    fn dev_fake_to_mock() -> Result<(), Vec<Error>> {
        let db = InMemoryDataStorage::new();
        let db_cache = InMemoryDataStorage::new();
        let cache_storage_config = SQLiteConfig::build().unwrap();
        let ocafile_source = r#"
-- name=fake_standard_patient

ADD ATTRIBUTE name=Text surname=Text height=Numeric weight=Numeric
ADD META en PROPS description="FAKE Standard Patient" name="FAKE Patient"
ADD CHARACTER_ENCODING ATTRS name="utf-8" surname="utf-8" height="utf-8" weight="utf-8"
"#.to_string();

        let ocafile_target = r#"
-- name=mock_standard_patient

ADD ATTRIBUTE first_name=Text last_name=Text hgt=Numeric wgt=Numeric
ADD META en PROPS description="MOCK Standard Patient" name="MOCK Patient"
ADD CHARACTER_ENCODING ATTRS first_name="utf-8" last_name="utf-8" hgt="utf-8" wgt="utf-8"
"#.to_string();
        let mut facade =
            Facade::new(Box::new(db), Box::new(db_cache), cache_storage_config);

        let mut bundle = Bundle::new();

        let result_source = facade.build_from_ocafile(ocafile_source)?;
        let mut source_said: Option<_> = None;
        if let BundleElement::Structural(ref structural) = result_source {
            source_said = structural.said.clone();
        }
        bundle.add(result_source);

        let result_target = facade.build_from_ocafile(ocafile_target)?;
        let mut target_said: Option<_> = None;
        if let BundleElement::Structural(ref structural) = result_target {
            target_said = structural.said.clone();
        }

        let ocafile_link = format!(r#"
-- precompiler=transformation
-- source=refs:{}
-- target=refs:{}

LINK ATTRIBUTE name     -> first_name
LINK ATTRIBUTE surname  -> last_name
LINK ATTRIBUTE height   -> hgt
LINK ATTRIBUTE weight   -> wgt
"#, source_said.unwrap(), target_said.unwrap());
        let result_link = facade.build_from_ocafile(ocafile_link)?;
        println!("{:#?}", result_link);
        bundle.add(result_link);

        bundle.fill_said();

        println!("{}", bundle.encode().unwrap());

        Ok(())
    }
}
