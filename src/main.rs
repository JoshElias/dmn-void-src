use git2::{build::RepoBuilder, FetchOptions, Repository};
use std::{
    env, fs,
    path::{Path, PathBuf},
};
use url::Url;

extern crate serde;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Deserialize)]
struct Config {
    dir_name: String,
    repo_urls: Vec<String>,
}

fn main() {
    let cwd = get_cwd();
    let config_path = cwd.join("config.toml");

    // match cwd.to_str() {
    //     Some(s) => println!("CWD: {}", s),
    //     None => println!("CWD not a valid UTF-8 sequence"),
    // }

    let contents = fs::read_to_string(config_path).expect("Error reading config file");
    let config: Config = toml::from_str(&contents).expect("Could not deserialize config");

    println!("Repo urls: {:?}", config.repo_urls);
    println!("Directory Name: {:?}", config.dir_name);

    let home_dir = match env::var("HOME") {
        Ok(v) => {
            println!("Home Directory: {}", v);
            v
        }
        Err(_e) => {
            panic!("Unable to get user's home  directory")
        }
    };
    let home_path = PathBuf::from(home_dir);
    let void_repo_path_buf = home_path.join(config.dir_name);
    println!("Void Repo Path: {}", void_repo_path_buf.display());
    let void_repo_path = void_repo_path_buf.as_path();

    if void_repo_path.exists() {
        println!("Void Repo already exists");
        match fs::remove_dir_all(void_repo_path) {
            Ok(()) => println!("Successfully deleted old app dir"),
            Err(_e) => panic!("Unable to delete directory"),
        }
    }

    match fs::create_dir_all(void_repo_path) {
        Ok(()) => println!("Successfully created new app dir"),
        Err(_e) => panic!("Unable to create new app dir"),
    }

    // let dest_path = match void_repo_path_buf.to_str() {
    //     Some(v) => v,
    //     None => panic!("Unable to convert void repo path to string"),
    // };

    let mut options = FetchOptions::new();
    options.depth(1);
    let mut builder = RepoBuilder::new();
    builder.fetch_options(options);

    for url_str in config.repo_urls {
        let url = Url::parse(&url_str).expect("Failed to parse URL");

        let base_name = url
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("unknown");

        let file_stem = Path::new(base_name)
            .file_stem()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("unknown");

        let new_void_path_buf = void_repo_path_buf.join(file_stem);
        // let new_void_path = match new_void_path_buf.to_str() {
        //     Some(v) => v,
        //     None => panic!("Uh ooooh"),
        // };

        match builder.clone(&url_str, new_void_path_buf.as_path()) {
            Ok(_r) => println!("Successfully cloned repo: {}", url),
            Err(_e) => panic!("Faild to clone repo"),
        }
    }
}

fn get_cwd() -> PathBuf {
    match env::current_dir() {
        Ok(dir) => {
            return dir;
        }
        Err(_e) => {
            panic!("Unable to get the current working directory");
        }
    }
}
