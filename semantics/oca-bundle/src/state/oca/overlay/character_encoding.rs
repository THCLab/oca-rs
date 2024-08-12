use crate::state::encoding::Encoding;
use crate::state::oca::overlay::overlay;
use said::derivation::HashFunctionCode;
use said::{sad::SerializationFormats, sad::SAD};
use serde::{ser::SerializeMap, Serialize};

overlay!(CharacterEncoding, attribute_character_encoding, encoding: Encoding);
