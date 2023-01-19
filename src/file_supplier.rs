struct extracInfo {
    number_of_files: usize,
    number_of_directories: usize,
    total_size: usize, // c'est drole

    // don't know now if needed
    tot_lines : usize,
    tot_blank_lines: usize,
    tot_comment_lines: usize,
    tot_code_lines: usize,
}

impl extracInfo {
    fn new() -> extracInfo {
        extracInfo {
            number_of_files: 0,
            number_of_directories: 0,
            total_size: 0,
            tot_lines : 0,
            tot_blank_lines: 0,
            tot_comment_lines: 0,
            tot_code_lines: 0,
        }
    }


}
