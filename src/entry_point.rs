use std::{
    fs,
    path::Path,
    process::exit,
    collections::HashSet,
    sync::Mutex,
    sync::atomic::{
        AtomicUsize, 
        Ordering
    },
};

use rayon::prelude::*;

use crate::file_supplier::ExtractInfo;
use crate::search::Search;

pub fn get_stat(arg: Vec<String>) {
    if arg.is_empty() {
        println!("No file or directory specified");
        exit(1);
    }

    let file = get_files(arg);
    get_file_stat(file);
}

pub fn search_for(arg: Vec<String>) {
    if arg.is_empty() {
        println!("No argument given");
        exit(1);
    }
     
    let mut arg_copy = arg.clone();

    let pattern = match arg[0].as_str() {
        "-t" | "--todo" => "TODO",
       _ => &arg[0],
    };
    println!("Searching for {}\n", pattern);
    arg_copy.remove(0);

    let files = get_files(arg_copy);
    let mut to_search= Search::new(files, pattern);
    to_search.give_and_output_search();
}

fn get_files(args: Vec<String>) -> Vec<String> {
    if args.is_empty() {
        println!("No file specified");
        exit(1);
    }
    let mut file = Vec::new();
    for arg in args {
        match Path::new(&arg).is_file() {
            true => {
                file.push(arg);
            }
            false => {
                match Path::new(&arg).is_dir() {
                    true => {
                        let a = get_files_in_path(&arg.as_str());
                        file.extend(a);
                    }
                    false => {
                        println!("{} is not a file or directory", arg);
                    }
                }
            }
        }
    }

    let file = remove_duplicate(file);

    if file.is_empty() { // needed ?
        println!("No file found");
        exit(1);
    }
    file
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

fn remove_duplicate(file: Vec<String>) -> Vec<String> {
    let file : Vec<String> = file.into_iter().collect::<HashSet<String>>().into_iter().collect();
    file
}

fn get_file_stat(file: Vec<String>) {
    let mut extract_info = ExtractInfo::new();

    let number_of_files = AtomicUsize::new(0);
    let number_of_files_ignore = AtomicUsize::new(0);
    let total_size = AtomicUsize::new(0);
    let total_lines = AtomicUsize::new(0);
    let total_blank_lines = AtomicUsize::new(0);
    let total_comment_lines = AtomicUsize::new(0);
    let total_code_lines = AtomicUsize::new(0);
    let file_stats_vec = Mutex::new(vec![]);

    file.par_iter().for_each(|arg| {
        match extract_info.get_argument(&arg) {
            Ok(file_stat) => {
                number_of_files.fetch_add(1, Ordering::Relaxed);
                print!("Number of files : {}\r",  number_of_files.load(Ordering::Relaxed));
                total_size.fetch_add(file_stat.get_size(), Ordering::Relaxed);
                total_lines.fetch_add(file_stat.get_lines(), Ordering::Relaxed);
                total_blank_lines.fetch_add(file_stat.get_blank_lines(), Ordering::Relaxed);
                total_comment_lines.fetch_add(file_stat.get_comment_lines(), Ordering::Relaxed);
                total_code_lines.fetch_add(file_stat.get_code_lines(), Ordering::Relaxed);
                let mut file_stats_vec = file_stats_vec.lock().unwrap();
                file_stats_vec.push(file_stat);
            }
            Err(_) => {
                number_of_files_ignore.fetch_add(1, Ordering::Relaxed);
            }
        }
    });

    extract_info.add_number_of_files(number_of_files.load(Ordering::Relaxed));
    extract_info.add_number_of_files_ignore(number_of_files_ignore.load(Ordering::Relaxed));
    extract_info.add_total_size(total_size.load(Ordering::Relaxed));
    extract_info.add_tot_lines(total_lines.load(Ordering::Relaxed));
    extract_info.add_tot_blank_lines(total_blank_lines.load(Ordering::Relaxed));
    extract_info.add_tot_comment_lines(total_comment_lines.load(Ordering::Relaxed));
    extract_info.add_tot_code_lines(total_code_lines.load(Ordering::Relaxed));
    for file_stat in file_stats_vec.lock().unwrap().iter() {
        extract_info.add_stat_for_each_language(file_stat.to_owned());
    }
    
    print!("\x1B[2K");
    println!("{}", extract_info)
}
