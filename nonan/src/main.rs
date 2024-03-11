fn main() {
    let x: f32 = 1.0 / 0.0 // nan..
    assert!(x.is_finite());
}
