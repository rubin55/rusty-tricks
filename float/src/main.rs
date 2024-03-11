fn main() {
    let result: f64 = 0.1 + 0.3;
    let desired: f64 = 0.4;
    let absolute_difference = (desired - result).abs();
    assert!(absolute_difference <= f64::EPSILON);
}
