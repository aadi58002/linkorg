use std::path::{Path,PathBuf};
use std::fs::File;
use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Links {
    name: String,
    link: String,
    read_till: String,
    line_number: usize,
}

#[derive(Debug, Deserialize, Serialize)]
struct Heading {
    heading: String,
    #[serde(skip_serializing)]
    level: usize,
    heading_or_links: Box<Vec<HeadingOrLinks>>,
}

#[derive(Debug, Deserialize, Serialize)]
enum HeadingOrLinks {
    Heading(Heading),
    Links(Links),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileData{
    tab_title: String,
    heading_or_links: Option<Vec<HeadingOrLinks>>,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn parse_org_file(path: PathBuf) -> FileData{
    let mut data = FileData{
        tab_title: format!("{}",path.file_name().unwrap().to_str().unwrap()).to_string(),
        heading_or_links: None,
    };
    if let Ok(lines) = read_lines(path){
        for line in lines{
            println!("{line:?}");
        }
    }
    data
}
