fn main() {
    let num_arr: Vec<i32> = vec![4,5,3,56,67,674,332,2,556,68,978,5,34,523];
    let mut max_num: i32 = 0;

    for item in num_arr.iter() {
        if max_num < *item {
            max_num = *item;
        }
    }

    println!("{}", max_num);
}
