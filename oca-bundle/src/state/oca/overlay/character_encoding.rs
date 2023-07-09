use crate::state::oca::overlay::overlay;
use crate::state::encoding::Encoding;
use serde::{Serialize, ser::SerializeMap};
use said::{sad::SAD, sad::SerializationFormats};

overlay!(CharacterEncoding, attribute_character_encoding, encoding: Encoding);
