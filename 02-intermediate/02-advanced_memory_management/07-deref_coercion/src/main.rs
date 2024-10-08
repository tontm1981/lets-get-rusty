use std::ops::{ Deref, DerefMut };

struct MySmartPointer<T> {
    value: T,
}

impl<T> MySmartPointer<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> Deref for MySmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for MySmartPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

fn main() {
    let s = MySmartPointer::new(Box::new("Let's get rusty".to_owned()));
    // Even the expected function input being different from the passed argument, the code compiles and works
    // due to the Deref coercion, which allows Rust compiler to coerce a reference of one type to a reference of another type.
    // For convenience, this happens implicitly when references are passed to functions or methods.
    // &Box -> &String -> &str
    print(&s);

    // Deref coercion only works on types that implements the Deref and DerefMut traits
}

fn print(s: &str) {
    println!("{s}",);
}

// ```rust
//     pub trait  Deref {
//         type Target: ?Sized;
//         fn deref(&self) -> &Self::Target;
//     }
// ```
// 
// `Sized` trait refers to types with constant size known at compile time
