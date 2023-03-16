use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error!");

    // Task 1
    if input.contains('s') {
        println!("It contains!");
    }

    // Task 2
    let subs = &input[1..input.len()-2];
    println!("{subs}");

    // Task 3
    let as_int : i32 = input.trim().parse().unwrap();
    let new_int = as_int * 2;
    println!("{new_int}");
}
