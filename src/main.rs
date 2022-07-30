mod clipboard;
mod config;
mod help;

use clipboard as cb;
use std::env;
use std::process;

fn print_help_and_exit() {
    help::print_help();
    process::exit(0);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        print_help_and_exit();
    }
    // else
    let arg = args[0].as_str();
    if ["-h", "--help"].contains(&arg) {
        print_help_and_exit();
    }
    // else
    cb::set_text(arg);
}
