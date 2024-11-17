struct Credentials<T> where T: Fn(&str, &str) -> bool 
{
    username: String,
    password: String,
    // We can store closures in structs. To do this, we need to use trait bounds and generics
    validator: T
}

impl<T> Credentials<T> where T: Fn(&str, &str) -> bool {
    fn is_valid(&self) -> bool {
        // in order to use a closure in struct, we must to encapsulate the property name with parenthesis
        (self.validator)(&self.username, &self.password)
    }
}

fn main() {
    // Closures are anonumous functions which we can store in variables or pass as arguments to other functions.
    // There are 3 traits for closures
    //      - Fn      : Immutably borrow vars in environment 
    //      - FnMut   : Mutably borrow vars in environment. Can change environment
    //      - FnOnce  : Take ownership of vars in environment. Can only be called once
    // The compiler can infer the types of the closure parameters and return types based on the context.
    // The closure is defined using the | | operator.
    #[allow(unused_variables)]
    let validator = |username: &str, password:&str| -> bool {
        !username.is_empty() && !password.is_empty()
    };

    let weak_password = "password123!".to_owned();
    // Closures, also, can capture variables in which scope they are defined.
    // In this case, the closure captures the weak_password variable.
    // let validator2 = |username: &str, password:&str| {

    // We can, also force the closure to take the ownership, by using the keyword "move", before closure declaration
    // let validator2 = move |username: &str, password:&str| { ... }

    let validator2 = |username: &str, password:&str| {
        !username.is_empty() && 
        !password.is_empty() &&
        password.len() >= 8 &&
        password.contains(['!', '@', '#', '$', '%','^', '&','*']) &&
        password != weak_password
    };
    // variable still in scope
    println!("{weak_password}");

    let creds = Credentials {
        username: String::from(""),
        password: "".to_owned(),
        validator: validator2
    };

    println!("{}", creds.is_valid());

    let validat = get_password_validator(8, true);
    let default_creds = get_default_creds(validat);
    println!("{}", default_creds.is_valid());
}

#[allow(dead_code)]
fn validate_credentials(username: &str, password: &str) -> bool {
    !username.is_empty() && !password.is_empty()
}

// Function that receives a closure as argument
fn get_default_creds<T>(f: T) -> Credentials<T>
where T: Fn(&str, &str) -> bool
{
    Credentials {
        username: String::from("guest"),
        password: String::from("password123!"),
        validator: f
    }
}

fn get_password_validator(min_len: usize, special_chars: bool) -> Box<dyn Fn(&str, &str) -> bool> {
    if special_chars {
        return Box::new(move |_: &str, password: &str| {
            !password.len()>= min_len &&
            password.contains(['!', '@', '#', '$', '%', '^', '&', '*'])
        });
    }
    Box::new(move |_: &str, password: &str| !password.len()>= min_len)
}