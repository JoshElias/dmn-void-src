use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub dir_name: String,
    pub repo_urls: Vec<String>,
}

pub fn read_config(config_path: &PathBuf) -> Config {
    let contents = fs::read_to_string(config_path).expect("Error reading config file");
    match toml::from_str::<Config>(&contents) {
        Ok(v) => {
            println!("Repo urls: {:?}", v.repo_urls);
            println!("Directory Name: {:?}", v.dir_name);
            println!("Successfully serialized config");
            v
        }
        Err(_e) => panic!("Could not deserialize config"),
    }
}
