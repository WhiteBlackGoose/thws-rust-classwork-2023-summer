type CMYK = (u8, u8, u8, u8);

fn highest_cyan(cmyks: &Vec<CMYK>) -> Option<u8> {
    let mut max: Option<u8> = None;
    for (cyan, _, _, _) in cmyks {
        match max {
            None => max = Some(cyan.clone()),
            Some(old_max) => {
                if cyan.clone() > old_max {
                    max = Some(cyan.clone());
                }
            }
        }
    }
    max
}

fn main() {
    match highest_cyan(&vec![(123, 21, 23, 14), (32, 31, 25, 124)]) {
        Some(highest) => println!("Highest cyan: {}", highest),
        None => println!("Error")
    }
}
