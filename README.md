# OCA Rust

Collection of libraries and tools related with Overlays Capture Architecture (OCA) build in Rust

## Usage

### Building OCA Bundle from OCA File

Used dependencies:
```toml
oca-rs = "0.3.0-rc.11"
```

```rust
let ocafile = r#"
ADD ATTRIBUTE d=Text i=Text passed=Boolean

ADD META en PROPS name="Entrance credential" description="Entrance credential"

ADD CHARACTER_ENCODING ATTRS d=utf-8 i=utf-8 passed=utf-8
ADD CONFORMANCE ATTRS d=M i=M passed=M
ADD LABEL en ATTRS d="Schema digest" i="Credential Issuee" passed="Passed"
ADD INFORMATION en ATTRS d="Schema digest" i="Credential Issuee" passed="Enables or disables passing"
"#;

let db = oca_rs::data_storage::InMemoryDataStorage::new():
let search_storage_config = oca_rs::repositories::SQLiteConfig::build()
    .path(":memory:".to_string())
    .unwrap();
let mut oca_facade = oca_rs::Facade::new(Box::new(db), search_storage_config);

let oca_bundle = oca_facade.build_from_ocafile(ocafile)?;
let oca_bundle_said = oca_bundle.said.clone().unwrap().to_string();

oca_facade.search_oca_bundle("Ent".to_string(), 10);

oca_facade.get_oca_bundle(oca_bundle_said.clone())?;
oca_facade.get_oca_bundle_steps(oca_bundle_said.clone())?;
oca_facade.get_oca_bundle_ocafile(oca_bundle_said)?;
```

## Workspaces

### oca-ast

OCA AST lib allowing to generate and work with OCA bundle AST

### ocafile

Lib and bin tool to deal with OCAFILE, parsing and creating ocafile

### oca-bundle

Library allowing to build oca bundle