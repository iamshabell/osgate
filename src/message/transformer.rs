use crate::lib::parser::XmlSchemaParser;

pub struct Transformer {
    parser: XmlSchemaParser,
    schema: String,
}

impl Transformer {
    pub fn new(schema: &str) -> Self {
        Self {
            parser: XmlSchemaParser::new(schema),
            schema: schema.to_string(),
        }
    }

    pub fn xml_to_json(&mut self) -> Result<String, serde_xml_rs::Error> {
        let _ = self.parser.parse();

        let result = self.parser.transform(&self.schema).unwrap();

        Ok(result)
    }

    pub fn json_to_xml(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let _ = self.parser.parse();

        let result = self.parser.json_to_xml(&self.schema)?;

        Ok(result)
    }
}
