use spl_parser::{parse_from_file};
use spl_analyser::walker::Walker;
use clap::{Arg, Command, ArgAction};
use colored::Colorize;

fn main() -> Result<(), String> {
    let args = Command::new("Incredibuild")
        .about("Compile SPL code to executable")
        .arg(Arg::new("input").index(1).required(true))
        .arg(Arg::new("output").short('o').long("output").required(false))
        .arg(Arg::new("debug").short('d').long("debug").required(false).action(ArgAction::SetTrue))
        .get_matches();

    let source_path = args.get_one::<String>("input").unwrap();

    let parsed_input = parse_from_file(&source_path);
    match parsed_input {
        Ok(_) => println!("{}", "Parsed successfully".green()),
        Err(e) => {
            println!("{}", e.red());
            return Err("Error in parsing".to_string());
        }
    }

    let mut walker = Walker::new(&source_path, args.get_flag("debug"));
    walker.traverse();
    let errors = walker.get_errors();
    for error in errors {
        println!("{}", error.to_string().red());
    }
    if errors.len() > 0 {
        return Err("Error in analysis".to_string());
    }

    Ok(())
}
