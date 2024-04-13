use std::io;
use clipboard_win::{formats, set_clipboard};

fn main() {
    let stdin = io::stdin();
    let text = io::read_to_string(stdin).expect("Can not read stdin");
    set_clipboard(formats::Unicode, text.trim_end()).expect("Can not set on clipboard");
}

