use std::{
    process::exit,
    env::args,
};

use file_handler::FileHandler;
use languages_mapping::Language;

pub mod file_handler;
pub mod languages_mapping;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();

    let mut args_itr = args.iter();
    if let Some(arg) = args_itr.next() {
        match arg.as_str() {
            "-f" => (),
            "-t" => (),
            "-d" => (),
            _ => {
                let mut file = FileHandler::new(&arg);
                file.get_language();
                println!("{:?}", file);
            }
        }
    }

    

}
