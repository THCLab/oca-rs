use core::str::FromStr;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum Language {
    #[serde(rename = "en_EN")]
    En,
    #[serde(rename = "en_US")]
    EnUs,
    #[serde(rename = "pl_PL")]
    Pl,
    #[serde(rename = "zh_ZH")]
    Zh,
}

impl FromStr for Language {
    type Err = ();

    fn from_str(input: &str) -> Result<Language, Self::Err> {
        match input {
            "0" => Ok(Language::En),
            "En" => Ok(Language::En),
            "en_EN" => Ok(Language::En),
            "1" => Ok(Language::EnUs),
            "EnUs" => Ok(Language::EnUs),
            "en_US" => Ok(Language::EnUs),
            "2" => Ok(Language::Pl),
            "Pl" => Ok(Language::Pl),
            "pl_PL" => Ok(Language::Pl),
            "3" => Ok(Language::Zh),
            "Zh" => Ok(Language::Zh),
            "zh_ZH" => Ok(Language::Zh),
            _ => Err(()),
        }
    }
}
