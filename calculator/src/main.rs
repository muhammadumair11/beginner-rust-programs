use std::io;

fn calculate(first_operand: i64, second_operand: i64, operator: char) -> i64 {
    match operator {
        '+' => first_operand + second_operand,
        '-' => first_operand - second_operand,
        '*' => first_operand * second_operand,
        '/' => first_operand / second_operand,
        _ => -1
    }
}



fn main() {
    let mut first_operand: String = String::new();
    let mut second_operand: String = String::new();
    let mut operator: String = String::new();

    io::stdin().read_line(&mut first_operand).expect("First Operand not entered");
    io::stdin().read_line(&mut second_operand).expect("First Operand not entered");
    io::stdin().read_line(&mut operator).expect("First Operand not entered");

    let first_operand: i64 = first_operand.trim().parse().expect("not valid");
    let second_operand: i64 = second_operand.trim().parse().expect("not valid");
    let operator: char = operator.trim().parse().expect("not valid");

    let result: i64 = calculate(first_operand, second_operand, operator);

    println!("{}", result);


}
