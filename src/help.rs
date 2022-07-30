use crate::config as cfg;

pub fn print_help() {
    let text = format!("\
ctc (copy text to clipboard) v{ver}

- copies its argument (as text) to the clipboard

Usage: ctc [option] [text_argument]
where option can be:
    -h or --help            get this help

see https://github.com/jabbalaci/jabba-cpc for more info", ver=cfg::VERSION);

    println!("{}", text);
}
