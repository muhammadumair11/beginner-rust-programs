use std::io;


fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Message not read");

    let mut checker_vec: Vec<char> = Vec::new();

    
    for item in input.trim().chars() {
        match checker_vec.last() {
            Some(&last_char) if last_char == item => {
                checker_vec.pop(); 
            }
            _ => {
                checker_vec.push(item); 
            }
        }
    }


    match checker_vec.is_empty() {
        true => println!("Input is palindrome"),
        false => println!("Input is not a palindrome"),
        
    }

}
