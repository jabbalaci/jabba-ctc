mod clipboard;
mod config;
mod help;

use clipboard as cb;
use jabba_lib::jfs;
use std::env;
use std::process;

fn print_help_and_exit() {
    help::print_help();
    process::exit(0);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut text = String::new();
    let mut must_set = false;

    if args.len() == 0 {
        print_help_and_exit();
    }
    if args.len() > 2 {
        eprintln!("Error: too many arguments");
        process::exit(1);
    }
    // else, if we have 1 or 2 arguments
    let first = args[0].clone();
    if args.len() == 1 {
        if first == "-h" || first == "--help" {
            print_help_and_exit();
        }
        // else
        text = first.clone();
        must_set = true;
    }
    if args.len() == 2 {
        let second = args[1].clone();
        if first == "-f" {
            let content = jfs::read(&second);
            if content.is_ok() {
                text = content.unwrap();
                must_set = true;
            } else {
                eprintln!("Error: cannot read the file {:?}", second);
                process::exit(1);
            }
        } else {
            eprintln!("Argument error");
            process::exit(1);
        }
    }

    if must_set {
        cb::set_text(&text);
    }
}
