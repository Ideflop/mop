use std::env::args;

use file_handler::FileHandler;
use languages_mapping::Language;
use file_supplier::ExtractInfo;

pub mod file_handler;
pub mod languages_mapping;
pub mod file_supplier;

fn main() {
    let mut args = args().skip(1).collect::<Vec<_>>();

    let mut args_itr = args.iter();
    if let Some(arg) = args_itr.next() {
        match arg.as_str() {
            "-f" => file_supplier::get_file(args.split_off(1)),
            "-t" => (),
            "-d" => file_supplier::get_dir_from_main(args.split_off(1)),
            "-s" => (),
            "-b" => (),
            _ => (),
        }
    }
}
