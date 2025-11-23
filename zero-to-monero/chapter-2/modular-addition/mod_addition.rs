fn mod_addition(a: i32, b: i32, n: i32) -> i32 {
    // This logic ONLY works when 0 <= a < n and 0 <= b < n
    assert!(n > 0, "modulus must be positive");

    // Compute x = n - a
    let x = n - a;

    // If x > b then c = a + b, otherwise c = b - x
    let c = if x > b {
        a + b
    } else {
        b - x
    };

    c  // Return the result
}

//Test the mod_addition function using values from zero-to-monero-v2.0.0
fn main(){
    let result = mod_addition(29,87,99);
    println!("Result is {}",result);
}
