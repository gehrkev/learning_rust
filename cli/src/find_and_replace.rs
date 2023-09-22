use text_colorizer::*;
use std::env;
use std::fs;
use regex::Regex;

#[derive(Debug)]
#[allow(dead_code)]
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}


pub fn run() {
    let args = parse_args();
    read_and_write(&args);
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect(); //collect the arguments from env -- skip(1) because run argument is the first one
    if args.len() != 4 {
        print_help();
        eprintln!("{} wrong number of arguments given, Expected 4, got {}", "Error:".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments {
        pattern: args[0].clone(),
        replace: args[1].clone(),
        input_file: args[2].clone(),
        output_file: args[3].clone(),
    }
}

fn print_help() {
    eprintln!("{} - replace a string with a new string", "Find and Replace".green());
    eprintln!("Usage: <target string> <replacement string> <INPUT FILE> <OUTPUT FILE>");
}

fn read_and_write(args: &Arguments) {
    let data = match fs::read_to_string(&args.input_file) {
        Ok(file_to_string) => file_to_string,
        Err(e) => {
            eprintln!("{} failed to read from file {}: {:?}", "Error:".red().bold(), args.input_file, e);
            std::process::exit(1);
        }
    };

    let replace_data = match replace(&args.pattern, &args.replace, &data) { //replace a pattern within the data (input file read)
        Ok(data_to_string) => data_to_string,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output_file, replace_data) { //yanked the & from replace_data because it will be deref'd anyway
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} failed to write to file {}: {:?}", "Error:".red().bold(), args.output_file, e);
            std::process::exit(1);
        }
    }
}

fn replace(target: &str, replace: &str, data: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(data, replace).to_string())
}
