use std::path::PathBuf;

use glob::glob;

pub fn find_org_files(path: PathBuf) -> Vec<PathBuf> {
    let path = path.join("**/*.org");
    let files: Vec<PathBuf> = glob(path.to_str().expect("Unable to convert OS path to str"))
        .expect("Failed to read glob pattern")
        .map(|file| file.unwrap())
        .collect();
    files
}
