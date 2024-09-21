use std::io::{self, Result};

fn is_prime(&input: &u32) -> bool {
    match input {
        1 => false,
        2 => true,
        _ => {
            for i in 2..input {
                if input % i == 0 {
                    return false;
                }
            }
            true
        }
    }
}

fn main() -> Result<()> {
    /*
        1: Initialized Input variable as mutable String
        2: Standard read_line taking mutable reference of input variable for user input
        3: Trimming and Parsing user input and shadowing input as immutable unsigned 32 int
    */


    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: u32 = input
                        .trim()
                        .parse()
                        .expect("Expected Number is negative");

    if is_prime(&input) {
        print!("{} is a prime number", input)
    } else {
        print!("{} is not a prime number", input)
    }

    Ok(())
}
