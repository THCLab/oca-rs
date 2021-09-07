use super::io::*;
use crate::state::Bundle;
use std::io::Read;

pub type GenericError = Box<dyn std::error::Error + Sync + Send>;
pub type GenericResult<T> = Result<T, GenericError>;

pub fn load(source: &mut dyn Read) -> GenericResult<()> {
    /*
    let v: Bundle = serde_json::from_reader(source)?;
    println!("{:?}", v);
    */

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::load;

    #[test]
    fn loads_json_from_str() {
        let data = r#"
 { 
 "capture_base": {"type": "abc", "classification": "def", "attributes": {}, "pii": []},
 "overlays": []
 }
        "#;
        load(&mut data.as_bytes()).unwrap();
    }
}
