use oca_bundle::state::oca::OCABundle;
use indexmap::IndexMap;
use ocaast::ast;
use oca_dag::data_storage::{ DataStorage, SledDataStorage };

fn generate_commands() -> Vec<ast::Command> {
    let mut commands: Vec<ast::Command> = vec![];

    commands.push(ast::Command {
        kind: ast::CommandType::Add,
        object_kind: ast::ObjectKind::CaptureBase,
        content: Some(ast::Content {
            attributes: None,
            properties: None,
        }),
    });

    let mut attributes = IndexMap::new();
    attributes.insert("name".to_string(), ast::NestedValue::Value("Text".to_string()));
    attributes.insert("last_name".to_string(), ast::NestedValue::Value("Text".to_string()));

    let mut properties = IndexMap::new();
    properties.insert("classification".to_string(), ast::NestedValue::Value("12345".to_string()));

    commands.push(ast::Command {
        kind: ast::CommandType::Add,
        object_kind: ast::ObjectKind::CaptureBase,
        content: Some(ast::Content {
            attributes: Some(attributes),
            properties: Some(properties),
        }),
    });

    commands
}

#[test]
fn test() {
    let commands = generate_commands();

    // FROM base
    let oca_bundle: Option<OCABundle> = None;
    let db = SledDataStorage::open("db_test");
    /*
    let oca_dag_manager = OCADagManager::new(oca_bundle, db);

    for command in commands {
        oca_dag_manager.apply(command);
    }
    */
}
