use super::io::*;
use std::io::Read;

pub fn load(source: &mut dyn Read) {
    let mut buffer = String::new();
    source.read_to_string(&mut buffer);
    println!("{}", buffer);
}

#[cfg(test)]
mod tests {
    use super::load;

    #[test]
    fn loads_json_from_str() {
        let data = r#"
 {"schema_base":{"@context":"https://odca.tech/v1","name":"Presentation Status","type":"spec/schema_base/1.0","description":"","classification":"","issued_by":"","attributes":{"presentation_urn":"Text","verified":"Boolean"},"pii_attributes":[]},"overlays":[{"@context":"https://odca.tech/overlays/v1","type":"spec/overlay/label/1.0","issued_by":"","role":"","purpose":"","schema_base":"hl:4YjW5R27kqiCDX5Tq6d3kNXmbK7g9skt6jjYW5iVZbL1","language":"en_US","attr_labels":{"presentation_urn":"Presentation URN","verified":"Verified"},"attr_categories":["_cat-1_"],"cat_labels":{"_cat-1_":""},"cat_attributes":{"_cat-1_":["presentation_urn","verified"]}},{"@context":"https://odca.tech/overlays/v1","type":"spec/overlay/character_encoding/1.0","issued_by":"","role":"","purpose":"","schema_base":"hl:4YjW5R27kqiCDX5Tq6d3kNXmbK7g9skt6jjYW5iVZbL1","default_character_encoding":"utf-8","attr_character_encoding":{"presentation_urn":"utf-8","verified":"utf-8"}}]}
        "#;
        load(&mut data.as_bytes());
    }
}
