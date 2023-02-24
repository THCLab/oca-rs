use crate::state::{attribute::Attribute, oca::overlay::{overlay, Overlay}};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;
use paste::paste;

overlay!(Cardinality, attribute_cardinality, cardinality: String);
