use std::thread::sleep;
use std::process::Command;
use std::process::Output;
use std::time::Duration;
use std::str;

fn main() {
    let number_of_cols: Output = 
        Command::new("sh")
            .arg("-c")
            .arg("tput cols")
            .output()
            .expect("There was an issue getting output!");

    
    let string_output: &str = match std::str::from_utf8(&number_of_cols.stdout) {
        Ok(v) => v,
        Err(e) => panic!("error: invalid ASCII code in output: {}", e)
    };

    let max: usize = string_output.trim_end().parse().expect("This isn't a number!");
    let mut count: usize = 0;
    let time = Duration::from_millis(40);
    let mut direction: isize = 1;

    println!("{:?}", string_output);
    loop {
        println!("{}", format!("{:\\^1$}", '/', count));
        sleep(time);
        if count == max {
            direction = -1;
        }
        if count == 0 {
            direction = 1;
        }
        if direction == 1 {
            count = count + 1;
        }
        if direction == -1 {
            count = count - 1;
        }
    }
   
}
