use std::fs;

use crate::OutputFormat;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut rdr = csv::Reader::from_path(input)?;
    let mut records = Vec::with_capacity(128);
    // csv的列名
    let headers = rdr.headers()?.clone();

    for result in rdr.records() {
        let record = result?;
        let json_value = headers
            .iter()
            .zip(record.iter()) // 列名与值zip起来
            .collect::<serde_json::Value>();
        records.push(json_value);
    }
    // for rc in &records {
    //     println!("{:?}", rc);
    // }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&records)?,
        OutputFormat::Toml => return Err(anyhow::anyhow!("Not supported format")),
        OutputFormat::Yaml => serde_yaml::to_string(&records)?,
    };
    fs::write(output, content)?;
    Ok(())
}
