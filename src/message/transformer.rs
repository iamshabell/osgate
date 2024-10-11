use serde_json::Value;
use serde_xml_rs::from_str;

pub fn xml_to_json(xml: &str) -> Result<Value, serde_xml_rs::Error> {
    let json_data: Value = from_str(xml)?;
    println!("Converted XML to JSON: {:?}", json_data);
    Ok(json_data)
}
