use spl_parser::{parse_from_file};
use spl_analyser::walker::Walker;
use clap::{Arg, Command, ArgAction};
use colored::Colorize;
use spl_irgen::emit::*;

fn main() -> Result<(), String> {
    let args = Command::new("Incredibuild")
        .about("Compile SPL code to executable")
        .arg(Arg::new("input").index(1).required(true))
        .arg(Arg::new("output").short('o').long("output").required(false))
        .arg(Arg::new("debug").short('d').long("debug").required(false).action(ArgAction::SetTrue))
        .arg(Arg::new("llvm-ir").short('l').long("llvm-ir").required(false).action(ArgAction::SetTrue))
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

    let ast = parsed_input?;
    let mut walker = Walker::new(ast.clone(), &source_path, args.get_flag("debug"));
    walker.traverse();
    let errors = walker.print_errors();
    if errors.is_err() {
        return Err(errors.unwrap_err());
    }

    if args.get_flag("llvm-ir") {
        let default_output = format!("{}", source_path.replace(".spl", ".ll"));
        let output_path = args.get_one::<String>("output").unwrap_or(&default_output);
        emit_llvmir_to_file(&source_path, ast, &output_path);
    } else {
        let default_output = format!("{}", source_path.replace(".spl", ".S"));
        let output_path = args.get_one::<String>("output").unwrap_or(&default_output);
        emit_object_to_file(&source_path, ast, output_path);
    }

    Ok(())
}
