use std::{io, fs::{self, File}};

pub fn explain() {
    //The ? operator can only be used if the function returns Result, Option, or the FromResidual trait (AGAIN?)
    // let file: File = File::open("hello.txt")?

    let maybe_char: char = last_char_of_first_line("abcd\nef\nあか\nさ\n気日\n生").unwrap();
    println!("{maybe_char}");
    crate::main_returns::explain();
}


//You heard that right, ? works with Options
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().last()?.chars().last()
}
//It won't automatically convert Results to Options tho - do ok to convert Result and ok_or to convert Option