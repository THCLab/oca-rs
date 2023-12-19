use crate::data_storage::Namespace;
use oca_bundle::state::oca::OCABundle;
use oca_ast::ast::{ObjectKind, Content, BundleContent, RefValue, CaptureContent};
use serde::{Serialize, ser::SerializeStruct};
use std::collections::HashSet;

use super::Facade;

impl Facade {
    pub fn explore(&self, said: String) -> Option<Relationship> {
        let relations_u8_res = self.db.get(Namespace::OCARelations, &said);

        match relations_u8_res {
            Ok(Some(relations_u8)) => {
                let relationship: Relationship = relations_u8.into();
                Some(Relationship {
                    base_object: OCAObject::new(self, said),
                    relations: relationship.relations,
                })
            }
            _ => None,
        }
    }

    fn insert_oca_objects_metadata(
        &mut self,
        oca_bundle: OCABundle,
    ) -> Result<(), String> {
        self.db.insert(
            Namespace::OCARelations,
            &format!("{}.metadata", oca_bundle.said.clone().unwrap()),
            &[ObjectKind::OCABundle(BundleContent { said: oca_ast::ast::ReferenceAttrType::Reference(RefValue::Name("".to_string())) }).into()],
        )?;
        self.db.insert(
            Namespace::OCARelations,
            &format!(
                "{}.metadata",
                oca_bundle.capture_base.said.clone().unwrap()
            ),
            &[ObjectKind::CaptureBase( CaptureContent { attributes: None, properties: None }).into()],
        )?;
        oca_bundle.overlays.iter().for_each(|overlay| {
            let _ = self.db.insert(
                Namespace::OCARelations,
                &format!("{}.metadata", overlay.said().clone().unwrap()),
                &[ObjectKind::Overlay(overlay.overlay_type().clone(), Content { attributes: None, properties: None }).into()],
            );
        });

        Ok(())
    }

    pub fn add_relations(
        &mut self,
        oca_bundle: OCABundle,
    ) -> Result<(), String> {
        self.insert_oca_objects_metadata(oca_bundle.clone())?;

        let oca_bundle_said = oca_bundle.said.clone().unwrap().to_string();
        let capture_base_said =
            oca_bundle.capture_base.said.clone().unwrap().to_string();

        let mut oca_bundle_rel = self
            .explore(oca_bundle_said.clone())
            .unwrap_or(Relationship::new(OCAObject::new(
                self,
                oca_bundle_said.clone(),
            )));
        oca_bundle_rel
            .add_relation(OCAObject::new(self, capture_base_said.clone()));

        let mut capture_base_rel = self
            .explore(capture_base_said.clone())
            .unwrap_or(Relationship::new(OCAObject::new(
                self,
                capture_base_said.clone(),
            )));
        capture_base_rel
            .add_relation(OCAObject::new(self, oca_bundle_said.clone()));

        for overlay in oca_bundle.overlays {
            let overlay_said = overlay.said().clone().unwrap().to_string();

            oca_bundle_rel
                .add_relation(OCAObject::new(self, overlay_said.clone()));
            capture_base_rel
                .add_relation(OCAObject::new(self, overlay_said.clone()));

            let mut overlay_rel = self.explore(overlay_said.clone()).unwrap_or(
                Relationship::new(OCAObject::new(self, overlay_said.clone())),
            );

            overlay_rel
                .add_relation(OCAObject::new(self, oca_bundle_said.clone()));
            overlay_rel
                .add_relation(OCAObject::new(self, capture_base_said.clone()));
            let overlay_rel_u8: Vec<u8> = overlay_rel.clone().into();
            self.db.insert(
                Namespace::OCARelations,
                &overlay_said.clone(),
                &overlay_rel_u8,
            )?;
        }

        let oca_bundle_rel_u8: Vec<u8> = oca_bundle_rel.clone().into();
        self.db.insert(
            Namespace::OCARelations,
            &oca_bundle_rel.base_object.said,
            &oca_bundle_rel_u8,
        )?;

        let capture_base_rel_u8: Vec<u8> = capture_base_rel.clone().into();
        self.db.insert(
            Namespace::OCARelations,
            &capture_base_rel.base_object.said,
            &capture_base_rel_u8,
        )?;

        Ok(())
    }

    fn object_type(&self, said: String) -> ObjectKind {
        let object_type = self
            .db
            .get(Namespace::OCARelations, &format!("{}.metadata", said))
            .unwrap();

        (*object_type.unwrap().first().unwrap()).into()
    }
}

#[derive(Clone)]
pub struct Relationship {
    pub base_object: OCAObject,
    pub relations: HashSet<OCAObject>,
}

impl Relationship {
    fn new(base_object: OCAObject) -> Self {
        Self {
            base_object,
            relations: HashSet::new(),
        }
    }

    fn add_relation(&mut self, object: OCAObject) {
        self.relations.insert(object);
    }
}

impl From<Relationship> for Vec<u8> {
    fn from(val: Relationship) -> Self {
        let mut result: Vec<u8> = Vec::new();

        val.relations.iter().for_each(|object| {
            result.push(object.object_type.clone().into());
            result.push(object.said.len().try_into().unwrap());
            result.extend(object.said.as_bytes());
        });

        result
    }
}

impl From<Vec<u8>> for Relationship {
    fn from(val: Vec<u8>) -> Self {
        let mut result = Relationship::new(OCAObject {
            said: "".to_string(),
            object_type: ObjectKind::OCABundle( BundleContent { said: oca_ast::ast::ReferenceAttrType::Reference(RefValue::Name("".to_string())) } ),
        });

        let mut tmp_val = val.clone();
        while !tmp_val.is_empty() {
            let said_len = tmp_val[1];
            let (o_u8, split_val) = tmp_val.split_at(2 + said_len as usize);
            result.add_relation(OCAObject::from(o_u8.to_vec()));
            tmp_val = split_val.to_vec();
        }

        result
    }
}

impl From<Vec<u8>> for OCAObject {
    fn from(val: Vec<u8>) -> Self {
        let object_type = val[0];
        let said_len = val[1];
        let said =
            String::from_utf8(val[2..2 + said_len as usize].to_vec()).unwrap();
        Self {
            said,
            object_type: object_type.into(),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct OCAObject {
    pub said: String,
    pub object_type: ObjectKind,
}

impl Serialize for OCAObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        #[derive(Serialize)]
        struct OverlayMetadata {
            kind: ObjectKind,
        }

        let mut state = serializer.serialize_struct("OCAObject", 3)?;
        state.serialize_field("said", &self.said)?;
        match &self.object_type {
            ObjectKind::OCABundle(_) |
            ObjectKind::CaptureBase(_) =>  {
                state.serialize_field("object_type", &self.object_type)?
            },
            ObjectKind::Overlay(_,_) => {
                state.serialize_field("object_type", "Overlay")?;
                let overlay_metadata = OverlayMetadata {
                    kind: self.object_type.clone(),
                };
                state.serialize_field("metadata", &overlay_metadata)?
            }
        }
        state.end()
    }
}

impl OCAObject {
    fn new(facade: &Facade, said: String) -> Self {
        Self {
            said: said.clone(),
            object_type: facade.object_type(said),
        }
    }
}
