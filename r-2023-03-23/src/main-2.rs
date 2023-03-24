use std::io;

fn find_pos(v: i32) -> Result<usize, String> {
    let arr = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];
    let mut found: Option<usize> = None;
    for i in 0..arr.len() {
        if arr[i] == v {
            found = Some(i);
            break;
        }
    }
    match found {
        Some(v) => Ok(v),
        None => Err("Not found".to_string())
    }
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).expect("No stdin");
    let res: i32 = buffer.trim().parse().unwrap();
    match find_pos(res) {
        Ok(v) => println!("Pos: {}", v),
        Err(_) => panic!("Ohno!")
    }
}
