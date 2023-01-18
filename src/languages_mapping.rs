use std::collections::{
    HashMap,
    HashSet,
};
use regex::Regex;
use once_cell::sync::Lazy;


#[derive(Clone)]
pub struct Language<'a> {
    pub name: &'a str,
    pub single_line_comment: Regex,
    pub multi_line_comment: Regex,
}

pub const LANGUAGES: Lazy<HashMap<&str, Language>> = Lazy::new(|| {
    let mut languages = HashMap::new();

    languages.insert("Rust", Language {
        name: "Rust",
        single_line_comment: Regex::new(r"//").unwrap(),
        multi_line_comment: Regex::new(r"/\*.*?\*/").unwrap()
    });
    languages.insert("C++", Language {
        name: "C++",
        single_line_comment: Regex::new(r"//").unwrap(),
        multi_line_comment: Regex::new(r"/\*.*?\*/").unwrap()
    });

    languages
});

pub const EXTENSIONS: Lazy<HashMap<&str, HashSet<&str>>> = Lazy::new(|| {
    let mut extensions = HashMap::new();

    extensions.insert("Rust", HashSet::from_iter(vec!["rs"]));
    extensions.insert("C++", HashSet::from_iter(vec!["cpp", "cc", "C"]));

    extensions
});
