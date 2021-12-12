/**
 * The goal is to create a program that accepts a series of
 * user inputs and then select the type of average they
 * want to output (mean, mediun, mode)
 */
fn main() {
    print_title();
    print_instructions();
    
}


fn print_title() {
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
}

fn print_instructions() {
    println!("
    Please input a list of numbers, delineateed by a comma:");
}