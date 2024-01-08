pub mod bundle;
pub mod ocafile;
use crate::data_storage::DataStorage;

pub struct Graph {
    pub db: Box<dyn DataStorage>,
}

#[allow(dead_code)]
impl Graph {
    fn new(db: Box<dyn DataStorage>) -> Self {
        Self { db }
    }

    pub fn add(&self, new: &str, to: Option<&str>) -> Result<(), String> {
        self.db.insert(
            &format!("{}.upscending", new),
            &self.generate_parents_digests(to),
        )?;

        if let Some(to) = to {
            let k = &format!("{}.downscending", to);
            let children_digests: Option<Vec<u8>> = self.db.get(k)?;
            self.db.insert(
                &format!("{}.downscending", to),
                &self.update_children_digests(children_digests.as_deref(), new),
            )?;
        }

        Ok(())
    }

    fn generate_parents_digests(&self, parent_said: Option<&str>) -> Vec<u8> {
        let mut digests: Vec<u8> = Vec::new();
        if let Some(to) = parent_said {
            digests.push(to.len().try_into().unwrap());
            digests.extend(to.as_bytes());
        }
        digests
    }

    fn update_children_digests(&self, children_digest: Option<&[u8]>, new: &str) -> Vec<u8> {
        let mut digests: Vec<u8> = Vec::new();
        if let Some(children_digest) = children_digest {
            digests.extend(children_digest);
        }
        digests.push(new.len().try_into().unwrap());
        digests.extend(new.as_bytes());
        digests
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_storage::{DataStorage, SledDataStorage};

    #[test]
    fn test_history() {
        let db = SledDataStorage::open("db_test");
        let graph = Graph::new(Box::new(db));
        let _ = graph.add("dag1", None);
        let _ = graph.add("dag2", Some("dag1"));
        let _ = graph.add("dag3", Some("dag2"));
        let _ = graph.add("dag4", Some("dag2"));
        let _ = graph.add("dag5", Some("dag4"));

        /* dbg!(graph.db.get("dag1.upscending"));
        dbg!(graph.db.get("dag1.downscending"));
        dbg!(graph.db.get("dag2.upscending"));
        dbg!(graph.db.get("dag2.downscending"));
        dbg!(graph.db.get("dag3.upscending"));
        dbg!(graph.db.get("dag3.downscending"));
        dbg!(graph.db.get("dag4.upscending"));
        dbg!(graph.db.get("dag4.downscending"));
        dbg!(graph.db.get("dag5.upscending"));
        dbg!(graph.db.get("dag5.downscending")); */
    }
}
