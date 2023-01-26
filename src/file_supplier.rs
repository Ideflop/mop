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
        if file.is_file() && !file.is_binary() && !EXTENSIONS_TO_IGNORE.contains(&arg.split(".").last().unwrap()) {
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
            if !subdir_path.to_str().unwrap().split('/').last().unwrap().starts_with(".") {
                file_names.append(&mut get_files_in_path(subdir_path.to_str().unwrap()));
            }
        }
    }

    file_names
}



// This if just for the print of the stats
static mut OUTPUT_LANGUAGE_SIZE: isize = -15;
static mut OUTPUT_NUMBER_OF_FILES_SIZE: isize = -5;
static mut OUTPUT_SIZE_SIZE: isize = -4;
static mut OUTPUT_BLANK_SIZE: isize = -11;
static mut OUTPUT_COMMENT_SIZE: isize = -13;
static mut OUTPUT_CODE_LINES_SIZE: isize = -10;
static mut OUTPUT_TOT_LINES_SIZE: isize = -5;

static mut OUTPUT_NUMBER_OF_FILES_PER_LANGUAGE: isize = 0;
static mut OUTPUT_SIZE_PER_LANGUAGE: isize = 0;
static mut OUTPUT_BLANK_PER_LANGUAGE: isize = 0;
static mut OUTPUT_COMMENT_PER_LANGUAGE: isize = 0;
static mut OUTPUT_CODE_LINES_PER_LANGUAGE: isize = 0;
static mut OUTPUT_TOT_LINES_PER_LANGUAGE: isize = 0;

impl fmt::Display for ExtractInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, ) -> fmt::Result {

        let mut size_number_of_files = 0;
        let mut size_size = 0;
        let mut size_blank = 0;
        let mut size_comment = 0;
        let mut size_code_lines = 0;
        let mut size_tot_lines = 0;
        let mut size_hyphen = 0;

        let mut tot_number_of_files = 0;
        let mut tot_size = 0;
        let mut tot_blank = 0;
        let mut tot_comment = 0;
        let mut tot_code_lines = 0;
        let mut tot_tot_lines = 0;

        unsafe {
            size_number_of_files = OUTPUT_NUMBER_OF_FILES_SIZE + self.number_of_files.to_string().len() as isize + 1;
            match size_number_of_files > 1 {
                true => OUTPUT_NUMBER_OF_FILES_PER_LANGUAGE = OUTPUT_NUMBER_OF_FILES_SIZE.abs() + size_number_of_files,
                false => {
                    size_number_of_files = 1;
                    OUTPUT_NUMBER_OF_FILES_PER_LANGUAGE = OUTPUT_NUMBER_OF_FILES_SIZE.abs() + 1;
                }
            }
            size_size = OUTPUT_SIZE_SIZE + self.total_size.to_string().len() as isize + 1;
            match size_size > 1 {
                true => OUTPUT_SIZE_PER_LANGUAGE = OUTPUT_SIZE_SIZE.abs() + size_size,
                false => {
                    size_size = 1;
                    OUTPUT_SIZE_PER_LANGUAGE = OUTPUT_SIZE_SIZE.abs() + 1;
                }
            }
            size_blank = OUTPUT_BLANK_SIZE + self.tot_blank_lines.to_string().len() as isize + 1;
            match size_blank > 1 {
                true => OUTPUT_BLANK_PER_LANGUAGE = OUTPUT_BLANK_SIZE.abs() + size_blank,
                false => {
                    size_blank = 1;
                    OUTPUT_BLANK_PER_LANGUAGE = OUTPUT_BLANK_SIZE.abs() + 1;
                }
            }
            size_comment = OUTPUT_COMMENT_SIZE + self.tot_comment_lines.to_string().len() as isize + 1;
            match size_comment > 1 {
                true => OUTPUT_COMMENT_PER_LANGUAGE = OUTPUT_COMMENT_SIZE.abs() + size_comment,
                false => {
                    size_comment = 1;
                    OUTPUT_COMMENT_PER_LANGUAGE = OUTPUT_COMMENT_SIZE.abs() + 1;
                }
            }
            size_code_lines = OUTPUT_CODE_LINES_SIZE + self.tot_code_lines.to_string().len() as isize + 1;
            match size_code_lines > 1 {
                true => OUTPUT_CODE_LINES_PER_LANGUAGE = OUTPUT_CODE_LINES_SIZE.abs() + size_code_lines,
                false => {
                    size_code_lines = 1;
                    OUTPUT_CODE_LINES_PER_LANGUAGE = OUTPUT_CODE_LINES_SIZE.abs() + 1;
                }
            }
            size_tot_lines = OUTPUT_TOT_LINES_SIZE + self.tot_lines.to_string().len() as isize + 1;
            match size_tot_lines > 1 {
                true => OUTPUT_TOT_LINES_PER_LANGUAGE = OUTPUT_TOT_LINES_SIZE.abs() + size_tot_lines,
                false => {
                    size_tot_lines = 1;
                    OUTPUT_TOT_LINES_PER_LANGUAGE = OUTPUT_TOT_LINES_SIZE.abs() + 1;
                }
            }
            size_hyphen = (size_number_of_files + size_size + size_blank + size_comment + size_code_lines + size_tot_lines).abs();
            match size_hyphen > 6 {
                true => size_hyphen = size_hyphen - 5,
                false => size_hyphen = 1,
            }

            tot_number_of_files = OUTPUT_TOT_LINES_PER_LANGUAGE - self.number_of_files.to_string().len() as isize;
            match tot_number_of_files > 1 {
                true => (),
                false => tot_number_of_files = 1,
            }
            tot_size = OUTPUT_SIZE_PER_LANGUAGE - self.total_size.to_string().len() as isize;
            match tot_size > 1 {
                true => (),
                false => tot_size = 1,
            }
            tot_blank = OUTPUT_BLANK_PER_LANGUAGE - self.tot_blank_lines.to_string().len() as isize;
            match tot_blank > 1 {
                true => (),
                false => tot_blank = 1,
            }
            tot_comment = OUTPUT_COMMENT_PER_LANGUAGE - self.tot_comment_lines.to_string().len() as isize;
            match tot_comment > 1 {
                true => (),
                false => tot_comment = 1,
            }
            tot_code_lines = OUTPUT_CODE_LINES_PER_LANGUAGE - self.tot_code_lines.to_string().len() as isize;
            match tot_code_lines > 1 {
                true => (),
                false => tot_code_lines = 1,
            }
            tot_tot_lines = OUTPUT_TOT_LINES_PER_LANGUAGE - self.tot_lines.to_string().len() as isize;
            match tot_tot_lines > 1 {
                true => (),
                false => tot_tot_lines = 1,
            }
        }

        let mut s = format!("
Number of files ignored: {}
Number of directories: {}
|----------------------------------------------------------------------------------{}|
| Language        |{}Files |{}Size |{}Blank lines |{}Comment lines |{}Code lines |{}TOTAL |
|----------------------------------------------------------------------------------{}|
"
,           self.number_of_files_ignore, self.number_of_directories, 
            "-".repeat( size_hyphen as usize ),
            " ".repeat( size_number_of_files as usize ),
            " ".repeat( size_size as usize ),
            " ".repeat( size_blank as usize ),
            " ".repeat( size_comment as usize ),
            " ".repeat( size_code_lines as usize ),
            " ".repeat( size_tot_lines as usize ),
            "-".repeat( size_hyphen as usize ),
            );

        for stat in self.stats_per_language.iter() {
            s += &format!("{}", stat).as_str();
        }

        s += format!(
"|----------------------------------------------------------------------------------{}|
| Total           |{}{} |{}{} |{}{} |{}{} |{}{} |{}{} |
|----------------------------------------------------------------------------------{}|
"
,           "-".repeat( size_hyphen as usize ),
            " ".repeat( tot_number_of_files as usize ), self.number_of_files, 
            " ".repeat( tot_size as usize ), self.total_size,
            " ".repeat( tot_blank as usize ), self.tot_blank_lines,
            " ".repeat( tot_comment as usize ), self.tot_comment_lines,
            " ".repeat( tot_code_lines as usize ), self.tot_code_lines,
            " ".repeat( tot_tot_lines as usize ), self.tot_lines,
            "-".repeat(size_hyphen as usize),
        ).as_str();
        write!(f, "{}", s)
    }
}

impl fmt:: Display for StatPerLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        let mut size_language = 0;
        let mut size_number_of_files = 0;
        let mut size_size = 0;
        let mut size_blank = 0;
        let mut size_comment = 0;
        let mut size_code_lines = 0;
        let mut size_tot_lines = 0;

        unsafe {
            size_language = OUTPUT_LANGUAGE_SIZE.abs() - self.language.len() as isize + 1;
            match size_language > 1 {
                true => (),
                false => size_language = 1,
            }
            size_number_of_files = OUTPUT_NUMBER_OF_FILES_PER_LANGUAGE - self.number_of_files.to_string().len() as isize;
            match size_number_of_files > 1 {
                true => (),
                false => size_number_of_files = 1,
            }
            size_size = OUTPUT_SIZE_PER_LANGUAGE - self.total_size.to_string().len() as isize;
            match size_size > 1 {
                true => (),
                false => size_size = 1,
            }
            size_blank = OUTPUT_BLANK_PER_LANGUAGE - self.tot_blank_lines.to_string().len() as isize;
            match size_blank > 1 {
                true => (),
                false => size_blank = 1,
            }
            size_comment = OUTPUT_COMMENT_PER_LANGUAGE - self.tot_comment_lines.to_string().len() as isize;
            match size_comment > 1 {
                true => (),
                false => size_comment = 1,
            }
            size_code_lines = OUTPUT_CODE_LINES_PER_LANGUAGE - self.tot_code_lines.to_string().len() as isize;
            match size_code_lines > 1 {
                true => (),
                false => size_code_lines = 1,
            }
            size_tot_lines = OUTPUT_TOT_LINES_PER_LANGUAGE - self.tot_lines.to_string().len() as isize;
            match size_tot_lines > 1 {
                true => (),
                false => size_tot_lines = 1,
            }

            
        }

        let s = format!(
"| {}{}|{}{} |{}{} |{}{} |{}{} |{}{} |{}{} |
"
,       self.language, " ".repeat( size_language as usize ),
        " ".repeat( size_number_of_files as usize ), self.number_of_files, 
        " ".repeat( size_size as usize ), self.total_size, 
        " ".repeat( size_blank as usize ), self.tot_blank_lines, 
        " ".repeat( size_comment as usize ), self.tot_comment_lines, 
        " ".repeat( size_code_lines as usize ), self.tot_code_lines, 
        " ".repeat( size_tot_lines as usize ), self.tot_lines
        );
        write!(f, "{}", s)
    }
}
