use crate::config as cfg;

pub fn print_help() {
    let text = format!("\
ctc (copy text to clipboard) v{ver}

Usage examples:

* ctc                   print help
* ctc -h, ctc --help    print help
* ctc <text>            copy the given text to clipboard
* ctc -f <file>         copy the content of the given file to clipboard

see https://github.com/jabbalaci/jabba-ctc for more info", ver=cfg::VERSION);

    println!("{}", text);
}
