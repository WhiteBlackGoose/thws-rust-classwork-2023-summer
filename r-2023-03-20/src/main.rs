use std::f32::consts::PI;

/// Area of a circle given its diameter
fn area(diam: f32) -> f32 {
    diam * diam * PI / 4.0
}

/// Assuming uniform value of the area,
/// computes the value of each unit
/// given the price for the whole area
fn size_per_euro(area: f32, price: f32) -> f32 {
    area / price
}

/// Entry point
fn main() {
    let diam: f32 = 5.0;
    let price: f32 = 10.0;
    let area = area(diam);
    println!("Area {area}");
    let size_per_euro = size_per_euro(area, price);
    println!("Size per euro {size_per_euro}");
}
