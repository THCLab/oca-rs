use super::Facade;
use crate::data_storage::DataStorage;
use oca_bundle::state::oca::OCABundle;
use oca_bundle::build::OCABuildStep;

impl Facade {
    pub fn get_oca_bundle(&self, said: String) -> Result<OCABundle, Vec<String>> {
        let r = self.db.get(&format!("oca.{}", said)).map_err(|e| vec![format!("{}", e)])?;
        let oca_bundle_str = String::from_utf8(
            r.ok_or_else(|| vec![format!("No OCA Bundle found for said: {}", said)])?
        ).unwrap();
        serde_json::from_str(&oca_bundle_str)
            .map_err(|e| vec![format!("Failed to parse oca bundle: {}", e)])
    }

    pub fn get_oca_bundle_steps(&self, said: String) -> Result<Vec<OCABuildStep>, Vec<String>> {
        let mut said = said;
        #[allow(clippy::borrowed_box)]
        fn extract_operation(db: &Box<dyn DataStorage>, said: &String) -> Result<(String, oca_ast::ast::Command), Vec<String>> {
            let r = db.get(&format!("oca.{}.operation", said))
                .map_err(|e| vec![format!("{}", e)])?
                .ok_or_else(|| vec![format!("No history found for said: {}", said)])?;

            let said_length = r.first().unwrap();
            let parent_said = String::from_utf8_lossy(&r[1..*said_length as usize + 1]).to_string();
            let op_length = r[*said_length as usize + 1];
            let op = String::from_utf8_lossy(&r[*said_length as usize + 2..*said_length as usize + 2 + op_length as usize]).to_string();

            Ok((
                parent_said,
                serde_json::from_str(&op).unwrap()
            ))
        }

        let mut history: Vec<OCABuildStep> = vec![];

        loop {
            let (parent_said, command) = extract_operation(&self.db, &said)?;
            history.push(
                OCABuildStep {
                    parent_said: parent_said.clone().parse().ok(),
                    command,
                    result: self.get_oca_bundle(said.clone()).unwrap(),
                }
            );
            said = parent_said;

            if said.is_empty() {
                break;
            }
        };
        history.reverse();
        Ok(history)
    }
}
