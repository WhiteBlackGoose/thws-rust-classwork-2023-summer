pub fn validate(interpretation: &str, bmi: f32) -> bool {
    let s: &str = &crate::bmi::bmi::interpret(bmi);
    s.eq(interpretation)
}
