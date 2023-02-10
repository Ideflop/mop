use std::{
    env,
    process::Command,
    io::{self, Write},
};

use termion::{
    color,
    style,
    input::TermRead,
    raw::IntoRawMode,
};

use crate::search::SearchResult;

pub fn print_and_choose( list: &Vec<SearchResult>) {
    let mut file_selected = false;
    let mut line_selected = false;
    let mut file_choosen = 0;
    let mut line_choosen = 0;
    let mut total_pattern_found = 0;
    
    for i in 0..list.len() {
        total_pattern_found += list[i].lines.len();
    }
    
    loop { // TODO display the number of times the pattern is found
        print!("{}[2J", 27 as char);
        print!("{}[1;1H", 27 as char);
        if file_selected {
            println!("The file {}{}{}{}{} is selected", color::Fg(color::Green), style::Bold, list[file_choosen-1].file_name, style::Reset,color::Fg(color::Reset));
        } else {
            print!("No file is selected. ");
            println!("Press {}{}enter{}{} to select a file", color::Fg(color::Red), style::Bold, style::Reset, color::Fg(color::Reset));
        }
        println!("Number of time (// TODO insert the pattern here) was found : {}{}{}{}{}", color::Fg(color::Green), style::Bold, total_pattern_found, style::Reset, color::Fg(color::Reset));
        println!("");
        for (index, item) in list.iter().enumerate() {
            if file_choosen != 0 {
                let index_string = format!("{} ", index + 1);
                if index_string.starts_with(file_choosen.to_string().as_str()) {
                    println!("{}{}{}{}{}) {}", color::Fg(color::Blue), 
                                               style::Bold, index + 1, 
                                               style::Reset,  
                                               color::Fg(color::Reset),  
                                               item,
                                               )
                }
            }
            else {
                println!("{}{}{}{}{}) {}", color::Fg(color::Blue), 
                                           style::Bold, index + 1, 
                                           style::Reset,  
                                           color::Fg(color::Reset),  
                                           item,
                                           )
            }
        }

        let stdin = io::stdin();
        let _stdout = io::stdout().into_raw_mode().unwrap(); // for interactive terminal without need to press enter
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
                            let line = item.lines[line_choosen - 1].0; // TODO add a way to give the line where the pattern in found 
                                                                            // ex : file blaba
                                                                            //     1) [42] pattern
                                                                            // possibility to give 42
                                                                            // + error can append if line choosen not in lines
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
