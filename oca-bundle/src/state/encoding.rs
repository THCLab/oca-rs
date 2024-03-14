use serde::{Deserialize, Serialize};
use std::str::FromStr;
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
    #[serde(rename = "utf-16")]
    Utf16,
    #[serde(rename = "utf-16be")]
    Utf16Be,
    #[serde(rename = "utf-16le")]
    Utf16Le,
}

impl FromStr for Encoding {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "base64" => Ok(Encoding::Base64),
            "utf-8" => Ok(Encoding::Utf8),
            "iso-8859-1" => Ok(Encoding::Iso8859_1),
            "utf-16" => Ok(Encoding::Utf16),
            "utf-16be" => Ok(Encoding::Utf16Be),
            "utf-16le" => Ok(Encoding::Utf16Le),
            _ => Err(()),
        }
    }
}
