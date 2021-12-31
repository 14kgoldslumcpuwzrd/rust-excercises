use std::io;

/**
 * Takes user terminal input and outputs pig-latin to the terminal
 * Next step: Make this a library function that accepts strings as well as files. (assume space delimiter)
 */
fn main() {
    // Defined Constants
    const DELIMETER: char = ' ';
    const DASH: char = '-';
    const SUFFIX: &str = "ay";

    //Defined variables
    let mut input = String::new();
    let mut words_from_input: Vec<String> = Vec::new();
    let mut current_word = String::from("");
    let mut converted_string = String::from("");
    
    // Algorithm

        // Get input
        println!("input string to be converted to pig latin:");
        io::stdin().read_line(&mut input).expect("failed to read input!");

        // Grab words from input after removing leading/trailing whitespaces
        for letter in input.trim_start().chars() {
            if letter != DELIMETER {
                current_word.push(letter);
            }
            else {
                words_from_input.push(current_word);
                current_word = String::from("");
            }
        };

        // Check if last word exists and isn't a newline character (from user enter input)
        if current_word != "\n" {
            current_word.pop();
            words_from_input.push(current_word);
        }

        // Replace the first consenent at the end of each word from input
        for word in &words_from_input {
            let mut chars = word.chars();
            let first_letter = match chars.next() {
                Some(item) => item,
                None => panic!("Error: somehow an empty word got through a place it logically shouldn't have."),
            };
            let new_word: String = String::from(chars.as_str()) 
                + &String::from(DASH)
                + &String::from(first_letter) 
                + &String::from(SUFFIX);


            // println!("first letter: {}", first_letter);
            // println!("old word: {}", word);
            // println!("new word: {}", new_word);
            converted_string = converted_string + &new_word + " ";
        }

        println!("Converted string:\n {}", converted_string);
}