#[derive(Debug)]
struct BrowserCommand<T> {
    #[allow(dead_code)]
    name: String,
    payload: T,
}

impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> BrowserCommand<T> {
        BrowserCommand { name, payload }
    }

    fn get_payload(&self) -> &T {
        &self.payload
    }
}

impl BrowserCommand<String> {
    fn print_payload(&self) {
        println!("{}", self.payload);
    }
}

fn main() {
    let cmd1 = BrowserCommand::new("navigate".to_owned(), String::from("https://www.google.com"));
    cmd1.print_payload();

    let cmd2 = BrowserCommand::new("zoom".to_owned(), 200);
    let p1 = cmd1.get_payload();
    let p2 = cmd2.get_payload();
    serialize_payload(p1);
    serialize_payload(p2);
}

#[allow(unused_variables)]
fn serialize_payload<T>(payload: T) -> String {
    "placeholder".to_owned()
}