// add3.rs
fn main() {
    let mut sum = 0.0;
    for i in 0..5 {
        sum += i as f64;
    }
    println!("sum is {}", sum);
}


// cannot add assign a integer to a float normally unless we cast. The casts are explicit.
