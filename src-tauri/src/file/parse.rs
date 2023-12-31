use lazy_static::lazy_static;
use regex::Regex;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct RegexFileCollections {
    links: Regex,
    heading: Regex,
    title: Regex,
    date: Regex,
    tags: Regex,
    description: Regex,
}

lazy_static! {
    static ref RE_ORG: RegexFileCollections = RegexFileCollections {
                      links: Regex::new(r"^(?: *\|)? *\[\[(.*)\]\[(.*)\]\](?: *\|)? *(?:\(([^()]*[^[Rr]ead])\))?(?: *\|)? *(?:\(([^()]*[Rr]ead)\))?(?: *\|)?.*-- ?after ?([\w\.]*) *(?: *\|)?").unwrap(),
                      heading: Regex::new(r"^(\*+) *(\w.*)").unwrap(),
                      title: Regex::new(r"#\+(?i)title: *(.*)").unwrap(),
                      date: Regex::new(r"#\+(?i)date: *\[(.*)\]").unwrap(),
                      tags: Regex::new(r"#\+(?i)filetags: *:(.*):*").unwrap(),
                      description: Regex::new(r"#\+(?i)description: *(.*)").unwrap(),
    };

    static ref RE_MD: RegexFileCollections = RegexFileCollections {
                      links: Regex::new(r"^(?: *\|)? *\[(.*)\]\((.*?)\)(?: *\|)? *(?:\(([^()]*[^[Rr]ead])\))?(?: *\|)? *(?:\(([^()]*[Rr]ead)\))?(?: *\|)?.*-- ?after ?([\w\.]*) *(?: *\|)?").unwrap(),
                      heading: Regex::new(r"^(#+) *(\w.*)").unwrap(),
                      title: Regex::new(r"(?i)title: *(.*)").unwrap(),
                      date: Regex::new(r"(?i)date: *\[(.*)\]").unwrap(),
                      tags: Regex::new(r"(?i)filetags: *:(.*):*").unwrap(),
                      description: Regex::new(r"(?i)description: *(.*)").unwrap(),
    };
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Link {
    pub name: String,
    pub link: String,
    pub read_till: String,
    pub description: Option<String>,
    pub likeability: Option<String>,
    pub line_number: usize,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Heading {
    pub title: String,
    pub level: usize,
    pub line_number: usize,
    pub heading: Vec<Heading>,
    pub links: Vec<Link>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FileData {
    pub file_name: String,
    pub file_meta_data: FileMetaData,
    pub level: usize,
    pub heading: Vec<Heading>,
    pub links: Vec<Link>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
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

#[derive(Debug, PartialEq, Deserialize, Serialize)]
enum FileType {
    Org,
    Md,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
enum LineType {
    Link(Link),
    Heading(Heading),
    MetaData(MetaData),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
enum MetaData {
    Title(String),
    Description(String),
    Date(String),
    Tags(Vec<String>),
}

fn classify_line(
    line: &String,
    line_number: usize,
    regex_collection: &RegexFileCollections,
    file_type: &FileType,
) -> Option<LineType> {
    if let Some(val) = regex_collection.links.captures(line.as_str()) {
        let description = match val.get(3).map(|m| m.as_str()) {
            Some(val) => Some(String::from(val)),
            None => None,
        };
        let likeability = match val.get(4).map(|m| m.as_str()) {
            Some(val) => Some(String::from(val)),
            None => None,
        };
        let (link_position, name_position) = match file_type {
            FileType::Org => (1, 2),
            FileType::Md => (2, 1),
        };

        Some(LineType::Link(Link {
            line_number,
            link: String::from(val.get(link_position).map(|m| m.as_str()).unwrap()),
            name: String::from(val.get(name_position).map(|m| m.as_str()).unwrap()),
            read_till: String::from(val.get(5).map(|m| m.as_str()).unwrap()),
            description,
            likeability,
        }))
    } else if let Some(val) = regex_collection.heading.captures(line.as_str()) {
        Some(LineType::Heading(Heading {
            line_number,
            title: String::from(val.get(2).map(|m| m.as_str()).unwrap()),
            level: val.get(1).map(|m| m.as_str()).unwrap().len(),
            heading: vec![],
            links: vec![],
        }))
    } else if let Some(val) = regex_collection.title.captures(line.as_str()) {
        Some(LineType::MetaData(MetaData::Title(String::from(
            val.get(1).map(|m| m.as_str()).unwrap(),
        ))))
    } else if let Some(val) = regex_collection.description.captures(line.as_str()) {
        Some(LineType::MetaData(MetaData::Description(String::from(
            val.get(1).map(|m| m.as_str()).unwrap(),
        ))))
    } else if let Some(val) = regex_collection.date.captures(line.as_str()) {
        Some(LineType::MetaData(MetaData::Date(String::from(
            val.get(1).map(|m| m.as_str()).unwrap(),
        ))))
    } else if let Some(val) = regex_collection.tags.captures(line.as_str()) {
        let tags_str: Vec<_> = val
            .get(1)
            .map(|m| m.as_str())
            .unwrap()
            .split(":")
            .filter(|x| *x != "")
            .map(|s| s.to_string())
            .collect();
        Some(LineType::MetaData(MetaData::Tags(tags_str)))
    } else {
        None
    }
}

fn parse_data_file(
    data: &mut FileData,
    file_meta_data: &mut FileMetaData,
    lines: Lines<BufReader<File>>,
    regex_collection: &RegexFileCollections,
    file_type: &FileType,
) {
    let mut _current_links_vec = &mut data.links;
    let mut _current_heading_vec = &mut data.heading;
    let mut _current_level = 0;
    let mut _len = 0;

    for (line_number, line) in lines.enumerate() {
        let line = line.unwrap();
        match classify_line(&line, line_number + 1, &regex_collection, file_type) {
            Some(LineType::Link(link)) => _current_links_vec.push(link),
            Some(LineType::Heading(heading)) => {
                _len = (&_current_heading_vec).len();
                if _current_level > heading.level - 1 {
                    _current_level = 0;
                    _current_links_vec = &mut data.links;
                    _current_heading_vec = &mut data.heading;
                    _len = (&_current_heading_vec).len();
                    while _len != 0 && (&_current_heading_vec[_len - 1]).level < heading.level - 1 {
                        _current_level = (&_current_heading_vec[_len - 1]).level;
                        unsafe {
                            _current_links_vec =
                                &mut *(&mut _current_heading_vec[_len - 1].links as *mut Vec<Link>);
                        }
                        _current_heading_vec = &mut _current_heading_vec[_len - 1].heading;
                        _len = (&_current_heading_vec).len();
                    }
                }
                _current_heading_vec.push(heading);
                _len += 1;
                _current_level = (&_current_heading_vec[_len - 1]).level;
                unsafe {
                    _current_links_vec =
                        &mut *(&mut _current_heading_vec[_len - 1].links as *mut Vec<Link>);
                }
                _current_heading_vec = &mut _current_heading_vec[_len - 1].heading;
            }
            Some(LineType::MetaData(metadata)) => match metadata {
                MetaData::Title(title) => file_meta_data.file_title = title,
                MetaData::Description(description) => file_meta_data.file_description = description,
                MetaData::Date(date) => file_meta_data.file_date = date,
                MetaData::Tags(tags) => file_meta_data.file_tags = tags,
            },
            None => println!("Line can't be classified : {}", line.clone()),
        }
    }
}

pub fn read_data_file(path: PathBuf) -> FileData {
    let mut data = FileData {
        file_name: format!("{}", path.file_name().unwrap().to_str().unwrap()).to_string(),
        file_meta_data: FileMetaData::default(),
        level: 0,
        heading: vec![],
        links: vec![],
    };

    let (file_type, regex_collection) = match path.extension().unwrap().to_str().unwrap() {
        "org" => (FileType::Org, &*RE_ORG),
        "md" => (FileType::Md, &*RE_MD),
        val => panic!("Not a supported file format {val}"),
    };

    unsafe {
        let file_meta_data = &mut *(&mut data.file_meta_data as *mut FileMetaData);
        if let Ok(lines) = read_lines(path) {
            parse_data_file(
                &mut data,
                file_meta_data,
                lines,
                regex_collection,
                &file_type,
            );
        }
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifying_org_test_heading() {
        let regex_collection = &*RE_ORG;
        assert_eq!(
            classify_line(
                &"** Testing".to_string(),
                1,
                regex_collection,
                &FileType::Org
            ),
            Some(LineType::Heading(Heading {
                title: "Testing".to_string(),
                level: 2,
                line_number: 1,
                heading: vec![],
                links: vec![],
            }))
        );
    }

    #[test]
    fn classifying_org_test_link() {
        let regex_collection = &*RE_ORG;
        assert_eq!(
            classify_line(
                &"[[Link to test book][Table test book]] (Mediocure read)             -- after 8"
                    .to_string(),
                1,
                regex_collection,
                &FileType::Org
            ),
            Some(LineType::Link(Link {
                name: "Table test book".to_string(),
                link: "Link to test book".to_string(),
                read_till: "8".to_string(),
                likeability: Some("Mediocure read".to_string()),
                description: None,
                line_number: 1,
            }))
        );
    }

    #[test]
    fn classifying_org_test_link_table() {
        let regex_collection = &*RE_ORG;
        assert_eq!(classify_line(&"| [[Link to test book][Table test book]] | | (Mediocure read)|             -- after 8 |".to_string(),1,regex_collection,&FileType::Org),Some(LineType::Link( Link{ 
            name: "Table test book".to_string(),
            link: "Link to test book".to_string(),
            read_till: "8".to_string(),
            likeability: Some("Mediocure read".to_string()),
            description: None,
            line_number: 1,
        })));
    }

    #[test]
    fn classifying_org_test_metadata_title() {
        let regex_collection = &*RE_ORG;
        assert_eq!(
            classify_line(
                &"#+title: testing title".to_string(),
                1,
                regex_collection,
                &FileType::Org
            ),
            Some(LineType::MetaData(MetaData::Title(
                "testing title".to_string()
            )))
        );
    }

    #[test]
    fn classifying_org_test_metadata_description() {
        let regex_collection = &*RE_ORG;
        assert_eq!(
            classify_line(
                &"#+description: testing description".to_string(),
                1,
                regex_collection,
                &FileType::Org
            ),
            Some(LineType::MetaData(MetaData::Description(
                "testing description".to_string()
            )))
        );
    }

    #[test]
    fn classifying_org_test_metadata_date() {
        let regex_collection = &*RE_ORG;
        assert_eq!(
            classify_line(
                &"#+date: [2023-07-10 Mon 17:00]".to_string(),
                1,
                regex_collection,
                &FileType::Org
            ),
            Some(LineType::MetaData(MetaData::Date(
                "2023-07-10 Mon 17:00".to_string()
            )))
        );
    }

    #[test]
    fn classifying_org_test_metadata_tags() {
        let regex_collection = &*RE_ORG;
        assert_eq!(
            classify_line(
                &"#+filetags: :tag1:tag2:tag3:".to_string(),
                1,
                regex_collection,
                &FileType::Org
            ),
            Some(LineType::MetaData(MetaData::Tags(vec![
                "tag1".to_string(),
                "tag2".to_string(),
                "tag3".to_string()
            ])))
        );
    }

    #[test]
    fn test_parsing_org() {
        let parsed_input =
            read_data_file(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../examples/test.org"));

        let expected_output = FileData {
            file_name: "test.org".to_string(),
            file_meta_data: FileMetaData {
                file_title: "test title".to_string(),
                file_description: "Test description to check parsing".to_string(),
                file_date: "2023-07-10 Mon 17:00".to_string(),
                file_tags: vec![
                    "testing".to_string(),
                    "rust".to_string(),
                    "orgmode".to_string(),
                    "linkorg".to_string(),
                ],
            },
            level: 0,
            heading: vec![
                Heading {
                    title: "Level 1 heading".to_string(),
                    level: 1,
                    line_number: 6,
                    heading: vec![Heading {
                        title: "Level 2 heading 1 under 1".to_string(),
                        level: 2,
                        line_number: 7,
                        heading: vec![],
                        links: vec![
                            Link {
                                name: "Table test book 1".to_string(),
                                link: "Link to test book 1".to_string(),
                                read_till: "20".to_string(),
                                description: Some("Good book".to_string()),
                                likeability: Some("Must read".to_string()),
                                line_number: 8,
                            },
                            Link {
                                name: "Table test book 2".to_string(),
                                link: "Link to test book 2".to_string(),
                                read_till: "2".to_string(),
                                description: None,
                                likeability: Some("Good read".to_string()),
                                line_number: 9,
                            },
                        ],
                    }],
                    links: vec![],
                },
                Heading {
                    title: "Level 2 heading 2 under 1".to_string(),
                    level: 2,
                    line_number: 10,
                    heading: vec![],
                    links: vec![
                        Link {
                            name: "Table test book 3".to_string(),
                            link: "Link to test book 3".to_string(),
                            read_till: "8".to_string(),
                            description: Some("Mediocure book".to_string()),
                            likeability: None,
                            line_number: 11,
                        },
                        Link {
                            name: "Table test book 4".to_string(),
                            link: "Link to test book 4".to_string(),
                            read_till: "9".to_string(),
                            description: None,
                            likeability: None,
                            line_number: 12,
                        },
                    ],
                },
                Heading {
                    title: "Level 1 Table heading".to_string(),
                    level: 1,
                    line_number: 13,
                    heading: vec![],
                    links: vec![
                        Link {
                            name: "Table test book 1".to_string(),
                            link: "Link to test book 1".to_string(),
                            read_till: "20".to_string(),
                            description: Some("Good book".to_string()),
                            likeability: Some("Must read".to_string()),
                            line_number: 15,
                        },
                        Link {
                            name: "Table test book 2".to_string(),
                            link: "Link to test book 2".to_string(),
                            read_till: "2".to_string(),
                            description: None,
                            likeability: Some("Good read".to_string()),
                            line_number: 16,
                        },
                        Link {
                            name: "Table test book 3".to_string(),
                            link: "Link to test book 3".to_string(),
                            read_till: "8".to_string(),
                            description: Some("Mediocure book".to_string()),
                            likeability: None,
                            line_number: 17,
                        },
                        Link {
                            name: "Table test book 4".to_string(),
                            link: "Link to test book 4".to_string(),
                            read_till: "9".to_string(),
                            description: None,
                            likeability: None,
                            line_number: 18,
                        },
                    ],
                },
            ],
            links: vec![],
        };

        assert_eq!(parsed_input, expected_output);
    }

    #[test]
    fn classifying_md_test_heading() {
        let regex_collection = &*RE_MD;
        assert_eq!(
            classify_line(
                &"## Testing".to_string(),
                1,
                regex_collection,
                &FileType::Md
            ),
            Some(LineType::Heading(Heading {
                title: "Testing".to_string(),
                level: 2,
                line_number: 1,
                heading: vec![],
                links: vec![],
            }))
        );
    }

    #[test]
    fn classifying_md_test_link() {
        let regex_collection = &*RE_MD;
        assert_eq!(
            classify_line(
                &"[Table test book](Link to test book) (Mediocure read)             -- after 8"
                    .to_string(),
                1,
                regex_collection,
                &FileType::Md
            ),
            Some(LineType::Link(Link {
                name: "Table test book".to_string(),
                link: "Link to test book".to_string(),
                read_till: "8".to_string(),
                likeability: Some("Mediocure read".to_string()),
                description: None,
                line_number: 1,
            }))
        );
    }

    #[test]
    fn classifying_md_test_link_table() {
        let regex_collection = &*RE_MD;
        assert_eq!(classify_line(&"| [Table test book](Link to test book) | | (Mediocure read)|             -- after 8 |".to_string(),1,regex_collection,&FileType::Md),Some(LineType::Link( Link{ 
            name: "Table test book".to_string(),
            link: "Link to test book".to_string(),
            read_till: "8".to_string(),
            likeability: Some("Mediocure read".to_string()),
            description: None,
            line_number: 1,
        })));
    }

    #[test]
    fn classifying_md_test_metadata_title() {
        let regex_collection = &*RE_MD;
        assert_eq!(
            classify_line(
                &"title: testing title".to_string(),
                1,
                regex_collection,
                &FileType::Md
            ),
            Some(LineType::MetaData(MetaData::Title(
                "testing title".to_string()
            )))
        );
    }

    #[test]
    fn classifying_md_test_metadata_description() {
        let regex_collection = &*RE_MD;
        assert_eq!(
            classify_line(
                &"description: testing description".to_string(),
                1,
                regex_collection,
                &FileType::Md
            ),
            Some(LineType::MetaData(MetaData::Description(
                "testing description".to_string()
            )))
        );
    }

    #[test]
    fn classifying_md_test_metadata_date() {
        let regex_collection = &*RE_MD;
        assert_eq!(
            classify_line(
                &"date: [2023-07-10 Mon 17:00]".to_string(),
                1,
                regex_collection,
                &FileType::Md
            ),
            Some(LineType::MetaData(MetaData::Date(
                "2023-07-10 Mon 17:00".to_string()
            )))
        );
    }

    #[test]
    fn classifying_md_test_metadata_tags() {
        let regex_collection = &*RE_MD;
        assert_eq!(
            classify_line(
                &"filetags: :tag1:tag2:tag3:".to_string(),
                1,
                regex_collection,
                &FileType::Md
            ),
            Some(LineType::MetaData(MetaData::Tags(vec![
                "tag1".to_string(),
                "tag2".to_string(),
                "tag3".to_string()
            ])))
        );
    }

    #[test]
    fn test_parsing_md() {
        let parsed_input =
            read_data_file(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../examples/test.md"));

        let expected_output = FileData {
            file_name: "test.md".to_string(),
            file_meta_data: FileMetaData {
                file_title: "test title".to_string(),
                file_description: "Test description to check parsing".to_string(),
                file_date: "2023-07-10 Mon 17:00".to_string(),
                file_tags: vec![
                    "testing".to_string(),
                    "rust".to_string(),
                    "orgmode".to_string(),
                    "linkorg".to_string(),
                ],
            },
            level: 0,
            heading: vec![
                Heading {
                    title: "Level 1 heading".to_string(),
                    level: 1,
                    line_number: 6,
                    heading: vec![Heading {
                        title: "Level 2 heading 1 under 1".to_string(),
                        level: 2,
                        line_number: 7,
                        heading: vec![],
                        links: vec![
                            Link {
                                name: "Table test book 1".to_string(),
                                link: "Link to test book 1".to_string(),
                                read_till: "20".to_string(),
                                description: Some("Good book".to_string()),
                                likeability: Some("Must read".to_string()),
                                line_number: 8,
                            },
                            Link {
                                name: "Table test book 2".to_string(),
                                link: "Link to test book 2".to_string(),
                                read_till: "2".to_string(),
                                description: None,
                                likeability: Some("Good read".to_string()),
                                line_number: 9,
                            },
                        ],
                    }],
                    links: vec![],
                },
                Heading {
                    title: "Level 2 heading 2 under 1".to_string(),
                    level: 2,
                    line_number: 10,
                    heading: vec![],
                    links: vec![
                        Link {
                            name: "Table test book 3".to_string(),
                            link: "Link to test book 3".to_string(),
                            read_till: "8".to_string(),
                            description: Some("Mediocure book".to_string()),
                            likeability: None,
                            line_number: 11,
                        },
                        Link {
                            name: "Table test book 4".to_string(),
                            link: "Link to test book 4".to_string(),
                            read_till: "9".to_string(),
                            description: None,
                            likeability: None,
                            line_number: 12,
                        },
                    ],
                },
                Heading {
                    title: "Level 1 Table heading".to_string(),
                    level: 1,
                    line_number: 13,
                    heading: vec![],
                    links: vec![
                        Link {
                            name: "Table test book 1".to_string(),
                            link: "Link to test book 1".to_string(),
                            read_till: "20".to_string(),
                            description: Some("Good book".to_string()),
                            likeability: Some("Must read".to_string()),
                            line_number: 15,
                        },
                        Link {
                            name: "Table test book 2".to_string(),
                            link: "Link to test book 2".to_string(),
                            read_till: "2".to_string(),
                            description: None,
                            likeability: Some("Good read".to_string()),
                            line_number: 16,
                        },
                        Link {
                            name: "Table test book 3".to_string(),
                            link: "Link to test book 3".to_string(),
                            read_till: "8".to_string(),
                            description: Some("Mediocure book".to_string()),
                            likeability: None,
                            line_number: 17,
                        },
                        Link {
                            name: "Table test book 4".to_string(),
                            link: "Link to test book 4".to_string(),
                            read_till: "9".to_string(),
                            description: None,
                            likeability: None,
                            line_number: 18,
                        },
                    ],
                },
            ],
            links: vec![],
        };

        assert_eq!(parsed_input, expected_output);
    }
}
