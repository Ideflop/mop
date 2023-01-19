use std::collections::{
    HashMap,
    HashSet,
};
use regex::Regex;
use once_cell::sync::Lazy;

#[derive(Clone)]
pub struct Language<'a> {
    name: &'a str,
    single_line_comment: Regex, // TODO: if faut check que le // est le premier caractère de la ligne (il peut y avoir des espaces avant)
    multi_line_comment_begin: Option<Regex>, // optional,  le meme que single_line_comment
    multi_line_comment_inbetween: Option<Regex>, // optional
    multi_line_comment_end: Option<Regex>, // optional
}

impl<'a> Language<'a> {
    pub fn get_name(&self) -> &'a str {
        self.name
    }

    pub fn get_single_line_comment(&self) -> &Regex {
        &self.single_line_comment
    }

    pub fn get_multi_line_comment_begin(&self) -> Option<&Regex> {
        self.multi_line_comment_begin.as_ref()
    }

    pub fn get_multi_line_comment_inbetween(&self) -> Option<&Regex> {
        self.multi_line_comment_inbetween.as_ref()
    }

    pub fn get_multi_line_comment_end(&self) -> Option<&Regex> {
        self.multi_line_comment_end.as_ref()
    }
}

pub const LANGUAGES: Lazy<HashMap<&str, Language>> = Lazy::new(|| {
    let mut languages = HashMap::new();

    languages.insert("Rust", Language {
        name: "Rust",
        single_line_comment: Regex::new(r"\s*//").unwrap(),
        multi_line_comment_begin: Some(Regex::new(r"/\*").unwrap()),
        multi_line_comment_inbetween: None, 
        multi_line_comment_end: Some(Regex::new(r"\*/").unwrap()),

    });
    //languages.insert("C++", Language {
    //    name: "C++",
    //    single_line_comment: Regex::new(r"^//").unwrap(),
    //    multi_line_comment: Regex::new(r"/\*.*?\*/").unwrap()
    //});

    languages
});

pub const EXTENSIONS: Lazy<HashMap<&str, HashSet<&str>>> = Lazy::new(|| {
    let mut extensions = HashMap::new();

    extensions.insert("Rust", HashSet::from_iter(vec!["rs"]));
    extensions.insert("C++", HashSet::from_iter(vec!["cpp", "cc", "C"]));

    extensions
});
