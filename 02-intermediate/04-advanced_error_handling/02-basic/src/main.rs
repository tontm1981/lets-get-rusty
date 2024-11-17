use core::num;
use std::{collections::HashMap, io};

#[derive(Debug)]
struct Card {
    #[allow(dead_code)]
    number: String,
    #[allow(dead_code)]
    expiry: Expiration,
    #[allow(dead_code)]
    cvv: String,
}

#[derive(Debug)]
struct Expiration {
    #[allow(dead_code)]
    year: u32,
    #[allow(dead_code)]
    month: u32,
}

fn main() {
    let credit_cards: HashMap<&str, &str> = HashMap::from([
        ("Amy", "1234567 04 25 123"),
        ("Tim", "1234567 06 27 123"),
        ("Bob", "1234567 12 27 123"),
    ]);

    println!("Enter a name: ");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");

    let result = get_credit_card_info(&credit_cards, name.trim());
    match result {
        Ok(card) => println!("Card: {:?}", card),
        Err(e) => println!("Error: {e}"),
    }
}

fn get_credit_card_info(cards: &HashMap<&str, &str>, name: &str) -> Result<Card, String> {
    let card_string = cards.get(name).ok_or(format!("No credit card found for {name}"));
    let card = parse_card(card_string?)?;
    Ok(card)
}

fn parse_card(card: &str) -> Result<Card, String> {
    let mut numbers = parse_card_numbers(card).map_err(|e| e.to_string())?;
    let len = numbers.len();
    let expected_len = 4;
    
    if len != expected_len {
        return Err(format!("Expected {expected_len} numbers, found {len}"));
    }

    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap();

    Ok(Card {
        number: number.to_string(),
        expiry: Expiration { year, month },
        cvv: cvv.to_string(),
    })
}

fn parse_card_numbers(card:&str) -> Result<Vec<u32>, num::ParseIntError> {
    let numbers = card
        .split(" ")
        .map(|s| s.parse())
        .collect::<Result<Vec<u32>, _>>()
        ?;
    Ok(numbers)
}