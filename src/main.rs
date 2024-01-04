use std::io; // input or output library that somes from standard library also know as std
use rand::Rng; // importing rand

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

        let secret_number = rand::thread_rng().gen_range(1..=100);

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
        .expect("Failed to read line"); // Handling errors from a return Results enum
                                        //The right way to suppress the warning is to actually write error-handling code, but in our case we just want to
                                        // crash this program when a problem occurs, so we can use expect

    println!("You guessed: {guess}");
    //This line prints the string that now contains the user’s input. The {} set of curly brackets is
    // a placeholder: think of {} as little crab pincers that hold a value in place.

    //When printing the value of a variable, the variable name can go inside the curly brackets.
    // When printing the result of evaluating an expression, place empty curly brackets in the format string,
    // then follow the format string with a comma-separated list of expressions to print in each empty curly
    // bracket placeholder in the same order. Printing a variable and the result of an expression in one call
    // to println! would look like this:

    let x = 5;
    let y = 10;

    println!("x = {x} and y + 2 = {}", y + 2);
    //This code would print x = 5 and y + 2 = 12.
}
