use std::io;

fn main() {
    // Creating a new String
    let mut buffer = String::new();
    // Printing out a request to the user to enter a message into the console:
    println!("Enter a message:");
    // using the io module with the stdin(), creating a new handle to access the new standard input stream
    // and, with the read_line() to get a line of input, and we are passing a mutable reference to the buffer variable as an input argument
    let _ = io::stdin().read_line(&mut buffer);
    //Finally, we are printing out the results of the standard input:
    println!("buffer is {}", buffer);
    // we need to use the trim() because the string we get from stdin() will get a new line character at the end, and we don't want that:
    //Another way to parse intead of using the Turbofish operator (::<>) is to explicitely type the variable at the beginning:
    ///because the parse() returns a Result Enum, which will either return the value we want, or an error, we are going to use the
    /// unwrap_or() method, which, if the parse() does not return what we want it to, we can have it default to a given value, in this
    /// case, 0
    let number = buffer.trim().parse::<i32>().unwrap_or(0);
    println!("number + 1 is {}", number + 1);

    //using this method of error handling allows the program to handle the error at runtime, without the program panicking and failing.
}
