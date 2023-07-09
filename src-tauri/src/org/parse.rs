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
    pub description: Option<String>,
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
    pub file_name: String,
    pub file_meta_data: Option<FileMetaData>,
    pub level: usize,
    pub heading: Vec<Heading>,
    pub links: Vec<Links>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct FileMetaData {
    pub file_title: Option<String>,
    pub file_description: Option<String>,
    pub file_creation_date: Option<String>,
    pub file_tags: Vec<String>,
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
        static ref RE_LINKS: Regex = Regex::new(r"^(?: *\|)? *\[\[(.*)\]\[(.*)\]\](?: *\|)? *(?:\(([^()]*[^[Rr]ead])\))?(?: *\|)? *(?:\(([^()]*[Rr]ead)\))?(?: *\|)?.*-- ?after ?([\w\.]*) *(?: *\|)?").unwrap();
    }
    if let Some(val) = RE_LINKS.captures(content.as_str()) {
        let description = match val.get(3).map(|m| m.as_str()) {
            Some(val) => Some(String::from(val)),
            None => None,
        };
        let likeability = match val.get(4).map(|m| m.as_str()) {
            Some(val) => Some(String::from(val)),
            None => None,
        };
        Some(Links {
            line_number,
            link: String::from(val.get(1).map(|m| m.as_str()).unwrap()),
            name: String::from(val.get(2).map(|m| m.as_str()).unwrap()),
            read_till: String::from(val.get(5).map(|m| m.as_str()).unwrap()),
            description,
            likeability,
        })
    } else {
        None
    }
}

fn is_metadata(content: &String, file_meta_data: &mut FileMetaData) -> bool {
    lazy_static! {
        static ref RE_TITLE: Regex = Regex::new(r"#\+(?i)title: *(.*)").unwrap();
        static ref RE_DATE: Regex = Regex::new(r"#\+(?i)date: *\[(.*)\]").unwrap();
        static ref RE_TAGS: Regex = Regex::new(r"#\+(?i)filetags: *:(.*):*").unwrap();
        static ref RE_DESCRIPTION: Regex = Regex::new(r"#\+(?i)description: *(.*)").unwrap();
    }
                              
    if let Some(val) = RE_TITLE.captures(content.as_str()) {
        file_meta_data.file_title = Some(String::from(val.get(1).map(|m| m.as_str()).unwrap()));
    } else if let Some(val) = RE_DESCRIPTION.captures(content.as_str()) {
        file_meta_data.file_description =  Some(String::from(val.get(1).map(|m| m.as_str()).unwrap()));
    } else if let Some(val) = RE_DATE.captures(content.as_str()) {
        file_meta_data.file_creation_date = Some(String::from(val.get(1).map(|m| m.as_str()).unwrap()));
    } else if let Some(val) = RE_TAGS.captures(content.as_str()) {
        let tags_str: Vec<_> = val.get(1).map(|m| m.as_str()).unwrap().split(":").map(|s| s.to_string()).collect();
        file_meta_data.file_tags = tags_str.to_owned();
    } else {
        return false;
    }
    return true;
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
        file_name: format!("{}", path.file_name().unwrap().to_str().unwrap()).to_string(),
        file_meta_data: None,
        level: 0,
        heading: vec![],
        links: vec![],
    };

    let mut file_meta_data = FileMetaData::default();

    if let Ok(lines) = read_lines(path) {
        let mut current_links_vec = &mut data.links;
        let mut current_heading_vec = &mut data.heading;
        let mut current_level = 0;
        let mut len = 0;
        for (index, line) in lines.enumerate() {
            let line = line.unwrap();
            if is_metadata(&line, &mut file_meta_data) {
            } else if let Some(head) = is_heading(&line, index + 1) {
                len = (&current_heading_vec).len();
                if current_level > head.level - 1 {
                    current_level = 0;
                    current_links_vec = &mut data.links;
                    current_heading_vec = &mut data.heading;
                    len = (&current_heading_vec).len();
                    while len != 0 && (&current_heading_vec[len - 1]).level < head.level - 1 {
                        current_level = (&current_heading_vec[len - 1]).level;
                        unsafe {
                            current_links_vec =
                                &mut *(&mut current_heading_vec[len - 1].links as *mut Vec<Links>);
                        }
                        current_heading_vec = &mut current_heading_vec[len - 1].heading;
                        len = (&current_heading_vec).len();
                    }
                }
                current_heading_vec.push(head);
                len += 1;
                current_level = (&current_heading_vec[len - 1]).level;
                unsafe {
                    current_links_vec =
                        &mut *(&mut current_heading_vec[len - 1].links as *mut Vec<Links>);
                }
                current_heading_vec = &mut current_heading_vec[len - 1].heading;
            } else if let Some(link) = is_link(&line, index + 1) {
                current_links_vec.push(link);
            } else {
                // println!("{line}");
            }
        }
    }
    // println!("{:#?}",&data);
    data.file_meta_data = Some(file_meta_data);
    data
}
