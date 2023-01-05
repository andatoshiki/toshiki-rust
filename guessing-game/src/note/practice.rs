fn main() {
    let x = 5;
    let y = 10;

    println!("x = {x} and y + 2 = {}", y + 2);
    // the {x}, x within the curly brackets placeholders indicates println macro to print out the x defined earlier with the value of 5
    // while the {} empty curly brackets placeholders indicates an empty space, substituting with the expresion calculated after the coma, y + 2, while y is defined with the value of 10, hence 10 + 2 = 12
    // hence the program will print x = 5 and y + 2 = 12 on the terminal interface
}