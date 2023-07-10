use std::str::FromStr;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Encoding {
    #[serde(rename = "base64")]
    Base64,
    #[serde(rename = "utf-8")]
    Utf8,
    #[serde(rename = "iso-8859-1")]
    Iso8859_1,
}

impl FromStr for Encoding {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "base64" => Ok(Encoding::Base64),
            "utf-8" => Ok(Encoding::Utf8),
            "iso-8859-1" => Ok(Encoding::Iso8859_1),
            _ => Err(()),
        }
    }
}
