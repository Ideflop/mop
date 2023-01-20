use crate::file_handler::FileHandler;

pub struct ExtractInfo {
    number_of_files: usize,
    number_of_directories: usize,
    total_size: usize, // c'est drole

    // don't know now if needed
    tot_lines : usize,
    tot_blank_lines: usize,
    tot_comment_lines: usize,
    tot_code_lines: usize,
}

impl ExtractInfo {
    pub fn new() -> ExtractInfo {
        ExtractInfo {
            number_of_files: 0,
            number_of_directories: 0,
            total_size: 0,
            tot_lines : 0,
            tot_blank_lines: 0,
            tot_comment_lines: 0,
            tot_code_lines: 0,
        }
    }

    pub fn get_argument(&self, arg: &str) {
        let file = FileHandler::new(&arg);
        let a = file.get_file_stat();
        println!("{:?}", a);
    }

}

// TODO: add the function that takes the argument, call ExtractInfo where there is fn that take all
// file and give them to bla bla. return to the function ExtractInfo and the call print 
