use std::fmt;
use std::sync::Mutex;

use crate::file_handler::FileHandler;
use crate::languages_mapping::EXTENSIONS_TO_IGNORE;
use crate::search_print::print_and_choose;

use rayon::prelude::*;

pub struct Search<'a> {
    files: Vec<String>,
    pattern: &'a str,
    search_result: Vec<SearchResult>,
}

impl<'a> Search<'a> {
    pub fn new(files: Vec<String>, pattern: &str) -> Search {
        Search { 
            files,
            pattern,
            search_result: Vec::new(),
        }
    }

    pub fn give_and_output_search(&mut self) {
        if self.pattern == "TODO" {
            self.search_todo();
        } else {
            self.search_pattern();
        }
        print_and_choose(&self.search_result);
    }

    fn search_todo(&mut self) {
        let search_result_vec = Mutex::new(Vec::new());
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
                                let mut search_result = SearchResult::new(file.to_string());
                                for i in 0..result.len() {
                                    let just_todo = result[i].1.split("TODO").last().unwrap().trim().to_string();
                                    let new_result = (result[i].0, just_todo);
                                    search_result.add_lines(new_result);
                                }
                                search_result_vec.lock().unwrap().push(search_result);
                            },
                        }
                    }
                    None => {
                        //println!("No language found for file: {}", file);
                    }
                }
            }
        });
        for item in search_result_vec.lock().unwrap().iter() {
            self.search_result.push(item.to_owned());
        }
    }

    fn search_pattern(&mut self) {
        let search_result_vec = Mutex::new(Vec::new());
        self.files.par_iter().for_each(|file| {
            let file_handler = FileHandler::new(file);
            if !file_handler.is_binary() {
                let result =file_handler.search_pattern(self.pattern);
                match result.is_empty() {
                    true  => (),
                    false  => {
                        let mut search_result = SearchResult::new(file.to_string());
                        for i in 0..result.len() {
                            search_result.add_lines(result[i].to_owned())  
                        }
                        search_result_vec.lock().unwrap().push(search_result);
                    },
                }
            } 
        });
        for item in search_result_vec.lock().unwrap().iter() {
            self.search_result.push(item.to_owned());
        }
    }
}

#[derive(Clone)]
pub struct SearchResult {
    pub file_name: String,
    pub lines: Vec<(u32, String)>, // for search_print
}

impl<'a> SearchResult {
    pub fn new(file_name: String) -> SearchResult {
        SearchResult {
            file_name,
            lines: Vec::new(),
        }
    }
    
    fn add_lines(&mut self, value: (u32, String) ) {
        self.lines.push(value);
    }
}

impl fmt::Display for SearchResult  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = format!("{}\n", self.file_name);       
        for (index, item)  in self.lines.iter().enumerate() {
            s += format!("    {}) [{}] : {}\n", index + 1, item.0, item.1).as_str()
        }
        write!(f, "{}", s)
    }
}
