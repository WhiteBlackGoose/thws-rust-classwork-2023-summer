fn parse(inp: &str) -> Option<i32> {
    match inp {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        "zero" => Some(0),
        _ => None
    }
}

fn get_num() -> Option<i32> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("");
    parse(buf.as_str().trim())
}

fn main() {
    let a1 = get_num().expect("Bad input!");
    let a2 = get_num().expect("Bad input!");
    let a3 = get_num().expect("Bad input!");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("");
    match buf.as_str().trim() {
        "+" => println!("{}", a1 + a2 + a3),
        "*" => println!("{}", a1 * a2 * a3),
        _ => panic!("Ohno!")
    };
}
