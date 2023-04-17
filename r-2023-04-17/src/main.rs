use std::time::Duration;

#[derive(Debug)]
struct Car {
    make: String,
    model: String,
    year: u16
}

trait Racing {
    fn racing_start(&self) -> Duration;
}

impl Racing for Car {
    fn racing_start(&self) -> Duration {
        Duration::from_secs(5)
    }
}

fn main() {
    let car = Car { make : String::from("Aaa"), model : String::from("Bbb"), year : 2032 };
    println!("{:#?}", car);
}
