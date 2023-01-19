use std::{
    fs,
    io::Read,
};
use regex::Regex;
use once_cell::sync::Lazy;

use crate::languages_mapping::{
    Language,
    LANGUAGES, EXTENSIONS,
};

const IS_BLANK: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s*$").unwrap());

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

    fn is_line_blank(&self, line: &str) -> bool {
        IS_BLANK.is_match(line)
    }

    fn is_line_single_comment(&self, line: &str, single_line_comment: &Regex) -> bool {
        single_line_comment.is_match(line) // or starts_with don't know what is better
    }

    fn is_line_multiple_comment_start(&self, line: &str, start_comment_on_multiple_line: &Regex) -> bool {
        start_comment_on_multiple_line.is_match(line)
    }

    fn is_line_multiple_comment_inbetween(&self, line : &str, inbetween_comment_on_multiple_line: &Regex) -> bool {
        inbetween_comment_on_multiple_line.is_match(line)
    }

    fn is_line_multiple_comment_end(&self, line: &str, end_comment_on_multiple_line: &Regex) -> bool {
        end_comment_on_multiple_line.is_match(line)
    }

    fn read_file(&self) -> String {
        let mut file = self.open_file();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        content
    }

    //fn get_lines(&self) -> Vec<String> {
    //}
    
    // TODO: fn that take file and return stats

    pub fn get_language(&self) -> Option<Language> {
        let extension = self.path.split('.').last();
        match extension {
            Some(ext) => {
                for (language, exts) in EXTENSIONS.iter() {
                    if exts.contains(ext) {
                        // TODO: ajouter FileStats language
                        return LANGUAGES.get(language).cloned();
                    }
                }
                None
            },
            _ => None,
        }
    }
}

pub struct FileStats<'a> {
    language: &'a str,
    lines : usize,
    blank_lines: usize,
    comment_lines: usize,
    code_lines: usize,
}

impl<'a> FileStats<'a> {
    pub fn new() -> FileStats<'a> {
        FileStats {
            language: "Unknown",
            lines: 0,
            blank_lines: 0,
            comment_lines: 0,
            code_lines: 0,
        }
    }

    pub fn add_language(&mut self, language: &'a str) {
        self.language = language;
    }

    pub fn add_line(&mut self) {
        self.lines += 1;
    }

    pub fn add_blank_lines(&mut self) {
        self.blank_lines += 1;
    }

    pub fn add_comment_lines(&mut self) {
        self.comment_lines += 1;
    }

    pub fn add_code_lines(&mut self) {
        self.code_lines += 1;
    }

    // TODO: add getters 
}








#[test] // TODO: handle that fields are private
fn test_get_language_rust() {
    let file_handler = FileHandler::new("main.rs");
    let expected_language = Language {
        name: "Rust",
        single_line_comment: Regex::new(r"^//").unwrap(),
        multi_line_comment_begin: Some(Regex::new(r"/\*").unwrap()),
        multi_line_comment_inbetween: None,
        multi_line_comment_end: Some(Regex::new(r"\*/").unwrap()),
    };

    let language = file_handler.get_language();
    assert!(language.is_some());
    let language = language.unwrap();
    assert_eq!(language.get_name(), expected_language.get_name());
    assert_eq!(language.get_single_line_comment().as_str(), expected_language.get_single_line_comment().as_str());
    assert_eq!(language.get_multi_line_comment_begin().unwrap().as_str(), expected_language.get_multi_line_comment_begin().unwrap().as_str());
    assert_eq!(language.get_multi_line_comment_inbetween().is_none(), expected_language.get_multi_line_comment_inbetween().is_none());
    assert_eq!(language.get_multi_line_comment_end().unwrap().as_str(), expected_language.get_multi_line_comment_end().unwrap().as_str());
}
