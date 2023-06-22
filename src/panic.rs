use std::env;

pub fn explain() {
    //When a panic happens, the default response is to print a fail msg, unwind and clean the stack, and quit
    //You can also display the call stack to make it easier to trace the panic's source

    env::set_var("RUST_BACKTRACE", "1");
    //bad_code();
    env::set_var("RUST_BACKTRACE", "0");
    //or RUST_BACKTRACE=1 cargo run

    //Panic can be called by native rust
    let vec = vec![10; 5];
    //println!("{}", vec[5]);
    //This is called a buffer overread - can cause security vulnerabilities, reading data beyond structure
}

fn bad_code() {
    panic!("Called the bad program");
}