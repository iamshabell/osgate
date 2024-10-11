use serde_json::Value;
use serde_xml_rs::from_str;

pub fn xml_to_json(xml: &str) -> Result<Value, serde_xml_rs::Error> {
    let json_data: Value = from_str(xml)?;
    Ok(json_data)
}
