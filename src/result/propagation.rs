use super::{File, io::{self, Read}};
pub mod specifics;

//When you don't handle the error in a function and make the caller handle it, that's error propagation
//We use io::Error because both methods(File::open(path) and File::read_to_string(&mut self) return it)
pub fn explain() -> Result<String, io::Error> {
    let username_file_result: Result<File, io::Error> = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        //return the entire function out of the error (must include, not last expression of func)
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        //returns entire function: last expression
        Err(e) => Err(e),
    }

    //It can be done easier with ?, the propagating error operator
}


//This does almost the same thing: if a ? operation returns an error, the error is returned from the function
//We still have to return an Ok at the end tho
fn shorter() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)

    //The only difference between ? and a Result match is that ? puts errors through a from function (traits!)
    //The from function changes the error's type into the one returned by the function
    //There's also a standard library function that does this for you (fs::read_to_string)
}