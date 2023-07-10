# OCA Rust

Collection of libraries and tools related with Overlays Capture Architecture (OCA) build in Rust

## Usage

### Building OCA Bundle from OCA File

Used dependencies:
```toml
oca-file = "0.1.1"
oca-bundle = "0.3.0-rc.3"
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

let ast = oca_file::ocafile::parse_from_string(ocafile.to_string());

let bundle = oca_bundle::state::oca::OCABundle::from_ast(ast);
```

## Workspaces

### oca-ast

OCA AST lib allowing to generate and work with OCA bundle AST

### ocafile

Lib and bin tool to deal with OCAFILE, parsing and creating ocafile

### oca-bundle

Library allowing to build oca bundle