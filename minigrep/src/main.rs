use std::{ process };

use clap::{self, clap_app};

use minigrep::Config;

fn main() {
    let matches = clap_app!(minigrep =>
        (version: "1.0")
        (author: "Leonardo Espinosa <leo@dabinsi.com>")    
        (about: "Minigrep from The Book (Rust) <https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html>")
        (@arg SEARCHSTRING: -s --searchstring +required +takes_value "String to search")
        (@arg FILENAME: -f --filename +required +takes_value "File where to search")
        (@arg CASE_SENSITIVE: -c --casesensitive +takes_value "Using search case sensitive")
    ).get_matches();

    // println!("Using search string: {}", matches.value_of("SEARCHSTRING").unwrap());
    // println!("Using file: {}", matches.value_of("FILENAME").unwrap());

    let value =  matches.value_of("CASE_SENSITIVE").unwrap_or("1");
    let case: bool = match value.parse::<u32>() {
                        Ok(n) =>  n != 0,
                        Err(_e) => {
                            if value == "false" { false }
                            else { true }
                        }
                    };

    let config = Config::new(matches.value_of("SEARCHSTRING").unwrap().to_string(), 
                                 matches.value_of("FILENAME").unwrap().to_string(),
                            case,);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }

}

