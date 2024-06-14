use crate::config::Config;
use git2::{build::RepoBuilder, FetchOptions};
use std::path::Path;
use url::Url;

pub fn clone_repos(config: &Config, install_path: &Path) {
    let mut options = FetchOptions::new();
    options.depth(1);
    let mut builder = RepoBuilder::new();
    builder.fetch_options(options);

    for url_str in &config.repo_urls {
        let url = Url::parse(&url_str).expect("Failed to parse URL");

        let segments = url.path_segments().unwrap();
        let base_name = segments.last().unwrap();
        let file_stem = Path::new(base_name).file_stem().unwrap();
        let repo_install_path = install_path.join(file_stem);

        match builder.clone(&url_str, &repo_install_path) {
            Ok(_r) => println!("Successfully cloned repo: {}", url),
            Err(_e) => panic!("Faild to clone repo"),
        }
    }
}
