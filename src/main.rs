use std::{
    process::exit,
    env::args,
};

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();

    let mut args_itr = args.iter();
    if let Some(arg) = args_itr.next() {
        match arg.as_str() {
            "-f" => (),
            "-t" => (),
            "-d" => (),
            _ => {
            }
        }
    }



}
