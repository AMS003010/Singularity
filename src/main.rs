use std::fs;
use serde_yaml::Result;

mod singularity;
use singularity::Config;

fn main() -> Result<()> {
    let yaml_data = fs::read_to_string("singularity.yaml").expect("Unable to read file");
    let singularity: Result<Config> = serde_yaml::from_str(&yaml_data);
    match singularity {
        Ok(value) => println!("{:#?}",value),
        Err(err) => println!("Error in parsing singularity.yaml {}", err),
    }
    Ok(())
}