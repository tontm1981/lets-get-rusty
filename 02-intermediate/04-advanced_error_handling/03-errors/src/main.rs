use std::{collections::HashMap, error::Error, fmt::{Debug, Display, Formatter}, io};

struct ParsePaymentInfoError {
    source: Option<Box<dyn Error>>,
    message: String
}

impl Display for ParsePaymentInfoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Failed to parse payment info")
    }
}

impl Debug for ParsePaymentInfoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}\n\t{}", self.message)?;

        if let Some(e) = self.source.as_ref() {
            write!(f, "\nCaused by\n\t{e:?}")?;
        } 
        Ok(())
    }
}

impl Error for ParsePaymentInfoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}

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
    env_logger::init();

    let credit_cards: HashMap<&str, &str> = HashMap::from([
        ("Amy", "1234567 04 25 123"),
        ("Tim", "1234567 0627 123"),
        ("Bob", "1234567 Dec 27 123"),
    ]);

    println!("Enter a name: ");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");

    let result = get_credit_card_info(&credit_cards, name.trim());
    match result {
        Ok(card) => println!("Card: {:?}", card),
        Err(e) => {
            match &e {
                CreditCardError::InvalidInput(e) => println!("Invalid input: {e}"),
                CreditCardError::Other(_, _) => println!("Something went worng. Try again"),
            }

            log::error!("{e:?}");
        },
    }
}

enum CreditCardError {
    InvalidInput(String),
    Other(Box<dyn Error>, String)
}

impl Debug for CreditCardError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CreditCardError::InvalidInput(msg) => write!(f, "{self}\n{msg}"),
            CreditCardError::Other(e, msg) => write!(f, "{self}\n{msg}\n\nCaused by\n\t{e:?}")
        }
    }
}

impl Display for CreditCardError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Failed to get credit card info")
    }
}

impl Error for CreditCardError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CreditCardError::InvalidInput(_) => None,
            CreditCardError::Other(e, _) => Some(e.as_ref()),
        }
    }
}

fn get_credit_card_info(cards: &HashMap<&str, &str>, name: &str) -> Result<Card, CreditCardError> {
    let card_string = cards.get(name).ok_or(CreditCardError::InvalidInput(format!("No credit card found for {name}")))?;
    let card = parse_card(card_string)
        .map_err(|e| {
            CreditCardError::Other(Box::new(e), format!("{name}'s card could not be parsed"))
        })?;
    Ok(card)
}

fn parse_card(card: &str) -> Result<Card, ParsePaymentInfoError> {
    let mut numbers = parse_card_numbers(card)?;
    let len = numbers.len();
    let expected_len = 4;
    
    if len != expected_len {
        return Err(ParsePaymentInfoError {
            source: None,
            message: format!("Expected {expected_len} numbers, found {len}"),
        })
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

fn parse_card_numbers(card:&str) -> Result<Vec<u32>, ParsePaymentInfoError> {
    let numbers = card
        .split(" ")
        .map(|s| {
            s.parse().map_err(|_| ParsePaymentInfoError {
                source: None,
                message: format!("Could not parse {s} as a number"),
            })
        })
        .collect::<Result<Vec<u32>, _>>()
        .map_err(|e| ParsePaymentInfoError {
            source: Some(Box::new(e)),
            message: format!("Failed to parse input ({card}) as numbers."),
        })?;
    Ok(numbers)
}