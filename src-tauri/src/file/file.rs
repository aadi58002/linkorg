use std::path::PathBuf;
use walkdir::WalkDir;

pub fn find_data_files(path: PathBuf) -> Vec<PathBuf> {
    let files = WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.is_file() {
                    path.extension().and_then(|ext| {
                        if ext == "org" || ext == "md" {
                            Some(path.to_path_buf())
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            })
        })
        .collect();
    files
}
