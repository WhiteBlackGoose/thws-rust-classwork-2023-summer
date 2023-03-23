fn main() {
    println!("Hello, world!");
    sqroot(5.0).expect("Bad input!");
    sqroot(-5.0).expect("Bad input!");
    match sqroot2(5.0) {
        Ok(v) => println!("Value: {}", v),
        Err(e) => println!("E {}", e)
    }
    match sqroot2(-5.0) {
        Ok(v) => println!("Value: {}", v),
        Err(e) => println!("E {}", e)
    }
}

fn sqroot(v: f32) -> Option<f32>{
    if v >= 0.0 {
        Some(v.powf(0.5))
    } else {
        None
    }
}

fn sqroot2(v: f32) -> Result<f32, String>{
    if v >= 0.0 {
        Ok(v.powf(0.5))
    } else {
        Err("Negative not allowed".to_string())
    }
}
