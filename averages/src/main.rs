/**
 * The goal is to create a program that accepts a series of
 * user inputs and then select the type of average they
 * want to output (mean, mediun, mode)
 */

use std::io;

const INPUT_MEAN: i32 = 1;
const INPUT_MEDIAN: i32 = 2;
const INPUT_MODE: i32 = 3;

fn mean(list: &Vec<i32>) -> f32 {
    println!("Passed into mean: {:?}", list);
    let mut sum: i32 = 0;
    let mut number_of_entries = 0;
    for entry in list.iter() {
        sum = sum + entry;
        number_of_entries = number_of_entries + 1;
    }
    (sum as f32) / (number_of_entries as f32)
}

fn median(list: &Vec<i32>) -> f32 {
    println!("Passed into median: {:?}", list);
    let mut sorted: Vec<i32> = list.to_vec();
    sorted.sort();
    let number_of_entries: i32 = sorted.len() as i32;
    println!("Sorted list: {:?}", sorted);

    match number_of_entries % 2 {
        0 => { // even
            println!("Even number of entries!");
            let right_index = (number_of_entries / 2) as usize;
            let left_index = right_index - 1;
            let left_value = match sorted.get(left_index) {
                Some(num) => *num,
                None => panic!("Index error in left value of even median block!")
            };
            let right_value = match sorted.get(right_index) {
                Some(num) => *num,
                None => panic!("Index error in right value of even median block")
            };

            ((left_value + right_value) / 2) as f32
        },
        _ => { // odd
            let index: usize = (number_of_entries / 2) as usize;
            let value: i32 = match sorted.get(index) {
                Some(num) => *num,
                None => panic!("Index error in odd median block!")
            };
            value as f32
        }
    }
}

fn mode(list: &Vec<i32>) -> f32 {
    println!("Passed into mode: {:?}", list);
    3.0
}


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

    let mut numbers_list: Vec<i32> = Vec::new();

    loop {
        println!("Please input a number to the list, or a non-number to end inputs:");
        println!("Range: [-2147483647, 2147483648]\n");

        let mut current_input = String::new();

        io::stdin().read_line(&mut current_input).expect("Failed to read line");

        let numerical_input: i32 = match current_input.trim().parse() {
            Ok(num) => num,
            Err(_) => break
        };

        numbers_list.push(numerical_input);
        println!("List: {:?}", numbers_list);
    }

    println!("Input done.");

    loop {
        println!("Please enter the number corresponding to what average you're looking to compute (or non-numerical input to quit):
            1) Mean
            2) Median
            3) Mode
        ");

        let mut current_input = String ::new();

        io::stdin().read_line(&mut current_input).expect("Failed to read line");

        let numerical_input: i32 = match current_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Quitting...");
                break;
            }
        };

        let decision: f32 = match numerical_input {
            INPUT_MEAN => mean(&numbers_list),
            INPUT_MEDIAN => median(&numbers_list),
            INPUT_MODE => mode(&numbers_list),
            _ => {
                println!("Unexpected error! Quitting!");
                break;
            }
        };
        println!("The average is: {}", decision);
        break;
    }

}