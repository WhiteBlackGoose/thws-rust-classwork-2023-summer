use std::fmt::Display;

#[derive(Debug)]
struct Student<T> {
    id: T,
    first_name: String,
    last_name: String,
}

impl<T: Display> Student<T> {
    fn pretty_print(&self) {
        println!("{} {} ({})", self.first_name, self.last_name, self.id);
    }

    fn pretty_print_censored(&self, forbidden: char) {
        let id_as_str = format!("{}", self.id);
        if id_as_str.contains(forbidden) {
            return;
        }
        println!("{} {} ({})", self.first_name, self.last_name, id_as_str);
    }
}

fn main() {
    Student { id: 555, first_name: String::from("Aaa"), last_name: String::from("Bbb") }.pretty_print();
    Student { id: "Aaa", first_name: String::from("Aaa"), last_name: String::from("Bbb") }.pretty_print();
    Student { id: "Aaa", first_name: String::from("Aaa"), last_name: String::from("Bbb") }.pretty_print_censored('A');
    Student { id: "Aaa", first_name: String::from("Aaa"), last_name: String::from("Bbb") }.pretty_print_censored('B');
}
