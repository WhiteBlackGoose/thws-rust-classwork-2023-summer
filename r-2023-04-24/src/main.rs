mod bmi_test;
mod bmi;

fn main() {
    println!("{}", crate::bmi_test::bmi_test::validate("normal weight", 20.5));
    println!("{}", crate::bmi_test::bmi_test::validate("overweight", 20.5));
}
