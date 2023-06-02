use std::path::PathBuf;

use glob::glob;

pub fn list_matching_paths(path: PathBuf) -> Vec<PathBuf>{
    let files: Vec<PathBuf> = glob(path.to_str().expect("Unable to convert OS path to str")).expect("Failed to read glob pattern").map(|file| {
        file.unwrap()
    }).collect();
    files
}

pub fn find_org_files(path: PathBuf) {
    println!("{:?}",list_matching_paths(path.join("**/*.org")));
}
