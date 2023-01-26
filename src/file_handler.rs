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
static EMPTY_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"").unwrap());

pub struct FileHandler {
    path: String,
}

impl FileHandler {
    pub fn new(path: &str) -> FileHandler {
        FileHandler {
            path: path.to_string(),
        }
    }

    pub fn is_file(&self) -> bool {
        fs::metadata(&self.path).unwrap().is_file()
    }

    fn open_file(&self) -> fs::File {
        fs::File::open(&self.path).unwrap()
    }

    pub fn is_binary(&self) -> bool {
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
        single_line_comment.is_match(line)
    }

    fn is_line_block_comment_start(&self, line: &str, start_comment_on_block_line: &Regex) -> bool {
        start_comment_on_block_line.is_match(line)
    }

    fn is_line_block_comment_end(&self, line: &str, end_comment_on_block_line: &Regex) -> bool {
        end_comment_on_block_line.is_match(line)
    }

    fn read_file(&self) -> String {
        let mut file = self.open_file();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        content
    }

    fn is_file_unknow<'a>(&self, file_stat: &mut FileStats<'a>) -> FileStats<'a> {
        println!("File {} is not supported", self.path);
        let file = self.read_file();
        file_stat.add_size(file.len());
        let mut lines = file.lines();
        while let Some(line) = lines.next() {
            if self.is_line_blank(line) {
                file_stat.add_blank_lines();
            } else {
                file_stat.add_code_lines();
            }
            file_stat.add_line();
        };
        file_stat.to_owned()
    }

    fn is_file_known<'a>(&self, language: Language , file_stat: &mut FileStats<'a>) -> FileStats<'a> {
        let file = self.read_file();
        file_stat.add_size(file.len());
        let mut lines = file.lines();

        let mut is_in_block_comment = false;
        let mut block_line_comment_begin_exist = false;
        let mut block_line_comment_end_exist = false;

        let regex_single_line_comment = language.get_single_line_comment();
        let regex_begin_comment_on_block_line = match language.get_block_line_comment_begin() {
            Some(regex) => {
                block_line_comment_begin_exist = true;
                regex
            },
            None => &EMPTY_REGEX,
        };

        let regex_end_comment_on_block_line = match language.get_block_line_comment_end() {
            Some(regex) => {
                block_line_comment_end_exist = true;
                regex
            },
            None => &EMPTY_REGEX,
        };

        while let Some(line) = lines.next() {
            if !is_in_block_comment {
                if self.is_line_blank(&line) {
                    file_stat.add_blank_lines();
                } else if self.is_line_single_comment(&line, &regex_single_line_comment) {
                    file_stat.add_comment_lines();
                } else if block_line_comment_begin_exist && self.is_line_block_comment_start(&line, &regex_begin_comment_on_block_line) {
                    file_stat.add_comment_lines();
                    is_in_block_comment = true;
                } else {
                    file_stat.add_code_lines();
                }
            } else {
                if block_line_comment_end_exist && self.is_line_block_comment_end(&line, &regex_end_comment_on_block_line) {
                    is_in_block_comment = false;
                } 
                file_stat.add_comment_lines();
            }
            file_stat.add_line();
        };
        file_stat.to_owned()
    }

    pub fn get_file_stat<'a>(&self) -> FileStats<'a> {
        let mut file_stat = FileStats::new();
        let file_stat = match self.get_language(&mut file_stat) {
            Some(l) => self.is_file_known(l, &mut file_stat),
            None => self.is_file_unknow(&mut file_stat),
        };
        file_stat
    }

    fn get_language(&self, file_stat: &mut FileStats) -> Option<Language> {
        let extension = self.path.split('.').last();
        match extension {
            Some(ext) => {
                for (language, exts) in EXTENSIONS.iter() {
                    if exts.contains(ext) {
                        file_stat.add_language(language);
                        return LANGUAGES.get(language).cloned();
                    }
                }
                None
            },
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FileStats<'a> {
    language: &'a str,
    size: usize,
    lines : usize,
    blank_lines: usize,
    comment_lines: usize,
    code_lines: usize,
}

impl<'a> FileStats<'a> {
    pub fn new() -> FileStats<'a> {
        FileStats {
            language: "Unknown",
            size: 0,
            lines: 0,
            blank_lines: 0,
            comment_lines: 0,
            code_lines: 0,
        }
    }

    pub fn add_language(&mut self, language: &'a str) {
        self.language = language;
    }
    
    pub fn add_size(&mut self, size: usize) {
        self.size += size;
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

    pub fn get_language(&self) -> &'a str {
        self.language
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_lines(&self) -> usize {
        self.lines
    }

    pub fn get_blank_lines(&self) -> usize {
        self.blank_lines
    }

    pub fn get_comment_lines(&self) -> usize {
        self.comment_lines
    }

    pub fn get_code_lines(&self) -> usize {
        self.code_lines
    }
}
