#![allow(unused, dead_code)]
mod panic;
mod result;

fn main() -> Result<(), Box <dyn std::error::Error>> {
    topic::introduce();
    panic::explain();
    result::explain();

    Ok(())
    //Please read the Rust book for info on when to panic, and when to use a result and error handling
}

mod topic {
    pub fn introduce() {
        println!("
        
        ----------------------CHAPTER 9: ERROR HANDLING----------------------
        
        Errors are unavoidable, so Rust provides tools to handle them.
        Rust groups errors into two categories:
            - Recoverable errors, like 'file not found,' where we just want the user to try again
            - Irrecoverable errors, like unwrapping a None, are usually a bug and should terminate the program
        
        Most languages group these errors into one, label them exceptions, and handle them with try catch
        Rust instead uses two tools: the Result<T, E> enum and the panic!() macro.

        First, we'll learn about panic!()
        ");
    }
}

mod main_returns {
    pub fn explain() {
        println!("If you need to return an Option or Result to use ?, are they unusable in main?");
        //Nope, main can return Result<(), Box<dyn Error>> (which is either nothing () or any kind of error)
        //If main returns a result, the executable will exit with 0 if Ok(()) or a non-zero integer if Err
        //Check out the std::process::Termination trait: any types that implement it can be returned by main
    }
}