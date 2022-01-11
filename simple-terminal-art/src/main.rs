use std::thread::sleep;
use std::process::Command;
use std::process::Output;
use std::str;

fn main() {
    let number_of_cols: Output = 
        Command::new("sh")
            .arg("-c")
            .arg("tput cols")
            .output()
            .expect("There was an issue getting output!");

    
    let mut string_output: &str = match std::str::from_utf8(&number_of_cols.stdout) {
        Ok(v) => v,
        Err(e) => panic!("error: invalid ASCII code in output: {}", e)
    };

    string_output = string_output.trim_end();

    println!("{:?}", string_output);
    // loop {

    // }
   
}
