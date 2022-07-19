use automl::settings::Algorithm;
use std::io::{stdin, stdout, Write};
use std::path::Path;

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

pub fn get_file_path(file_desc: &str, verify_exists: bool) -> String {
    print!("Please enter the {file_desc} file path: ");
    stdout().flush().unwrap();

    let mut path = String::new();
    loop {
        if stdin().read_line(&mut path).is_ok() {
            trim_newline(&mut path);

            if !verify_exists || Path::new(&path).is_file() {
                break;
            }
        }

        path.clear();
        print!("Invalid file path. Please enter the {file_desc} file path: ");
        stdout().flush().unwrap();
    }

    path
}

pub fn get_alg() -> Algorithm {
    println!("Choose a regression algorithm to use");
    println!("1. Linear regression");
    println!("2. Decision tree regression");

    let mut input = String::new();
    loop {
        if stdin().read_line(&mut input).is_ok() {
            trim_newline(&mut input);

            match input.as_ref() {
                "1" => return Algorithm::Linear,
                "2" => return Algorithm::DecisionTreeRegressor,
                _ => (),
            }
        }

        input.clear();
        println!();
        println!("Invalid option, please enter \"1\" or \"2\"");
        println!("Choose a regression algorithm to use");
        println!("1. Linear regression");
        println!("2. Decision tree regression");
    }
}

pub fn yes_or_no(message: &str) -> bool {
    println!("{message} (Y/N): ");

    let mut input = String::new();
    loop {
        if stdin().read_line(&mut input).is_ok() {
            trim_newline(&mut input);

            match input.as_ref() {
                "Y" | "y" => return true,
                "N" | "n" => return false,
                _ => (),
            }
        }

        input.clear();
        println!();
        println!("Invalid option, please enter \"Y\" or \"N\"");
        println!("{message}: ");
    }
}
