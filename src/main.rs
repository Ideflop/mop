use std::env::args;

use file_handler::FileHandler;
use languages_mapping::Language;
use file_supplier::ExtractInfo;

pub mod file_handler;
pub mod languages_mapping;
pub mod file_supplier;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();

    let mut args_itr = args.iter();
    if let Some(arg) = args_itr.next() {
        match arg.as_str() {
            "-f" => (),
            "-t" => (),
            "-d" => (),
            _ => {
                let a = ExtractInfo::new();
                a.get_argument(&arg)
            }
        }
    }

    

}
