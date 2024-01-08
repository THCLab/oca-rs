use indexmap::IndexMap;
use oca_ast::ast;
use oca_bundle::state::oca::OCABundle;
use oca_dag::data_storage::{DataStorage, SledDataStorage};

fn generate_commands() -> Vec<ast::Command> {
    let mut commands: Vec<ast::Command> = vec![];

    commands.push(ast::Command {
        kind: ast::CommandType::Add,
        object_kind: ast::ObjectKind::CaptureBase(ast::CaptureContent {
            attributes: None,
            properties: None,
        }),
    });

    let mut attributes = IndexMap::new();
    attributes.insert(
        "name".to_string(),
        ast::NestedAttrType::Value(ast::AttributeType::Text),
    );
    attributes.insert(
        "last_name".to_string(),
        ast::NestedAttrType::Value(ast::AttributeType::Text),
    );

    let mut properties = IndexMap::new();
    properties.insert(
        "classification".to_string(),
        ast::NestedValue::Value("12345".to_string()),
    );

    commands.push(ast::Command {
        kind: ast::CommandType::Add,
        object_kind: ast::ObjectKind::CaptureBase(ast::CaptureContent {
            attributes: Some(attributes),
            properties: Some(properties),
        }),
    });

    commands
}

#[test]
fn test() {
    let _commands = generate_commands();

    // FROM base
    let _oca_bundle: Option<OCABundle> = None;
    let _db = SledDataStorage::open("db_test");
    /*
    let oca_dag_manager = OCADagManager::new(oca_bundle, db);

    for command in commands {
        oca_dag_manager.apply(command);
    }
    */
}
