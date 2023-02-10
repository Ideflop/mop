use std::env::args;

pub mod entry_point;

pub mod file_handler;
pub mod languages_mapping;
pub mod file_supplier;
pub mod search;
pub mod search_print;

fn main() {
    let mut args = args().skip(1).collect::<Vec<_>>();

    let mut args_itr = args.iter();
    if let Some(arg) = args_itr.next() {
        match arg.as_str() {
            "-h" | "--help" => (),
            "-m" | "--metric" => entry_point::get_stat(args.split_off(1)),
            "-t" | "--todo" => entry_point::search_for(args),
            "-s" | "--search" => entry_point::search_for(args.split_off(1)),
            "-b" => (),
            _ => (),
        }
    }
}
