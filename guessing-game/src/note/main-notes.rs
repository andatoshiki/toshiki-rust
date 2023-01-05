use std::io; // imports the default library into scope, the io library comes from the standard library, this input provides a lot of features for convinience, such recording input in this scenario
use rand::Rng; // imports the rand dependencies added in cargo, 
use std::cmp::Ordering; // imports the type std::cmp::Ordering from the std library

fn main() { // The main (fn) function is the entry point into the program, fn syntax declares a new function; the parentheses indicates no parameters denoted, the curly brackets indicates the start of the function
    println!("Hello, world!"); // println! is a macro that prints strings to screen

    println!("Please input your guess."); // same as the previous line

    let secret_number = rand::thread_rng().gen_range(1..=100); // use rand to generate a number to guess
    // the rand::thread_rng function provides us the particular random number we're going to use
    // gen_range is the method to generate a range of random numbers, while the range is between 1 to 100

    let guess:u32 = guess.trim().parse().espect("Please type a number!"); // create a variable named guess, while we have already created one, but this ensures us to convert between different values types
    // 

    println!("The secret number is: {secret_number}");

    let mut guess = String::new(); // create a variable to store the user input, use let statement to create the variable
    // let apples = 5; indicates letting a variable with the string named apple equal to an integer 5: immutable
    // let mut bananas = 5; indicates letting a variable equal to a value of 5 but is mutable because of the mut statement for making mutable variables: mutable
    // :: syntax in the new ::new line indicates new is an associated function of the String type. In this case the new function creates a new string with String::new()
    
    // starting to revieve user input
    io::stdin() // corresponds to the prior import of the io library, if not imported, the stdin function returns an instance of std:io::Stdin 
        .read_line(&mut guess) // read_line method is the standard input handle for dealing with terminal input
        // pass the &mut guess argument to read_line to tell whatever the user types into the input 
        // because the input need to be appended as a string, hence the string argument must be mutable in order to change the string content
        // &mut's & indicates the argument is a reference

        .expect("Failed to read line"); // it is better to break the lines our else the code would be written as follows
        // io::stdin().read_line(&mut guess).expect("Failed to read line"); hard to read
        // always introduce a new line when intriducing a new method with .method_name() syntax
        // expect is a method to handle the outcome of an unespected event, if we do not have .expect method, the program will be compiled but will bump out warning
    
    println!("You guessed: {guess}"); // println with placeholders, variable names can go between the curly brackets as placeholders, the guess variable is the input string defiled earlier when input, see practice.rs for further explanation

    match guess.cmp(&secret_number) { // cmp method compares the two different values and can be called on anything that can be compared, matches the input string, valued as guess, to the secret number defined later and make the comparison
        // Ordering is another type of enum that contains three different types of variants, Less, Greater and Equal
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    // up till here, the code won't compile, because the guess variable is inputted as a string, while the secret number is an integer, hence the rust system won't accpet
    // hence if we wan to convert the input String to an integer, it is required to add up the line as follows
    // let guess: u32 = guess.trim().parse().expect("Please type a number!"); see line 20 in main.rs for a clearer version
}
