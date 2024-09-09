use std::fs;

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

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut rdr = csv::Reader::from_path(input)?;
    let records = rdr
        .deserialize()
        .map(|record| record.unwrap())
        .collect::<Vec<Player>>();
    // for rc in &records {
    //     println!("{:?}", rc);
    // }
    let json = serde_json::to_string_pretty(&records)?;
    fs::write(output, json)?;
    Ok(())
}
