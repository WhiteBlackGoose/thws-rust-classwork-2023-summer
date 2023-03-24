use std::time;
use std::time::SystemTime;

use rand::prelude::*;

fn main() {
    let mut arr = [0; 64000];
    for i in 0..arr.len() {
        arr[i] = rand::random();
        arr[i] %= 123;
    }
    let time_before = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        ;
    arr.sort();
    let time_after = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        ;
    for el in arr {
        println!("{}", el);
    }
    println!("Elapsed time: {}", time_after - time_before);
}
