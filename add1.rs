// add1.rs
fn main() {
    // sum is immutable because we are missing the mut keyword
    let sum = 0;
    for i in 0..5 {
        sum += i;
    }
    println!("sum is {}", sum);
}
