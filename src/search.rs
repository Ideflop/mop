use crate::file_handler::FileHandler;
use crate::languages_mapping::EXTENSIONS_TO_IGNORE;

use rayon::prelude::*;

pub struct Search<'a> {
    files: Vec<String>,
    pattern: &'a str,
    search_result: Vec<SearchResult<'a>>,
}

impl<'a> Search<'a> {
    pub fn new(files: Vec<String>, pattern: &str) -> Search {
        Search { 
            files,
            pattern,
            search_result: Vec::new(),
        }
    }

    pub fn give_and_output_search(&self) {
        if self.pattern == "TODO" {
            self.search_todo();
        } else {
            self.search_pattern();
        }
        
    }

    fn search_todo(&self) {
        self.files.par_iter().for_each(|file| {
            let file_handler = FileHandler::new(file);
            if !file_handler.is_binary() && !EXTENSIONS_TO_IGNORE.contains(&file.split(".").last().unwrap()) {
                match file_handler.get_language_for_search() {
                    Some(language) => {
                        let single_comment = language.get_single_line_comment().to_string();
                        let single_comment = single_comment.split("*").last().unwrap();
                        let pattern = single_comment.to_owned() + " TODO";
                        let result =file_handler.search_pattern(pattern.as_str());
                        match result.is_empty() {
                            true  => (),
                            false  => {
                                let mut search_result = SearchResult::new(file);
                                for i in 0..result.len() {
                                    let just_todo = result[i].1.split("TODO").last().unwrap().trim().to_string();
                                    let new_result = (result[i].0, just_todo);
                                    search_result.add_lines(new_result);
                                }
                            },
                        }
                    }
                    None => {
                        //println!("No language found for file: {}", file);
                    }
                }
            }
        });
    }

    fn search_pattern(&self) {
        //for file in &self.files {
        self.files.par_iter().for_each(|file| {
            let file_handler = FileHandler::new(file);
            if !file_handler.is_binary() {
                let result =file_handler.search_pattern(self.pattern);
                match result.is_empty() {
                    true  => (),
                    false  => {
                        let mut search_result = SearchResult::new(file);
                        for i in 0..result.len() {
                            search_result.add_lines(result[i].to_owned())  
                        }
                    },
                }
            } 
        });
    }
}

struct SearchResult<'a> {
    file_name: &'a str,
    lines: Vec<(u32, String)>,
}

impl<'a> SearchResult<'a> {
    pub fn new(file_name: &'a str) -> SearchResult<'a> {
        SearchResult {
            file_name,
            lines: Vec::new(),
        }
    }
    
    fn add_lines(&mut self, value: (u32, String) ) {
        self.lines.push(value)
    }
}
