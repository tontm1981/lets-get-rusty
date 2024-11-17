/// # Iterators
/// 
///     The iterator design pattern allows different types to have a common interface for iteration, while abstracting away
/// how that iteration is implemented internally. Iterators are heavily used in idiomatic rust code, so, it's important to understand
/// the Iterator Design Pattern and how it's implemented in Rust
/// 
/// ``` rust
/// trait Iterator {
///     type: Item;
///
///     fn next(&mut self) -> Option<Self::Item>;
/// }
/// ```

/// Using ***Item*** as Generics, allows to implements different types.
/// ```rust
/// trait Iterator<Item> {
///     fn next(&mut self) -> Option<Item>;
/// }
/// 
/// impl Iterator<String> for MyStruct {
///     fn next(&mut self) -> Option<String> {
///         None
///     }
/// }
///     
/// impl Iterator<i32> for MyStruct {
///     fn next(&mut self) -> Option<i32> {
///         None
///     }
/// }
/// ```

/// In order to avoid multiple implementations for different types, we use Associated Types
// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }
// struct MyStruct {}

// impl Iterator for MyStruct {
//     type Item = String;
    
//     fn next(&mut self) -> Option<Self::Item> {
//         None
//     }
// }

// fn main() {
//     let mut m = MyStruct{};
//     let item: Option<String> = m.next();
//     println!("{item:?}");
// }

// struct Person {
//     first_name: String,
//     last_name: String,
//     occupation: String,
// }

// struct PersonIterator {
//     values: Vec<String>,
// }

// impl Iterator for PersonIterator {
//     type Item = String;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.values.is_empty() {
//             return None;
//         }
//         Some(self.values.remove(0))
//     }
// }

// impl IntoIterator for Person {
//     type Item = String;

//     type IntoIter = PersonIterator;

//     fn into_iter(self) -> Self::IntoIter {
//         PersonIterator {
//             values: vec![
//                 self.first_name,
//                 self.last_name,
//                 self.occupation
//             ]
//         }
//     }
// }

// fn main() {
//     let p = Person {
//         first_name: "John".to_owned(),
//         last_name: "Wick".to_string(),
//         occupation: "Philanthropist".to_owned()
//     };
//     // let mut i = p.into_iter();
//     // println!("{:?}", i.next()); // Some("John")
//     // println!("{:?}", i.next()); // Some("Wick")
//     // println!("{:?}", i.next()); // Some("Philanthropist")
//     // println!("{:?}", i.next()); // None
//     for item in p {
//         println!("{item}");
//     }
// }

use std::collections::HashMap;

// Iterating over collections
fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("red_team".to_owned(), 8);
    scores.insert("blue_team".to_string(), 2);
    scores.insert("yellow_team".to_owned(), 6);

    for (team, score) in &scores {
        println!("{team} got {score} points");
    }
}