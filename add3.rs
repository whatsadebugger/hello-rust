// add3.rs
fn main() {
    // with rust type inference this is not a float
    let mut sum = 0.0;
    for i in 0..5 {
        // without this cast we get an error
        sum += i as f64;
    }
    println!("sum is {}", sum);
}


// cannot add assign a integer to a float normally unless we cast. The casts are explicit.
