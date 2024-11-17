use std::{ fs, num::ParseIntError};

fn main() {
    let i = parse_file("input.txt");
    match i {
        Ok(i) => println!("Parsed value: {i}"),
        Err(e) => {
            match e {
                ParseFileError::File => println!("Error reading file"),
                ParseFileError::Parse(e) => println!("Error parsing file: {e}")
            }
        }
    }
}

// Due to `io::Error` is not  a trait, but a specific struct, we must use another way to return erros, mainly in case of multiple error types
// fn parse_file(filename: &str) -> Result<i32, io::Error> {

// This approach can handle this scenario, but, it's useful only if we don't care about error types
// fn parse_file(filename: &str) -> Result<i32, Box<dyn error::Error>> {

fn parse_file(filename: &str) -> Result<i32, ParseFileError> {
    let s = fs::read_to_string(filename).map_err(|_| ParseFileError::File)?;
    let i = s
        .parse()
        .map_err(|e| ParseFileError::Parse(e))?;
    Ok(i)
}

enum ParseFileError {
    File,
    Parse(ParseIntError)
}