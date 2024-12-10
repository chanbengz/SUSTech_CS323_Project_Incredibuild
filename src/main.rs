use spl_parser::{parse_from_file};
use spl_analyser::walker::Walker;
use clap::{Arg, Command};
use colored::Colorize;

fn main() {
    let args = Command::new("Incredibuild")
        .about("Compile SPL code to executable")
        .arg(
            Arg::new("input").index(1).required(true)
        )
        .arg(
            Arg::new("output").short('o').long("output").required(false)
        )
        .get_matches();

    let source_path = args.get_one::<String>("input").unwrap();

    let parsed_input = parse_from_file(&source_path);
    match parsed_input {
        Ok(_) => println!("{}", "Parsed successfully".green()),
        Err(e) => {
            println!("{}", e.red());
        }
    }

    let mut walker = Walker::new(&source_path);
    walker.traverse();
    let errors = walker.get_errors();
    for error in errors {
        println!("{}", error.to_string().red());
    }
}
