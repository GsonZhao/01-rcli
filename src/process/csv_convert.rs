use csv::Reader;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    pub name: String,
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub kit: u32,
}

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);

    let headers = reader.headers()?.clone();

    for result in reader.records() {
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        OutputFormat::Toml => {
            // TOML不支持数组作为顶级结构，需要包装在对象中
            let wrapper = std::collections::HashMap::from([("data".to_string(), ret)]);
            toml::to_string(&wrapper)?
        }
    };

    std::fs::write(output, content)?;
    Ok(())
}
