use std::io;
use std::fs;

/**
 * This app takes a given directory as input and prints the tree structure
 * to standard output.
 */

fn main() {

    // Defined Variables
    let mut input = String::new();
     
    // Get Input
    println!("Enter directory you'd like to visualize:");
    io::stdin().read_line(&mut input).expect("Failed to get input");

    println!("input: {}", input);
    let output = fs::read_dir(&input.as_str()).unwrap();

    for path in output {
        println!("Name: {}", path.unwrap().path().display())
    }
}
