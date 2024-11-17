/// # Function Pointers
/// 
/// Function pointers are simmilar to closures, except to the fact that
/// they don't capture variables in theis environment
/// 
/// All functions are coerced to a function pointer.

fn main() {
    let greater_than = |x: &i32| *x > 10;
    let result = are_both_true(greater_than, less_than, &15);
    println!("{}", result);
}

fn less_than(x: &i32) -> bool {
    *x < 20
}

/// Accepting closures and function pointers
// fn are_both_true<T, U, V>(f1: T, f2: U, item: &V) -> bool

/// Now, accepting function pointers only
/// Function Pointers is a concrete type represented by `fn(...) -> ...`
/// Not to be confused with capital `Fn(...) -> ...`, which is closure trait
fn are_both_true<V>(f1: fn(&V) -> bool, f2: fn(&V) -> bool, item: &V) -> bool {
    f1(item) && f2(item)
}