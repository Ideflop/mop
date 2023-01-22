use std::{
    fs,
    fmt,
    path::Path,
    process::exit,
};

use crate::{
    languages_mapping::EXTENSIONS_TO_IGNORE,
    file_handler::{FileHandler, FileStats},
};

pub struct ExtractInfo {
    number_of_files: usize,
    number_of_files_ignore: usize,
    number_of_directories: usize,
    total_size: usize,
    tot_lines : usize,
    tot_blank_lines: usize,
    tot_comment_lines: usize,
    tot_code_lines: usize,
    stats_per_language: Vec<StatPerLanguage>,
}

impl ExtractInfo {
    pub fn new() -> ExtractInfo {
        ExtractInfo {
            number_of_files: 0,
            number_of_files_ignore: 0,
            number_of_directories: 0,
            total_size: 0,
            tot_lines : 0,
            tot_blank_lines: 0,
            tot_comment_lines: 0,
            tot_code_lines: 0,
            stats_per_language: Vec::new(),
        }
    }

    pub fn add_number_of_files(&mut self) {
        self.number_of_files += 1;
    }

    pub fn add_number_of_files_ignore(&mut self) {
        self.number_of_files_ignore += 1;
    }

    pub fn add_number_of_directories(&mut self) {
        self.number_of_directories += 1;
    }

    pub fn add_total_size(&mut self, size: usize) {
        self.total_size += size;
    }

    pub fn add_tot_lines(&mut self, lines: usize) {
        self.tot_lines += lines;
    }

    pub fn add_tot_blank_lines(&mut self, lines: usize) {
        self.tot_blank_lines += lines;
    }

    pub fn add_tot_comment_lines(&mut self, lines: usize) {
        self.tot_comment_lines += lines;
    }

    pub fn add_tot_code_lines(&mut self, lines: usize) {
        self.tot_code_lines += lines;
    }

    pub fn add_stat_for_each_language(&mut self, stats: FileStats) {
        let mut found = false;
        for stat in self.stats_per_language.iter_mut() {
            if stat.language == stats.get_language() {
                stat.add_number_of_files();
                stat.add_total_size(stats.get_size());
                stat.add_tot_lines(stats.get_lines());
                stat.add_tot_blank_lines(stats.get_blank_lines());
                stat.add_tot_comment_lines(stats.get_comment_lines());
                stat.add_tot_code_lines(stats.get_code_lines());
                found = true;
                break;
            }
        }
        if !found {
            self.stats_per_language.push(StatPerLanguage::new(
          stats.get_language().to_string(),
                    stats.get_size(),
                    stats.get_lines(),
                    stats.get_blank_lines(),
                    stats.get_comment_lines(),
         stats.get_code_lines(),
            ));
        }

    }

    pub fn get_argument<'a>(&self, arg: &str) -> Result<FileStats<'a>, String> {
        let file = FileHandler::new(&arg);
        if file.is_file() && !file.is_binary() && !EXTENSIONS_TO_IGNORE.contains(&arg.split('.').last().unwrap()) {
            Ok(file.get_file_stat())
        } else {
            Err(format!("{} is not a file or is binary", arg))
        }
    }

}

struct StatPerLanguage {
    language: String,
    number_of_files: usize,
    total_size: usize,
    tot_lines : usize,
    tot_blank_lines: usize,
    tot_comment_lines: usize,
    tot_code_lines: usize,
}

impl StatPerLanguage {
    fn new(language: String, size: usize, lines: usize, blank_lines: usize, comment_lines: usize, code_line: usize) -> StatPerLanguage {
        StatPerLanguage {
            language,
            number_of_files: 1,
            total_size: size,
            tot_lines : lines,
            tot_blank_lines: blank_lines,
            tot_comment_lines: comment_lines,
            tot_code_lines: code_line,
        }
    }

    fn add_number_of_files(&mut self) {
        self.number_of_files += 1;
    }

    fn add_total_size(&mut self, size: usize) {
        self.total_size += size;
    }

    fn add_tot_lines(&mut self, lines: usize) {
        self.tot_lines += lines;
    }

    fn add_tot_blank_lines(&mut self, lines: usize) {
        self.tot_blank_lines += lines;
    }

    fn add_tot_comment_lines(&mut self, lines: usize) {
        self.tot_comment_lines += lines;
    }

    fn add_tot_code_lines(&mut self, lines: usize) {
        self.tot_code_lines += lines;
    }
}

pub fn get_file(file: Vec<String>) {
    if file.is_empty() {
        println!("No file given");
        exit(1)
    }
    let mut extract_info = ExtractInfo::new();
    for arg in file {
        match extract_info.get_argument(&arg) {
            Ok(file_stat) => {
                extract_info.add_number_of_files();
                extract_info.add_total_size(file_stat.get_size());
                extract_info.add_tot_lines(file_stat.get_lines());
                extract_info.add_tot_blank_lines(file_stat.get_blank_lines());
                extract_info.add_tot_comment_lines(file_stat.get_comment_lines());
                extract_info.add_tot_code_lines(file_stat.get_code_lines());
                extract_info.add_stat_for_each_language(file_stat);
            }
            Err(_) => {
                extract_info.add_number_of_files_ignore();
            }
        }
    };
    println!("{}", extract_info)
}

pub fn get_dir_from_main(dir: Vec<String>) {
    if dir.is_empty() {
        println!("No directory given");
        exit(1)
    }
    let mut file = Vec::new();
    for arg in dir {
        match Path::new(&arg).is_dir() {
            true => {
                let a = get_files_in_path(&arg.as_str());
                file.extend(a);
            }
            false => {
                println!("{} is not a directory", arg);
                exit(1)
            }
        }
    }
    get_file(file);
}

fn get_files_in_path(path: &str) -> Vec<String> {
    let path = Path::new(path);

    let mut file_names = vec![];
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let file_type = entry.file_type().unwrap();
        let file_name = entry.file_name();
        if file_type.is_file() {
            if !file_name.to_str().unwrap().starts_with("."){
                file_names.push(entry.path().into_os_string().into_string().unwrap());
            }
        } else if file_type.is_dir() {
            let subdir_path = entry.path();
            file_names.append(&mut get_files_in_path(subdir_path.to_str().unwrap()));
        }
    }

    file_names
}

impl fmt::Display for ExtractInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, ) -> fmt::Result {
        let mut s = format!("
Number of files ignored: {}
Number of directories: {}
|---------------------------------------------------------------------------------|
| Language      | Files | Size | Blank lines | Comment lines | Code lines | TOTAL |
|---------------------------------------------------------------------------------|
"
, self.number_of_files_ignore, self.number_of_directories);
        for stat in self.stats_per_language.iter() {
            s += &format!("{}", stat).as_str();
        }
        s += format!(
"|---------------------------------------------------------------------------------|
| Total         |   {}  |   {} |        {} |          {} |         {} |    {} |
|---------------------------------------------------------------------------------|
",   self.number_of_files, self.total_size, self.tot_lines, self.tot_blank_lines, self.tot_comment_lines, self.tot_code_lines).as_str();
        write!(f, "{}", s)
    }
}

impl fmt:: Display for StatPerLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!(
"| {}              {}      {} |        {} |            {} |       {} |     {} |
", self.language, self.number_of_files, self.total_size, self.tot_blank_lines, self.tot_comment_lines, self.tot_code_lines, self.tot_lines);
        write!(f, "{}", s)
    }
}
