use std::{
    env,
    process::Command,
    io::{self, Write},
};

use termion::{
    input::TermRead,
    raw::IntoRawMode,
};

use crate::search::SearchResult;

pub fn print_and_choose( list: &Vec<SearchResult>) {
    let mut file_selected = false;
    let mut line_selected = false;
    let mut file_choosen = 0;
    let mut line_choosen = 0;

    loop {
        print!("{}[2J", 27 as char);
        print!("{}[1;1H", 27 as char);
        if file_selected {
            println!("The file {} is selected", list[file_choosen-1].file_name);
        } else {
            println!("No file is selected");
            println!("Press enter to select a file");
        }
        println!("");
        for (index, item) in list.iter().enumerate() {
            if file_choosen != 0 {
                let index_string = format!("{} ", index + 1);
                if index_string.starts_with(file_choosen.to_string().as_str()) {
                    println!("{}) {}", index + 1,  item)
                }
            }
            else {
                println!("{}) {}", index + 1,  item)
            }
        }

        let stdin = io::stdin();
        let stdout = io::stdout().into_raw_mode().unwrap(); // for interactive terminal without need to press enter
        io::stdout().flush().unwrap();

        let event= stdin.keys().next().unwrap().unwrap();

        match event {
            termion::event::Key::Char('q') => {
                println!("\r");
                break;
            }
            termion::event::Key::Char('\n') => {
                if file_choosen != 0 {
                    file_selected = true;
                }
                if file_selected && line_choosen != 0 {
                    line_selected = true;
                }

                if file_selected && line_selected {
                    for (index, item) in list.iter().enumerate() {
                        if index == file_choosen - 1 {
                            let editor = match env::var("EDITOR") {
                                Ok(val) => val,
                                Err(_) => {
                                    println!("The $EDITOR environment variable is not set.");
                                    return;
                                }
                            };
                            let line = item.lines[line_choosen - 1].0;
                            Command::new("/usr/bin/sh")
                                                .arg("-c")
                                                .arg(format!("{} +{} {}",editor, line, item.file_name))
                                                .spawn()
                                                .expect("Error: Failed to run editor")
                                                .wait()
                                                .expect("Error: Editor returned a non-zero status");
                            print!("{}[2J", 27 as char);
                            print!("{}[1;1H", 27 as char);
                            return;
                        }
                    }
                }
            }
            termion::event::Key::Backspace | termion::event::Key::Delete=> {
                if file_choosen != 0 {
                    if line_choosen != 0 && file_selected {
                        if line_choosen.to_string().len() == 1 {
                            line_choosen = 0;
                        } else {
                            line_choosen = line_choosen / 10;
                        }
                        line_selected = false;
                    } else {
                        if file_choosen.to_string().len() == 1 {
                            file_choosen = 0;
                        } else {
                            file_choosen = file_choosen / 10;
                        }
                        file_selected = false;
                    }
                }
            }
            termion::event::Key::Char(c) => {
                if c.is_digit(10) {
                    if file_choosen != 0 {
                        if file_selected {
                            if line_choosen != 0 {
                                line_choosen = line_choosen * 10 + c.to_digit(10).unwrap() as usize;
                            } else {
                                line_choosen = c.to_digit(10).unwrap() as usize;
                            }
                        } else {
                            file_choosen = file_choosen * 10 + c.to_digit(10).unwrap() as usize;
                        }
                    } else {
                        file_choosen = c.to_digit(10).unwrap() as usize;
                    }
                }
            }
            _ => {
            }
        }
    }
}
