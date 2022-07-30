# jabba-ctc

The name "ctc" stands for _**c**opy **t**ext to **c**lipboard_. This is a binary crate.

`ctc` can copy its argument (as text) to the clipboard OR it can
copy the content of a text file to the clipboard.

```
$ ctc --help
ctc (copy text to clipboard) v0.1.1

Usage examples:

* ctc                   print help
* ctc -h, ctc --help    print help
* ctc <text>            copy the given text to clipboard
* ctc -f <file>         copy the content of the given file to clipboard
```

Supported platforms: Windows and Linux (with X server).

## Windows

Put `ctc.exe` to a folder that is in your PATH.

Example:

```
c:\> ctc hello.txt
```

Now the text "**hello.txt**" is copied to the clipboard.

## Linux

`ctc` relies on the external command `xclip` to manipulate the content of the clipboard.
Thus, you must install `xclip` using your package manager (under Ubuntu it's
`sudo apt install xclip`).

Under Linux, there are two clipboards. They are called "primary" and "clipboard". `ctc`
puts the text on both of them, thus you can insert the text with one of the following
methods: Ctrl+v, Shift+Insert, or mouse middle click.

See the example above, it works similarly under Linux.

## Installation

`ctc` is written in Rust. If you have the Rust compiler, you can install it directly
from crates.io using the command `cargo`:

    $ cargo install jabba-ctc

## Links

* [jabba-cpc](https://github.com/jabbalaci/jabba-cpc): **c**opy **p**ath to **c**lipboard
