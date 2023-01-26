use std::{
    clone::Clone,
    collections::{
    HashMap,
    HashSet,
}};
use regex::Regex;
use once_cell::sync::Lazy;

#[derive(Clone)]
pub struct Language<'a> {
    name: &'a str,
    single_line_comment: Regex, 
    block_line_comment_begin: Option<Regex>,
    block_line_comment_inbetween: Option<Regex>, // needed ? just need to check if the block comment is closed or not
    block_line_comment_end: Option<Regex>, 
}

impl<'a> Language<'a> {
    pub fn get_name(&self) -> &'a str {
        self.name
    }

    pub fn get_single_line_comment(&self) -> &Regex {
        &self.single_line_comment
    }

    pub fn get_block_line_comment_begin(&self) -> Option<&Regex> {
        self.block_line_comment_begin.as_ref()
    }

    pub fn get_block_line_comment_inbetween(&self) -> Option<&Regex> {
        self.block_line_comment_inbetween.as_ref()
    }

    pub fn get_block_line_comment_end(&self) -> Option<&Regex> {
        self.block_line_comment_end.as_ref()
    }
}

pub const LANGUAGES: Lazy<HashMap<&str, Language>> = Lazy::new(|| {
    let mut languages = HashMap::new();

    languages.insert("Rust", Language {
        name: "Rust",
        single_line_comment: Regex::new(r"^\s*//").unwrap(),
        block_line_comment_begin: Some(Regex::new(r"\s*/\*").unwrap()),
        block_line_comment_inbetween: None, 
        block_line_comment_end: Some(Regex::new(r".*?\*/\s*").unwrap()),

    });
    languages.insert("Java", Language {
        name: "Java",
        single_line_comment: Regex::new(r"^\s*//").unwrap(),
        block_line_comment_begin: Some(Regex::new(r"\s*/\*").unwrap()),
        block_line_comment_inbetween: None, 
        block_line_comment_end: Some(Regex::new(r".*?\*/\s*").unwrap()),

    });

    languages
});

pub const EXTENSIONS: Lazy<HashMap<&str, HashSet<&str>>> = Lazy::new(|| {
    let mut extensions = HashMap::new();

    extensions.insert("C++", HashSet::from_iter(vec!["cpp", "cc", "C"]));
    extensions.insert("Java", HashSet::from_iter(vec!["java"]));
    extensions.insert("Rust", HashSet::from_iter(vec!["rs"]));

    extensions
});

pub const EXTENSIONS_TO_IGNORE: [&str; 64] = ["pdf", "png", "jpg", "jpeg", "gif", "svg", "ico", "bmp", "tiff", "tif", "webp", "psd", "eps", "raw", "cr2", "nef", "orf", "sr2", "arw", "dng", "heic", "heif", "indd", "zip", "rar", "tar", "gz", "7z", "bz2", "dmg", "iso", "mp3", "mp4", "wav", "flac", "aac", "ogg", "wma", "m4a", "avi", "mov", "wmv", "mpg", "mpeg", "3gp", "mkv", "flv", "swf", "f4v", "f4p", "f4a", "f4b", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "odt", "ods", "odp", "bluej", "class", "jar" ];
