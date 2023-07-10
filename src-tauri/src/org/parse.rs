use lazy_static::lazy_static;
use regex::Regex;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::{Path, PathBuf};

lazy_static! {
    static ref RE_LINKS: Regex = Regex::new(r"^(?: *\|)? *\[\[(.*)\]\[(.*)\]\](?: *\|)? *(?:\(([^()]*[^[Rr]ead])\))?(?: *\|)? *(?:\(([^()]*[Rr]ead)\))?(?: *\|)?.*-- ?after ?([\w\.]*) *(?: *\|)?").unwrap();
    static ref RE_HEADING: Regex = Regex::new(r"^(\*+) *(\w.*)").unwrap();
    static ref RE_TITLE: Regex = Regex::new(r"#\+(?i)title: *(.*)").unwrap();
    static ref RE_DATE: Regex = Regex::new(r"#\+(?i)date: *\[(.*)\]").unwrap();
    static ref RE_TAGS: Regex = Regex::new(r"#\+(?i)filetags: *:(.*):*").unwrap();
    static ref RE_DESCRIPTION: Regex = Regex::new(r"#\+(?i)description: *(.*)").unwrap();
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Link {
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
    pub links: Vec<Link>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileData {
    pub file_name: String,
    pub file_meta_data: FileMetaData,
    pub level: usize,
    pub heading: Vec<Heading>,
    pub links: Vec<Link>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileMetaData {
    pub file_title: String,
    pub file_description: String,
    pub file_date: String,
    pub file_tags: Vec<String>,
}

impl Default for FileMetaData {
    fn default() -> Self {
        FileMetaData {
            file_title: "No title".to_string(),
            file_description: "No description".to_string(),
            file_date: "No creation Date found".to_string(),
            file_tags: vec![],
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

enum LineType {
    Link(Link),
    Heading(Heading),
    MetaData(MetaData),
}

enum MetaData {
    Title(String),
    Description(String),
    Date(String),
    Tags(Vec<String>),
}

fn classify_line(line: &String, line_number: usize) -> Option<LineType> {
    if let Some(val) = RE_LINKS.captures(line.as_str()) {
        let description = match val.get(3).map(|m| m.as_str()) {
            Some(val) => Some(String::from(val)),
            None => None,
        };
        let likeability = match val.get(4).map(|m| m.as_str()) {
            Some(val) => Some(String::from(val)),
            None => None,
        };
        Some(LineType::Link(Link {
            line_number,
            link: String::from(val.get(1).map(|m| m.as_str()).unwrap()),
            name: String::from(val.get(2).map(|m| m.as_str()).unwrap()),
            read_till: String::from(val.get(5).map(|m| m.as_str()).unwrap()),
            description,
            likeability,
        }))
    } else if let Some(val) = RE_HEADING.captures(line.as_str()) {
        Some(LineType::Heading(Heading {
            line_number,
            title: String::from(val.get(2).map(|m| m.as_str()).unwrap()),
            level: val.get(1).map(|m| m.as_str()).unwrap().len(),
            heading: vec![],
            links: vec![],
        }))
    } else if let Some(val) = RE_TITLE.captures(line.as_str()) {
        Some(LineType::MetaData(MetaData::Title(String::from(
            val.get(1).map(|m| m.as_str()).unwrap(),
        ))))
    } else if let Some(val) = RE_DESCRIPTION.captures(line.as_str()) {
        Some(LineType::MetaData(MetaData::Description(String::from(
            val.get(1).map(|m| m.as_str()).unwrap(),
        ))))
    } else if let Some(val) = RE_DATE.captures(line.as_str()) {
        Some(LineType::MetaData(MetaData::Date(String::from(
            val.get(1).map(|m| m.as_str()).unwrap(),
        ))))
    } else if let Some(val) = RE_TAGS.captures(line.as_str()) {
        let tags_str: Vec<_> = val
            .get(1)
            .map(|m| m.as_str())
            .unwrap()
            .split(":")
            .map(|s| s.to_string())
            .collect();
        Some(LineType::MetaData(MetaData::Tags(tags_str)))
    } else {
        None
    }
}

fn parse_org_file(
    data: &mut FileData,
    file_meta_data: &mut FileMetaData,
    lines: Lines<BufReader<File>>,
) {
    let mut current_links_vec = &mut data.links;
    let mut current_heading_vec = &mut data.heading;
    let mut current_level = 0;
    let mut len = 0;
    for (line_number, line) in lines.enumerate() {
        let line = line.unwrap();
        match classify_line(&line, line_number + 1) {
            Some(LineType::Link(link)) => current_links_vec.push(link),
            Some(LineType::Heading(heading)) => {
                len = (&current_heading_vec).len();
                if current_level > heading.level - 1 {
                    current_level = 0;
                    current_links_vec = &mut data.links;
                    current_heading_vec = &mut data.heading;
                    len = (&current_heading_vec).len();
                    while len != 0 && (&current_heading_vec[len - 1]).level < heading.level - 1 {
                        current_level = (&current_heading_vec[len - 1]).level;
                        unsafe {
                            current_links_vec =
                                &mut *(&mut current_heading_vec[len - 1].links as *mut Vec<Link>);
                        }
                        current_heading_vec = &mut current_heading_vec[len - 1].heading;
                        len = (&current_heading_vec).len();
                    }
                }
                current_heading_vec.push(heading);
                len += 1;
                current_level = (&current_heading_vec[len - 1]).level;
                unsafe {
                    current_links_vec =
                        &mut *(&mut current_heading_vec[len - 1].links as *mut Vec<Link>);
                }
                current_heading_vec = &mut current_heading_vec[len - 1].heading;
            }
            Some(LineType::MetaData(metadata)) => match metadata{
                MetaData::Title(title) => file_meta_data.file_title = title,
                MetaData::Description(description) => file_meta_data.file_description = description,
                MetaData::Date(date) => file_meta_data.file_date = date,
                MetaData::Tags(tags) => file_meta_data.file_tags = tags,
            },
            None => println!("Line can't be classified : {}", line.clone()),
        }
    }
}

pub fn read_org_file(path: PathBuf) -> FileData {
    let mut data = FileData {
        file_name: format!("{}", path.file_name().unwrap().to_str().unwrap()).to_string(),
        file_meta_data: FileMetaData::default(),
        level: 0,
        heading: vec![],
        links: vec![],
    };

    unsafe{
        let file_meta_data = &mut *(&mut data.file_meta_data as *mut FileMetaData);
        if let Ok(lines) = read_lines(path) {
            parse_org_file(&mut data,file_meta_data, lines);
        }
    }
    data
}
