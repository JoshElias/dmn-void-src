use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub fn get_install_path(dir_name: &String) -> PathBuf {
    let home_dir = env::var("HOME").expect("Unable to get user's home  directory");
    let home_path = PathBuf::from(home_dir);
    home_path.join(&dir_name)
}

pub fn prep_install_dir(install_path: &Path) {
    if install_path.exists() {
        println!("Void Repo already exists");
        match fs::remove_dir_all(install_path) {
            Ok(()) => println!("Successfully deleted old app dir"),
            Err(_e) => panic!("Unable to delete directory"),
        }
    }

    match fs::create_dir_all(install_path) {
        Ok(()) => println!("Successfully created new app dir"),
        Err(_e) => panic!("Unable to create new app dir"),
    }
}
