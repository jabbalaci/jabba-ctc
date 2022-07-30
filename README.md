# jabba-ctc

The name "ctc" stands for _**c**opy **t**ext to **c**lipboard_. This is a binary crate.

`ctc` copies its argument (as text) to the clipboard.

Supported platforms: Windows and Linux (with X server).

## Windows

Put `ctc.exe` to a folder that is in your PATH.

Example:

```
c:\> ctc hello.txt
```

Now the text "**hello.txt**" is copied to the clipboard.

Help: `ctc.exe -h`

## Linux

`ctc` relies on the external command `xclip` to manipulate the content of the clipboard.
Thus, you must install `xclip` using your package manager (under Ubuntu it's
`sudo apt install xclip`).

Under Linux, there are two clipboards. They are called "primary" and "clipboard". `ctc`
puts the text on both of them, thus you can insert the text with one of the following
methods: Ctrl+v, Shift+Insert, or mouse middle click.

See the example above, it works similarly under Linux.

Help: `ctc -h`

## Installation

`ctc` is written in Rust. If you have the Rust compiler, you can install it directly
from crates.io using the command `cargo`:

    $ cargo install jabba-ctc
