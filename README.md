# OCA Rust

Collection of libraries and tools related with Overlays Capture Architecture (OCA) build in Rust

## Usage

### Building OCA Bundle from OCA File

Used dependencies:
```toml
oca-rs = "0.3.0-rc.5"
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

let db = oca_rs::data_storage::SledDataStorage::open("db_test");
let oca_facade = oca_rs::Facade::new(Box::new(db));

let oca_bundle = oca_facade.build_from_ocafile(ocafile)?;

oca_facade.get_oca_bundle("OCA Bundle SAID".to_string())?;
oca_facade.get_oca_bundle_steps("OCA Bundle SAID".to_string())?;
```

## Workspaces

### oca-ast

OCA AST lib allowing to generate and work with OCA bundle AST

### ocafile

Lib and bin tool to deal with OCAFILE, parsing and creating ocafile

### oca-bundle

Library allowing to build oca bundle