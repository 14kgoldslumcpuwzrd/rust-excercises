/**
 * The goal is to create a program that accepts a series of
 * user inputs and then select the type of average they
 * want to output (mean, mediun, mode)
 */

use std::io;


fn main() {
    println!(" 
     █████╗ ██╗   ██╗███████╗██████╗  █████╗  ██████╗ ███████╗██████╗          
    ██╔══██╗██║   ██║██╔════╝██╔══██╗██╔══██╗██╔════╝ ██╔════╝██╔══██╗         
    ███████║██║   ██║█████╗  ██████╔╝███████║██║  ███╗█████╗  ██████╔╝         
    ██╔══██║╚██╗ ██╔╝██╔══╝  ██╔══██╗██╔══██║██║   ██║██╔══╝  ██╔══██╗         
    ██║  ██║ ╚████╔╝ ███████╗██║  ██║██║  ██║╚██████╔╝███████╗██║  ██║██╗██╗██╗
    ╚═╝  ╚═╝  ╚═══╝  ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝╚═╝╚═╝
    .
    .
    .");

    let mut numbers_list: Vec<u32> = Vec::new();

    loop {
        println!("Please input a number to the list, or a non-number to end inputs:");

        let mut current_input = String::new();

        io::stdin().read_line(&mut current_input).expect("Failed to read line");

        let numerical_input: u32 = match current_input.trim().parse() {
            Ok(num) => num,
            Err(_) => break
        };

        numbers_list.push(numerical_input);
    }

    println!("Input done.");
    println!("{:?}", numbers_list);

}
