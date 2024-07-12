use std::fs;
use serde_yaml::Result;

mod glance;
use glance::Glance;

fn main() -> Result<()> {
    let yaml_data = fs::read_to_string("glance.yaml").expect("Unable to read file");
    let glance: Glance = serde_yaml::from_str(&yaml_data)?;
    
    // Print parsed data
    println!("{:#?}", glance);
    
    Ok(())
}
