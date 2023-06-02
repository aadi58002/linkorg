use std::{fs,path::{PathBuf,Path}, io::Write};
use serde::{Serialize,Deserialize};
use dirs;

#[derive(Serialize,Deserialize,Default,Debug)]
pub struct Config{
    pub notes_dir: PathBuf,
}

fn expand_tilde(path: &PathBuf) -> Option<PathBuf> {
    if !path.starts_with("~") {
        return Some(path.to_path_buf());
    }
    if path == Path::new("~") {
        return dirs::home_dir();
    }
    dirs::home_dir().map(|mut home| {
        if home == Path::new("/") {
            // Corner case: `home` root directory;
            // don't prepend extra `/`, just drop the tilde.
            path.strip_prefix("~").unwrap().to_path_buf()
        } else {
            home.push(path.strip_prefix("~/").unwrap());
            home
        }
    })
}

fn create_config(config_path: &PathBuf){
    let config_dir = config_path.parent().expect("Unable to get the parent directory from config path");
    let _ = std::fs::create_dir_all(config_dir);
    let default_config = toml::to_string(&Config::default()).unwrap();
    let mut config_file = std::fs::File::create(config_path).expect("Unable to create config file");
    config_file.write_all(default_config.as_bytes()).expect("Unable to write default config to the config file");
}

pub fn parse_config(config_path: PathBuf) -> Config{

    if !config_path.exists(){
        create_config(&config_path);
    }
    
    
    let mut config  = match fs::read_to_string(config_path){
        Ok(val) => toml::from_str(val.as_str()).unwrap(),
        Err(_) => Config::default(),
    };

    config.notes_dir = expand_tilde(&config.notes_dir).unwrap();
    return config;
}
