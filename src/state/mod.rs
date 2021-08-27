use std::collections::HashMap;

pub enum CaptureBase {
    Type(String),
    Classification(String),
    Attributes(HashMap<String, AttributeType>),
    Pii(Vec<String>),
}

pub enum AttributeType {
    Text,
    Number,
}
