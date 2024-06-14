use crate::config;
use crate::path;
use crate::repos;
use std::env;

pub fn install() {
    let config_path = env::current_dir()
        .expect("Unable to get the current working directory")
        .join("config.toml");
    let config = config::read_config(&config_path);
    let install_path = path::get_install_path(&config.dir_name);
    println!("Install Path: {}", install_path.display());

    path::prep_install_dir(&install_path);
    repos::clone_repos(&config, &install_path);
}
