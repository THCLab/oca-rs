use crate::state::{attribute::Attribute, encoding::Encoding, oca::overlay::{overlay, Overlay}};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;
use paste::paste;

overlay!(CharacterEncoding, attribute_character_encoding, encoding: Encoding);
