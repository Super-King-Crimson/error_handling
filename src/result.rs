pub mod propagation;

use std::{fs::File, io::{self, Error, ErrorKind}};

pub fn explain() {
    //Has two values: Ok (type T) and Err (type E)
    //They allow an err to be handled: this returns a Ok<std::fs::File> or an Err<std::io::Error>
    let greeting_file_result: Result<File, Error> = File::open("hello.txt");

    //Result's in the prelude btw
    let greeting_file = match greeting_file_result {
        Ok(f) => f,
        Err(e) => match e.kind() {
            //File not found error: make new file
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(new) => new,
                Err(new_e) => panic!("Failed to create file: {new_e:?}"),
            },

            //Any other error (like "don't have permission to open"): panic!
            other_err => panic!("Failed to open file: {e:?}"),
        }
    };

    //This is a lot of matching! In chapter 13 we'll learn about closures (which makes the code above look like this)
    let greeting_file_result: Result<File, Error> = File::open("goodbye.txt");
    let greeting_file: File = greeting_file_result.unwrap_or_else(|e| {
        if e.kind() == ErrorKind::NotFound {
            File::create("goodbye.txt").unwrap_or_else(|new_e| panic!("Failed to create file: {new_e:?}"))
        } else {
            panic!("Failed to find file: {e:?}");
        }
    });

    //expect and unwrap are shortcuts for panic on None/Err (expect - explain why you operation should succeed)
    
    //Wait, why'd you make me handle the error?
    let result = propagation::explain(); 
    propagation::specifics::explain();
}