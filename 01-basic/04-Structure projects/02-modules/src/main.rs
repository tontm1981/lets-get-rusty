use auth_service::{Credentials, authenticate};

fn main() {
    let credentials = Credentials {
        username: "teste".to_string(), 
        password: "123456".to_string() 
    };
    authenticate(credentials);
}