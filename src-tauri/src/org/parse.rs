use lazy_static::lazy_static;
use regex::Regex;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
struct Links {
    name: String,
    link: String,
    read_till: String,
    likeability: Option<String>,
    line_number: usize,
}

#[derive(Debug, Deserialize, Serialize)]
struct Heading {
    title: String,
    #[serde(skip_serializing)]
    level: usize,
    line_number: usize,
    heading: Vec<Heading>,
    links: Vec<Links>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileData {
    tab_title: String,
    heading: Vec<Heading>,
    links: Vec<Links>,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_link(content: &String, line_number: usize) -> Option<Links> {
    lazy_static! {
        static ref RE_HEADING: Regex = Regex::new(r"^\[\[(.*)\]\[(.*)\]\] *(?:\([^()]*[^[Rr]ead]\))?(?:\(([^()]*[Rr]ead)\))?.*-- ?after ?(\w*)").unwrap();
    }
    if let Some(val) = RE_HEADING.captures(content.as_str()) {
        let likeability = match val.get(3).map(|m| m.as_str()) {
            Some(val) => Some(String::from(val)),
            None => None,
        };
        Some(Links {
            line_number,
            link: String::from(val.get(1).map(|m| m.as_str()).unwrap()),
            name: String::from(val.get(2).map(|m| m.as_str()).unwrap()),
            read_till: String::from(val.get(4).map(|m| m.as_str()).unwrap()),
            likeability,
        })
    } else {
        None
    }
}

fn is_heading(content: &String, line_number: usize) -> Option<Heading> {
    lazy_static! {
        static ref RE_HEADING: Regex = Regex::new(r"^(\*+) *(\w.*)").unwrap();
    }
    if let Some(val) = RE_HEADING.captures(content.as_str()) {
        Some(Heading {
            line_number,
            title: String::from(val.get(2).map(|m| m.as_str()).unwrap()),
            level: val.get(1).map(|m| m.as_str()).unwrap().len(),
            heading: vec![],
            links: vec![],
        })
    } else {
        None
    }
}

pub fn parse_org_file(path: PathBuf) -> FileData {
    let mut data = FileData {
        tab_title: format!("{}", path.file_name().unwrap().to_str().unwrap()).to_string(),
        heading: vec![],
        links: vec![],
    };

    if let Ok(lines) = read_lines(path) {
        let mut dynamic_heading: &mut Vec<Heading> = vec![&mut data.heading];
        for (index, line) in lines.enumerate() {
            let line = line.unwrap();
            if let Some(head) = is_heading(&line, index + 1) {
                println!("head: {:?}", head);
            } else if let Some(link) = is_link(&line, index + 1) {
                println!("link: {:?}", link);
            } else {
                println!("{line}");
            }
        }
    }
    data
}
