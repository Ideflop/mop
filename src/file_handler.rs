use std::{
    fs,
    io::Read,
};
use regex::Regex;

use crate::languages_mapping::{
    Language,
    LANGUAGES, EXTENSIONS,
};

#[derive(Debug)]
pub struct FileHandler {
    path: String,
}

impl FileHandler {
    pub fn new(path: &str) -> FileHandler {
        FileHandler {
            path: path.to_string(),
        }
    }

    fn is_file(&self) -> bool {
        fs::metadata(&self.path).unwrap().is_file()
    }

    fn is_dir(&self) -> bool {
        fs::metadata(&self.path).unwrap().is_dir()
    }

    fn open_file(&self) -> fs::File {
        fs::File::open(&self.path).unwrap()
    }

    fn is_binary(&self) -> bool {
        let mut file = self.open_file();
        match file.read_to_string(&mut String::new()) {
            Ok(_) => false,
            Err(_) => true,
        }
    }

    pub fn get_language(&self) -> Option<Language>{
        let extension = self.path.split('.').last();
        match extension {
            Some(ext) => {
                for (language, exts) in EXTENSIONS.iter() {
                    if exts.contains(ext) {
                        return LANGUAGES.get(language).cloned();
                    }
                }
                None
            },
            _ => None,
        }
    }
}


#[test]
fn test_get_language_rust() {
    let file_handler = FileHandler::new("main.rs");
    let expected_language = Language {
        name: "Rust",
        single_line_comment: Regex::new(r"//").unwrap(),
        multi_line_comment: Regex::new(r"/\*.*?\*/").unwrap()
    };

    let language = file_handler.get_language();
    assert!(language.is_some());
    let language = language.unwrap();
    assert_eq!(language.name, expected_language.name);
    assert_eq!(language.single_line_comment.as_str(), expected_language.single_line_comment.as_str());
    assert_eq!(language.multi_line_comment.as_str(), expected_language.multi_line_comment.as_str());
}

#[test]
fn test_get_language_cpp() {
    let file_handler = FileHandler::new("main.cc");
    let expected_language = Language {
        name: "C++",
        single_line_comment: Regex::new(r"//").unwrap(),
        multi_line_comment: Regex::new(r"/\*.*?\*/").unwrap()
    };

    let language = file_handler.get_language();
    assert!(language.is_some());
    let language = language.unwrap();
    assert_eq!(language.name, expected_language.name);
    assert_eq!(language.single_line_comment.as_str(), expected_language.single_line_comment.as_str());
    assert_eq!(language.multi_line_comment.as_str(), expected_language.multi_line_comment.as_str());
}

