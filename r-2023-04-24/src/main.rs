mod server {
    fn compute_bmi(weight: f32, height: f32) -> f32 {
        let height_m = height / 100.;
        let bmi = weight / height_m.powi(2);
        let rounded = (bmi * 10.0).round() / 10.0;
        rounded
    }

    fn interpret(bmi: f32) -> String {
        match bmi {
            bmi if bmi < 15.0 => String::from("very severely underweight"),
            bmi if bmi < 16.0 => String::from("severely underweight"),
            bmi if bmi < 18.5 => String::from("underweight"),
            bmi if bmi < 25.0 => String::from("normal weight"),
            bmi if bmi < 30.0 => String::from("overweight"),
            bmi if bmi < 35.0 => String::from("moderately obese"),
            bmi if bmi < 40.0 => String::from("severely obese"),
            _ => String::from("very severely obese")
        }
    }

    pub async fn start() -> Result<(), reqwest::Error> {
        loop {
        }
    }
}

mod client {
    pub async fn request() {
        
    }
}

#[tokio::main]
fn main() {
    println!("{}", bmi_test::validate("normal weight", 20.5));
    println!("{}", bmi_test::validate("overweight", 20.5));
}
