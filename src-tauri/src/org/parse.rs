use lazy_static::lazy_static;
use regex::Regex;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct Links {
    pub name: String,
    pub link: String,
    pub read_till: String,
    pub likeability: Option<String>,
    pub line_number: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Heading {
    pub title: String,
    pub level: usize,
    pub line_number: usize,
    pub heading: Vec<Heading>,
    pub links: Vec<Links>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileData {
    pub file_title: String,
    pub level: usize,
    pub heading: Vec<Heading>,
    pub links: Vec<Links>,
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
        file_title: format!("{}", path.file_name().unwrap().to_str().unwrap()).to_string(),
        level: 0,
        heading: vec![],
        links: vec![],
    };

    if let Ok(lines) = read_lines(path) {
        let mut current_level = 0;
        let mut current_heading_vec = &mut data.heading;
        for (index, line) in lines.enumerate() {
            let line = line.unwrap();
            if let Some(head) = is_heading(&line, index + 1) {
                let mut len = (&current_heading_vec).len();
                if current_level != head.level - 1{
                    current_level = 0;
                    current_heading_vec = &mut data.heading;
                    while len != 0 && (&current_heading_vec[len - 1]).level != head.level - 1 {
                        current_level = (&current_heading_vec[len - 1]).level;
                        current_heading_vec = &mut current_heading_vec[len - 1].heading;
                        len = (&current_heading_vec).len();
                    }
                }
                if head.line_number == 4{
                    println!("{:#?}",&head);
                    println!("{:#?}",data);
                    loop{}
                }
                current_heading_vec.push(head);
                len += 1;
                current_level = (&current_heading_vec[len - 1]).level;
                current_heading_vec = &mut current_heading_vec[len - 1].heading;
            } else if let Some(link) = is_link(&line, index + 1) {
                // let len = (&current_heading_vec).len();
                // if len == 0{
                //     data.links.push(link);
                // }else{
                //     current_heading_vec[len-1].links.push(link);
                // }
            } else {
                println!("{line}");
            }
        }
    }
    data
}
