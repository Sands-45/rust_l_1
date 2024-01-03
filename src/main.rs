use std::io; // input or output library that somes from standard library also know as std

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

     // we create a variable using "let" and then use the key word "mut" so the value is mutable
     // "String" is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.
     // The :: syntax in the ::new line indicates that new is an associated function of the String type. 
     // An associated function is a function that’s implemented on a type, in this case String.
     // This new function creates a new, empty string.
     // In full, the let mut guess = String::new(); line has created a mutable variable
     // that is currently bound to a new, empty instance of a String.
    let mut guess = String::new();

    // We call the stdin function from the io module, which will allow us to handle user input
    // The stdin function returns an instance of std::io::Stdin, which is a type that represents
    // a handle to the standard input for your terminal.
    //the line .read_line(&mut guess) calls the read_line method on the standard input handle to get
    // input from the user. We’re also passing &mut guess as the argument to read_line to tell it what
    // string to store the user input in.
    // The & indicates that this argument is a reference, which gives you a way to let multiple parts of 
    // your code access one piece of data without needing to copy that data into memory multiple times.
    // like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}