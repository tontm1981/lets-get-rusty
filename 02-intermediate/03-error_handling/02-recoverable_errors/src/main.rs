use std::fs::File;

// Some errors in Rust are recoverable, which means that application doesn't need to stop, due to these errors. 
// There are another ways to procceed.
// In this case, where our errors are recoverable, we can use `Result` type.
//
// The `Result` enum has 2 variants: Ok and Err.
//      - Ok holds the value, in success case
//      - Err holds a generic error
fn main() {
    let file = File::open("hello.txt").expect("Failed to open file");
    println!("{:?}", file);
}
