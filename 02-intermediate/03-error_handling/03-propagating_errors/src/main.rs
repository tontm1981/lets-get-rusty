use std::{fs::File, io::{self, Read}};

fn main() {
    let content = read_file("example.txt");
    println!("{:?}", content);
    let user = User {
        firstname: "John".to_string(),
        lastname: "Doe".to_string(),
    };
    let initials = get_initials(user);
    println!("{:?}", initials);
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut contents = String::new();
    File::open(filename)?.read_to_string(&mut contents)?;
    Ok(contents)
}

struct User {
    firstname: String,
    lastname: String,
}

fn get_initials(user: User) -> Option<String> {
    let first_initial = user.firstname.chars().next()?;
    let last_initial = user.lastname.chars().next()?;
    Some(format!("{}.{}", first_initial, last_initial))
}